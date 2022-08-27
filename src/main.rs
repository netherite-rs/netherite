extern crate core;


use std::path::Path;
use std::sync::Arc;

use crate::config::ServerProperties;
use crate::server::server::Server;

mod packets;
mod net;
mod encryption;
mod config;
mod dimension;
mod world;
mod server;
mod game_mode;

#[tokio::main]
async fn main() {
    let directory = Path::new("run").to_path_buf();
    let properties = ServerProperties::read(&directory.join("server.toml"));
    if let Ok(properties) = properties {
        let server = Server::new(directory, properties).await;
        Server::start(Arc::new(server)).await;
    } else {
        eprintln!("Error parsing server.toml: {}", properties.err().unwrap().to_string());
    }
}