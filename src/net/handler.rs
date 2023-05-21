use bytebuffer::ByteBuffer;

use crate::protocol::Serverbound;

use crate::net::codec::ClientCodec;
use crate::net::packet_handler::serverbound;
use crate::packets::handshake;
use crate::packets::login;

use crate::packets::status;
use crate::Server;

pub struct PacketHandler;

impl PacketHandler {
    pub async fn handle_handshake_packet(codec: &mut ClientCodec, id: i32, data: &mut ByteBuffer) {
        match id {
            0x00 => {
                let packet = handshake::Handshake::read_packet(data);
                serverbound::handle_handshake(&packet, codec).await;
            }
            n => panic!("invalid handshake packet id: {}", n)
        }
    }

    pub async fn handle_login_packet(codec: &mut ClientCodec, id: i32, data: &mut ByteBuffer, server: &Server) {
        match id {
            0x00 => {
                let packet = login::LoginStart::read_packet(data);
                serverbound::handle_login_start(packet, codec, server).await;
            }
            0x01 => {
                let packet = login::EncryptionResponse::read_packet(data);
                serverbound::handle_encryption_response(packet, codec, server).await;
            }
            0x02 => {
                let packet = login::LoginPluginResponse::read_packet(data);
                println!("{:?}", packet.message_id);
            }
            n => panic!("invalid login packet id: {}", n)
        }
    }

    pub async fn handle_status_packet(codec: &mut ClientCodec, id: i32, data: &mut ByteBuffer, server: &Server) {
        match id {
            0x00 => {
                let packet = status::StatusRequest::read_packet(data);
                serverbound::handle_status_request(&packet, codec, server).await;
            }
            0x01 => {
                let packet = status::PingPacket::read_packet(data);
                serverbound::handle_ping_request(&packet, codec).await;
            }
            n => panic!("invalid status packet id: {}", n)
        }
    }

    pub async fn handle_play_packet(codec: &mut ClientCodec, id: i32, data: &mut ByteBuffer, server: &Server) {
        match id {
            n => { /*panic!("invalid play packet id: {}", n)*/ }
        }
    }
}
