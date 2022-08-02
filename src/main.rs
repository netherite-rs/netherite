use std::io::{Read, Write};
use bytebuffer::ByteBuffer;
use byteorder::{BigEndian, WriteBytesExt};
use nbt::Blob;
use protocol::fields::{PacketField, VarLong};
use protocol_derive::{Clientbound, Serverbound};
use protocol::bound::Clientbound;
use protocol::packet_io::PacketWriterExt;
use crate::server::Server;

mod server;

#[derive(Serverbound)]
#[packet(id = 0x01)]
struct LoginPacket {
    name: String,
    value: VarLong,
}

#[tokio::main]
async fn main() {
    let packet = LoginPacket { name: String::new(), value: VarLong(30) };
    println!("{}", LoginPacket::id())
    // let server = Server::new(2000).await;
    // server.start().await;
}
