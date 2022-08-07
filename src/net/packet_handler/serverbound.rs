use tokio::io::AsyncWriteExt;
use chat::style::RgbColor;
use chat::text_component::TextComponent;
use protocol::fields::generic::Json;

use crate::net::codec::{ClientCodec, ProtocolStage};
use crate::packets::handshake;
use crate::packets::handshake::Handshake;
use crate::packets::login::{EncryptionResponse, LoginStart};
use crate::packets::status::{PingRequest, PingResponse, Response, RespPlayers, RespPlayerSample, RespVersion, StatusRequest, StatusResponse};

pub(crate) async fn handle_handshake(packet: &Handshake, codec: &mut ClientCodec) {
    let next_state = packet.next_state.0;
    match next_state {
        handshake::STATUS => codec.set_stage(ProtocolStage::Status),
        handshake::LOGIN => codec.set_stage(ProtocolStage::Login),
        _ => panic!("invalid value for next_state in HandshakePacket. Expected 1 or 2, found {}", next_state)
    }
}

pub(crate) async fn handle_encryption_response(packet: &EncryptionResponse, codec: &mut ClientCodec) {
    println!("EncryptionResponse: {:?}", packet);
}

pub(crate) async fn handle_login_start(packet: &LoginStart, codec: &mut ClientCodec) {
    println!("LoginPacket: {:?}", packet);
}

pub(crate) async fn handle_ping_request(packet: &PingRequest, codec: &mut ClientCodec) {
    codec.write_packet(&PingResponse {
        payload: packet.payload
    }).await.unwrap();
}

pub(crate) async fn handle_status_request(_: &StatusRequest, codec: &mut ClientCodec) {
    let response = Response {
        version: RespVersion {
            name: "1.19".to_string(),
            protocol: 759,
        },
        players: RespPlayers {
            max: 100,
            online: 5,
            sample: vec![
                RespPlayerSample {
                    name: "thinkofdeath".to_string(),
                    id: "4566e69f-c907-48ee-8d71-d7ba5aa00d20".to_string(),
                }
            ],
        },
        description: TextComponent::builder()
            .text("Welcome to netherite-rs!".to_string())
            .color(&RgbColor::new(230, 20, 40))
            .push_child(TextComponent::builder()
                .text("Hello!".to_string())
                .obfuscated()
                .build())
            .build(),
        favicon: None,
        previews_chat: true,
    };
    codec.write_packet(&StatusResponse {
        response: Json(response)
    }).await.unwrap();
}
