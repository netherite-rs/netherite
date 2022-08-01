use std::io::{Read, Write};
use nbt::Blob;
use protocol::fields::PacketField;
use crate::server::Server;

mod server;

#[tokio::main]
async fn main() {
    let server = Server::new(2000).await;
    server.start().await;
}
