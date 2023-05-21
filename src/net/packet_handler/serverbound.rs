use nbt::Blob;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use rsa::pkcs8::DecodePublicKey;
use sha2::{Digest, Sha256};

use bytebuffer::ByteBuffer;
use enum_utils::OrdinalEnum;

use crate::chat::style::RgbColor;
use crate::chat::text_component::TextComponent;
use crate::net::codec::{ClientCodec, ProtocolStage};
use crate::packets::handshake;
use crate::packets::handshake::Handshake;
use crate::packets::login::{EncryptionRequest, EncryptionResponse, LoginPlay, LoginStart, LoginSuccess};
use crate::packets::play::SetDefaultSpawnPosition;
use crate::packets::status::{
    PingPacket, Response, RespPlayers, RespPlayerSample, RespVersion, StatusRequest, StatusResponse,
};
use crate::protocol::fields::generic::Json;
use crate::protocol::fields::key::Key;
use crate::protocol::fields::numeric::{Angle, VarInt};
use crate::protocol::fields::position::Position;
use crate::protocol::fields::profile::GameProfile;
use crate::Server;
use crate::server::authentication;

pub(crate) async fn handle_handshake(packet: &Handshake, codec: &mut ClientCodec) {
    let next_state = packet.next_state.0;
    match next_state {
        handshake::STATUS => codec.set_stage(ProtocolStage::Status),
        handshake::LOGIN => codec.set_stage(ProtocolStage::Login),
        v => panic!(
            "invalid state in handshake packet. Expected 1 (status) or 2 (login), found {}",
            v
        ),
    }
}

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
            panic!("salt does not match!");
        }
    } else {
        let verify_token = packet.verify_token.unwrap();
        let verify_token = server.encryption().decrypt(&verify_token).unwrap();
        if !server.encryption().compare_verify_tokens(&verify_token) {
            panic!("verify tokens do not match.");
        }
    }

    codec.enable_encryption(shared_secret.try_into().unwrap());
    let hex = authentication::generate_server_hash(
        &server.encryption().public_key_encoded(),
        &codec.encryption().as_ref().unwrap().secret(),
    );
    let profile = authentication::authenticate(
        codec.player_name().as_ref().unwrap(),
        &hex,
        server.properties().server().address(),
    ).await;

    codec.enable_compression(*server.properties().server().compression_threshold()).await;

    let success = LoginSuccess { profile };
    codec.write_packet(&success).await.unwrap();
    codec.set_profile(Some(success.profile));

    server.finish_login(codec).await;
}

fn verify_salt(
    salt: i64,
    message_signature: &Vec<u8>,
    codec: &mut ClientCodec,
    server: &Server,
) -> bool {
    let public_key = codec.public_key().as_ref().unwrap();
    let mut hash = Sha256::default();
    hash.update(server.encryption().verify_token());
    hash.update(salt.to_be_bytes());
    let hash = hash.finalize().to_vec();
    let padding = PaddingScheme::new_pkcs1v15_sign::<Sha256>();
    return public_key
        .verify(padding, hash.as_slice(), message_signature)
        .is_ok();
}

pub(crate) async fn join_player(codec: &mut ClientCodec,
                                server: &Server) {
    codec.set_stage(ProtocolStage::Play);
    let mut registry_codec = ByteBuffer::from_bytes(include_bytes!("../../../codecs/registry_codec.nbt"));
    let registry_codec = Blob::from_reader(&mut registry_codec).unwrap();
    let play = LoginPlay {
        entity_id: 0,
        is_hardcore: false,
        game_mode: server.properties().game().default_gamemode().ordinal() as u8,
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
        max_players: VarInt(*server.properties().status().max_players() as i32),
        view_distance: VarInt(*server.properties().game().view_distance() as i32),
        simulation_distance: VarInt(*server.properties().game().simulation_disance() as i32),
        reduced_debug_info: *server.properties().game().reduce_debug_info(),
        enable_respawn_screen: *server.properties().game().enable_respawn_screen(),
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

    // codec.write_packet(&SetDefaultSpawnPosition {
    //     position: Position {
    //         x: 10,
    //         y: 10,
    //         z: 10,
    //     },
    //     angle: 3.0,
    // }).await.unwrap();
}

pub(crate) async fn handle_login_start(
    packet: LoginStart,
    codec: &mut ClientCodec,
    server: &Server,
) {
    let name = packet.name;
    let public_key = packet.public_key.unwrap();
    let public_key = RsaPublicKey::from_public_key_der(public_key.as_slice()).unwrap();
    codec.set_public_key(Some(public_key));
    if *server.properties().server().online_mode() {
        codec
            .write_packet(&EncryptionRequest {
                server_id: "".to_string(),
                public_key: server.encryption().public_key_encoded(),
                verify_token: server.encryption().verify_token(),
            })
            .await
            .unwrap();
    } else {
        codec
            .enable_compression(*server.properties().server().compression_threshold())
            .await;
        let success = LoginSuccess {
            profile: GameProfile::offline(&name),
        };
        codec.write_packet(&success).await.unwrap();
        codec.set_profile(Some(success.profile));
        server.finish_login(codec).await;
    }
    codec.set_player_name(Some(name));
}

pub(crate) async fn handle_ping_request(packet: &PingPacket, codec: &mut ClientCodec) {
    codec.write_packet(packet).await.unwrap();
    codec.close_connction().await;
}

pub(crate) async fn handle_status_request(
    _: &StatusRequest,
    codec: &mut ClientCodec,
    server: &Server,
) {
    let response = Response {
        version: RespVersion {
            name: "1.19.2".to_string(),
            protocol: 760,
        },
        players: RespPlayers {
            max: *server.properties().status().max_players(),
            online: 5,
            sample: vec![RespPlayerSample {
                name: "thinkofdeath".to_string(),
                id: "4566e69f-c907-48ee-8d71-d7ba5aa00d20".to_string(),
            }],
        },
        description: TextComponent::builder()
            .text(server.properties().status().motd().to_string())
            .color(&RgbColor::new(230, 47, 70))
            .build(),
        favicon: Some(
            server
                .properties()
                .status()
                .read_icon(server.run_directory()),
        ),
        previews_chat: true,
    };
    codec
        .write_packet(&StatusResponse {
            response: Json(response),
        })
        .await
        .unwrap();
}
