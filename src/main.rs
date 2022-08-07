extern crate core;

use crate::server::Server;

mod server;
mod packets;
mod net;

#[tokio::main]
async fn main() {
    let server = Server::new(2000).await;
    server.start().await;
}