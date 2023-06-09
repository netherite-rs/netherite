use std::sync::Arc;
use byteorder::ReadBytesExt;
use protocol::fields::io_ext::PacketReaderExt;

use protocol::fields::numeric::VarInt;
use protocol::Serverbound;

use crate::client::client::{Client, ProtocolStage};
use crate::server::server::Server;

pub const STATUS: i32 = 1;
pub const LOGIN: i32 = 2;

// #[derive(Serverbound)]
// #[packet(id = 0x00)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl protocol::Serverbound for Handshake {
    fn read_packet(input: &mut bytebuffer::ByteBuffer) -> Handshake {
        Handshake {
            protocol_version: input.read_varint().expect("failed to read protocol_version"),
            server_address: input.read_utf8().expect("failed to read server_address"),
            server_port: input.read_u16().expect("failed to read server_port"),
            next_state: input.read_varint().expect("failed to read next_state"),
        }
    }
    fn id() -> i32 { 0i32 }
}