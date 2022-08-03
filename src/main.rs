use crate::server::Server;

mod server;
mod packets;

#[tokio::main]
async fn main() {
    let server = Server::new(2000).await;
    server.start().await;
}