use std::borrow::Borrow;
use std::ops::Deref;

use rustc_serialize::hex::ToHex;
use sha1::{Digest, Sha1};
use uuid::Uuid;

use chat::style::RgbColor;
use chat::text_component::TextComponent;
use protocol::fields::generic::Json;
use protocol::fields::profile::GameProfile;

use crate::net::codec::{ClientCodec, ProtocolStage};
use crate::packets::handshake;
use crate::packets::handshake::Handshake;
use crate::packets::login::{EncryptionRequest, EncryptionResponse, LoginStart, LoginSuccess};
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

pub(crate) async fn handle_encryption_response(
    packet: EncryptionResponse,
    codec: &mut ClientCodec,
    server: &Server,
) {
    if !(packet.has_verify_token) {
        return;
    }
    let verify_token = packet.verify_token.unwrap();
    let verify_token = server.encryption().decrypt(&verify_token).unwrap();
    if !server.encryption().compare_verify_tokens(verify_token) {
        return;
    }
    let shared_secret = server.encryption().decrypt(&packet.shared_secret).unwrap();
    let mut key = [0; 16];
    for i in 0..16 { key[i] = shared_secret[i]; }
    codec.set_encryption(key);

    let mut hasher = Sha1::new();
    hasher.update(b"");
    hasher.update(shared_secret.as_slice());
    hasher.update(server.encryption().public_key_encoded());
    let hex = hasher.finalize();
    let hex = hex.to_hex();
    let response = reqwest::get(format!(
        "{}?username={}&server={}&ip={}",
        HAS_JOINED,
        codec.player_name().as_ref().unwrap(),
        hex,
        server.properties().server().address())
    ).await;
    if response.is_err() {
        eprintln!("couldn't authenticate player {} using Mojang.", codec.player_name().as_ref().unwrap());
        return;
    }
    let profile = response.unwrap().json::<GameProfile>().await.unwrap();
    let packet = LoginSuccess { profile };
    codec.write_packet(&packet).await.unwrap();
    codec.set_profile(Some(packet.profile));
    server.players().send()
}

pub(crate) async fn handle_login_start(packet: LoginStart, codec: &mut ClientCodec, server: &Server) {
    let name = packet.name;
    if server.properties().server().online_mode() {
        codec.write_packet(&EncryptionRequest {
            server_id: "".to_string(),
            public_key: server.encryption().public_key_encoded(),
            verify_token: server.encryption().verify_token(),
        }).await.unwrap();
    } else {
        let success = LoginSuccess {
            profile: GameProfile {
                uuid: offline_mode_uuid(&name),
                username: name.clone(),
                properties: vec![],
            }
        };
        codec.write_packet(&success).await.unwrap();
        codec.set_profile(Some(success.profile))
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
            name: "1.19".to_string(),
            protocol: 759,
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
            .color(&RgbColor::new(230, 20, 40))
            .build(),
        favicon: Some(
            server.properties().status().read_icon()
        ),
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
