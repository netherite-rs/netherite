extern crate core;

use std::path::Path;
use std::sync::Arc;

use crate::config::ServerProperties;
use crate::server::Server;

mod server;
mod packets;
mod net;
mod encryption;
pub mod config;
mod entity;

#[tokio::main]
async fn main() {
    let properties = match ServerProperties::read(&Path::new("server.toml")) {
        Ok(v) => v,
        Err(why) => {
            eprintln!("Error parsing server.toml: {}", why.to_string());
            return;
        }
    };

    let server = Server::new(properties).await;
    Server::start(Arc::new(server)).await;
}