use bytebuffer::ByteBuffer;

use protocol::bound::Serverbound;

use crate::net::codec::ClientCodec;
use crate::net::packet_handler::serverbound;
use crate::packets::handshake;
use crate::packets::login;
use crate::packets::play;
use crate::packets::status;

/// generated code
pub struct PacketHandler;

impl PacketHandler {
    pub async fn handle_handshake_packet(codec: &mut ClientCodec, id: i32, data: &mut ByteBuffer) {
        match id {
            0 => {
                let packet = handshake::Handshake::read_packet(data);
                serverbound::handle_handshake(&packet, codec).await;
            }
            n => panic!("invalid handshake packet id: {}", n)
        }
    }

    pub async fn handle_login_packet(codec: &mut ClientCodec, id: i32, data: &mut ByteBuffer) {
        match id {
            1 => {
                let packet = login::EncryptionResponse::read_packet(data);
                serverbound::handle_encryption_response(&packet, codec).await;
            }
            0 => {
                let packet = login::LoginStart::read_packet(data);
                serverbound::handle_login_start(&packet, codec).await;
            }
            n => panic!("invalid login packet id: {}", n)
        }
    }

    pub async fn handle_play_packet(codec: &mut ClientCodec, id: i32, data: &mut ByteBuffer) {
        match id {
            n => panic!("invalid play packet id: {}", n)
        }
    }

    pub async fn handle_status_packet(codec: &mut ClientCodec, id: i32, data: &mut ByteBuffer) {
        match id {
            0 => {
                let packet = status::StatusRequest::read_packet(data);
                serverbound::handle_status_request(&packet, codec).await;
            }
            1 => {
                let packet = status::PingRequest::read_packet(data);
                serverbound::handle_ping_request(&packet, codec).await;
            }
            n => panic!("invalid status packet id: {}", n)
        }
    }
}
