use protocol::fields::numeric::VarInt;
use protocol::Serverbound;

pub const STATUS: i32 = 1;
pub const LOGIN: i32 = 2;

#[derive(Serverbound)]
#[packet(id = 0x00)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}