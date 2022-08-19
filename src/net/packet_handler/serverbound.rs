use std::thread;
use std::time::Duration;
use bytebuffer::ByteBuffer;
use nbt::Blob;
use num_bigint::BigInt;
use rsa::Hash::SHA2_256;
use rsa::PaddingScheme;
use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
use rsa::RsaPublicKey;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use chat::style::RgbColor;
use chat::text_component::TextComponent;
use protocol::bound::{Clientbound, Serverbound};
use protocol::fields::generic::Json;
use protocol::fields::identifier::Key;
use protocol::fields::numeric::VarInt;
use protocol::fields::position::Position;
use protocol::fields::profile::{GameProfile, Property};

use crate::net::codec::{ClientCodec, ProtocolStage};
use crate::packets::handshake;
use crate::packets::handshake::Handshake;
use crate::packets::login::{EncryptionRequest, EncryptionResponse, LoginPlay, LoginStart, LoginSuccess};
use crate::packets::status::{PingRequest, PingResponse, Response, RespPlayers, RespPlayerSample, RespVersion, StatusRequest, StatusResponse};
use crate::Server;

const HAS_JOINED: &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined";

pub(crate) async fn handle_handshake(packet: &Handshake, codec: &mut ClientCodec) {
    let next_state = packet.next_state.0;
    match next_state {
        handshake::STATUS => codec.set_stage(ProtocolStage::Status),
        handshake::LOGIN => codec.set_stage(ProtocolStage::Login),
        _ => panic!("invalid value for next_state in HandshakePacket. Expected 1 or 2, found {}", next_state)
    }
}

/*
EncryptionResponse {
	shared_secret: [...],
	has_verify_token: false,
	verify_token: None,
	salt: Some(1723352296252182653),
	message_signature: Some(-9222655986088836972)
}
*/

pub(crate) async fn handle_encryption_response(
    packet: EncryptionResponse,
    codec: &mut ClientCodec,
    server: &Server,
) {
    let shared_secret = server.encryption().decrypt(&packet.shared_secret).unwrap();
    if !packet.has_verify_token {
        if !verify_salt(
            packet.salt.unwrap(),
            packet.message_signature.as_ref().unwrap(),
            codec,
            server,
        ) {
            panic!("salt does not match.");
        }
    } else {
        let verify_token = packet.verify_token.unwrap();
        let verify_token = server.encryption().decrypt(&verify_token).unwrap();
        if !server.encryption().compare_verify_tokens(verify_token) {
            panic!("verify tokens do not match.");
        }
    }

    codec.enable_encryption(shared_secret.try_into().unwrap());
    let hex = generate_server_hash(&server, &codec.encryption().as_ref().unwrap().secret());
    let profile: GameProfile = authenticate(
        codec.player_name().as_ref().unwrap(),
        &hex,
        server.properties().server().address(),
    ).await;
    println!("profile: {:?}", profile);
    codec.enable_compression(server.properties().server().compression_threshold()).await;
    let success = LoginSuccess { profile };
    codec.write_packet(&success).await.unwrap();
    println!("sent profile packet");
    codec.set_profile(Some(success.profile));

    join_player(codec).await;
}

fn verify_salt(salt: i64, message_signature: &Vec<u8>, codec: &mut ClientCodec, server: &Server) -> bool {
    let public_key = codec.public_key().as_ref().unwrap();
    let document = public_key.to_public_key_der().unwrap();
    let key = document.as_ref();
    let mut hash = Sha256::new();
    hash.update(key);
    hash.update(server.encryption().verify_token());
    hash.update(salt.to_be_bytes());
    let hash = hash.finalize().to_vec();
    let padding = PaddingScheme::new_pkcs1v15_sign(Some(SHA2_256));
    // public_key.verify(padding,
    //                   hash.as_slice(),
    //                   message_signature,
    // ).expect("failed to verify signature");
    // return publicKeyInfo.createSignatureValidator().validate((SignatureUpdater)((updater) -> {
    //     updater.update(nonce);
    //     updater.update(signature.saltAsBytes());
    // }), signature.signature());
    true
}

async fn authenticate(player: &str, hex: &str, ip: &str) -> GameProfile {
    let response = reqwest::get(format!(
        "{}?username={}&serverId={}&ip={}",
        HAS_JOINED,
        player,
        hex,
        ip,
    )).await;
    if response.is_err() {
        panic!("couldn't authenticate player {} using Mojang.", player);
    }
    response.unwrap().json::<GameProfile>().await.unwrap()
}

fn generate_server_hash(server: &Server, key: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(b"");
    hasher.update(key);
    hasher.update(&server.encryption().public_key_encoded());
    let hex = hasher.finalize();
    let bigint = BigInt::from_signed_bytes_be(hex.as_slice());
    format!("{:x}", bigint)
}

async fn join_player(codec: &mut ClientCodec) {
    codec.set_stage(ProtocolStage::Play);
    let mut registry_codec = ByteBuffer::from_bytes(include_bytes!("../../../codecs/registry_codec.nbt"));
    let registry_codec = Blob::from_reader(&mut registry_codec).unwrap();
    let play = LoginPlay {
        entity_id: 0,
        is_hardcore: false,
        game_mode: 1,
        previous_gamemode: -1,
        dimension_names: vec![
            Key::new("dimension", "world"),
            Key::new("dimension", "world_nether"),
            Key::new("dimension", "world_the_end"),
        ],
        dimesion_codec: registry_codec,
        dimension_type: Key::minecraft("overworld"),
        dimension_name: Key::new("dimension", "world"),
        hashed_seed: -20,
        max_players: VarInt(5),
        view_distance: VarInt(12),
        simulation_distance: VarInt(12),
        reduced_debug_info: false,
        enable_respawn_screen: false,
        is_debug: false,
        is_flat: false,
        has_death_location: true,
        death_dimension_name: Some(Key::new("dimension", "world")),
        death_location: Some(Position {
            x: 10,
            y: 10,
            z: 10,
        }),
    };
    codec.write_packet(&play).await.unwrap();
}

pub(crate) async fn handle_login_start(packet: LoginStart, codec: &mut ClientCodec, server: &Server) {
    let name = packet.name;
    let public_key = packet.public_key.unwrap();
    let public_key = RsaPublicKey::from_public_key_der(public_key.as_slice()).unwrap();
    codec.set_public_key(Some(public_key));
    if server.properties().server().online_mode() {
        codec.write_packet(&EncryptionRequest {
            server_id: "".to_string(),
            public_key: server.encryption().public_key_encoded(),
            verify_token: server.encryption().verify_token(),
        }).await.unwrap();
    } else {
        codec.enable_compression(server.properties().server().compression_threshold()).await;
        let success = LoginSuccess {
            profile: GameProfile {
                id: offline_mode_uuid(&name),
                name: name.clone(),
                properties: vec![
                    Property {
                        name: String::from("texterues"),
                        value: "ewogICJ0aW1lc3RhbXAiIDogMTY2MDY2Nzg5Mzc0OCwKICAicHJvZmlsZUlkIiA6ICI4ZDkxZmViYTU4N2Q0NmNjOGFmZTUxNzkzNmMxMWYyMCIsCiAgInByb2ZpbGVOYW1lIiA6ICJSZXZ4cnNhbCIsCiAgInNpZ25hdHVyZVJlcXVpcmVkIiA6IHRydWUsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS83N2FlYTFhN2U5MjNkNzg0YTZiMzFiMjE3MzMwOTcwYjAwMTRhNDFlMjFmYjMxYmE0YmFjMmNhNmQ5NGI0NWYiCiAgICB9CiAgfQp9".to_string(),
                        signature: Some("tlxE3J8jndIwHLaJwPjABH/XAbrYCsExlXB7dje4UbyAX3lTKqD9d+ElQdaoHnIU6ctRcOXmrQVCOeEh+9VWDXRljSNgAdcjCcRqZZXlblZdi+YEykCRaGjHCpjw4c/8Pl9kOjLHcs/jmbftOj3nn5flK0yNceAoyYTIiRxxN6ky8hGEM4v+LfnuSp7PSwvOpbjdSuP2/T2yjKL3Scbji7Y7UtLmhYmW3HBUbPo5fI2TMWeuI8rU40N+6hmkijDSrL2qiUqAOUi+NPbg8j10dOSWYO7BNwu/Lyp92JPVVkDrThuaAoZPh6bIsWRPy5rJR+EBkx4KtmLcdrxe7yhven1FjTB1Q8ViAx9Y+XSkvk5bKhVqXFC1Vix/tdX2fXPOfNEfNZr5QIW04DStfXpU9LZ8nG+d4Ul3U6VyAAO5XcGpvWe3QQQ/36JLfVEPTiyaR9rY24dTtMPpD8jZfyiz1rKnIY7bijlqpMifeKj8f+5/5ZUnVlU4z6ia37187NNOQvU5kG33xqg5olIncSmFWWbq/uIrE0Gw1di27lnVXgPdYAGJpTqqKrmE7SQGV2bnc6fQtdNAlz07mcm6asWG7NPjtdigmfOWXCma+YtqewTLkIU0iThznbecVShiZHk93SXfm2Ypn+IfDdiyZif5Txd+BxVtwvnh6IaqUnPMVas=".to_string()),
                    }
                ],
            }
        };
        codec.write_packet(&success).await.unwrap();
        codec.set_profile(Some(success.profile));
        join_player(codec).await;
    }
    codec.set_player_name(Some(name));
}

pub(crate) async fn handle_ping_request(packet: &PingRequest, codec: &mut ClientCodec) {
    codec.write_packet(&PingResponse {
        payload: packet.payload
    }).await.unwrap();
}

pub(crate) async fn handle_status_request(_: &StatusRequest, codec: &mut ClientCodec, server: &Server) {
    let response = Response {
        version: RespVersion {
            name: "1.19.1".to_string(),
            protocol: 760,
        },
        players: RespPlayers {
            max: server.properties().status().max_players(),
            online: 5,
            sample: vec![
                RespPlayerSample {
                    name: "thinkofdeath".to_string(),
                    id: "4566e69f-c907-48ee-8d71-d7ba5aa00d20".to_string(),
                }
            ],
        },
        description: TextComponent::builder()
            .text(server.properties().status().motd().to_string())
            .color(&RgbColor::new(230, 47, 70))
            .build(),
        favicon: Some(server.properties().status().read_icon()),
        previews_chat: true,
    };
    codec.write_packet(&StatusResponse {
        response: Json(response)
    }).await.unwrap();
}

fn offline_mode_uuid(username: &str) -> Uuid {
    let digest = md5::compute(format!("OfflinePlayer:{}", username).as_bytes());
    let mut builder = uuid::Builder::from_bytes(digest.try_into().unwrap());
    builder
        .set_variant(uuid::Variant::RFC4122)
        .set_version(uuid::Version::Md5);
    builder.into_uuid()
}
