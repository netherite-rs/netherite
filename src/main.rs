use protocol::identifier::Identifier;
use crate::server::Server;
use protocol_derive::Clientbound;
use protocol::bound::Clientbound;

mod server;

#[derive(Clientbound)]
#[packet(id = 0x01)]
struct Packet {}

#[tokio::main]
async fn main() {
    // let server = Server::new(2000).await;
    // server.start().await;
    let id = Identifier::minecraft("hello".to_string());
}
