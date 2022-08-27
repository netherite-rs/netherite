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
    let properties = ServerProperties::read(&Path::new("../run/server.toml"));
    if let Ok(properties) = properties {
        let server = Server::new(properties).await;
        Server::start(Arc::new(server)).await;
    } else {
        eprintln!("Error parsing server.toml: {}", properties.err().unwrap().to_string());
    }
}

// fn main() {
//     let mut file1 = File::open(Path::new("codecs/registry_codec.nbt")).unwrap();
//     let blob: nbt::Blob = Blob::from_reader(&mut file1).unwrap();
//     let mut file = File::create(Path::new("codecs/registry-codec.json")).unwrap();
//     serde_json::to_writer(&mut file, &blob).unwrap();
// }