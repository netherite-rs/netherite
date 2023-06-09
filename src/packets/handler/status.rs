use std::sync::Arc;

use bytebuffer::ByteBuffer;
use chat::style::RgbColor;
use chat::text_component::TextComponent;
use protocol::fields::generic::Json;

use protocol::Serverbound;

use crate::client::client::Client;
use crate::packets::status::{PingPacket, Response, RespPlayers, RespPlayerSample, RespVersion, StatusRequest, StatusResponse};
use crate::server::server::Server;

pub async fn receive_status(id: i32, data: &mut ByteBuffer, client: &mut Client, server: Arc<Server>) {
    if id == StatusRequest::id() {
        let packet = StatusRequest::read_packet(data);
        handle_status(packet, client, server).await;
    } else if id == PingPacket::id() {
        let packet = PingPacket::read_packet(data);
        client.send_packet(&packet).await.unwrap();
        client.close_connection(server.as_ref()).await;
    }
}

pub async fn handle_status(
    _packet: StatusRequest,
    client: &mut Client,
    server: Arc<Server>,
) {
    let response = Response {
        version: RespVersion {
            name: "1.19.4".to_string(),
            protocol: 762,
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
        favicon: Some(server.properties().status().icon().to_string()),
        previews_chat: true,
    };
    client.send_packet(&StatusResponse {
        response: Json(response),
    }).await.unwrap();
}
