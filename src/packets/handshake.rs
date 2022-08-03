use protocol::fields::numeric::VarInt;
use protocol_derive::Serverbound;

#[derive(Serverbound)]
#[packet(id = 0x00)]
pub struct HandshakePacket {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
}
