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
mod dimension;
mod world;
mod util;

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

// fn main() {
//     let mut file1 = File::open(Path::new("codecs/registry_codec.nbt")).unwrap();
//     let blob: nbt::Blob = Blob::from_reader(&mut file1).unwrap();
//     let mut file = File::create(Path::new("codecs/registry-codec.json")).unwrap();
//     serde_json::to_writer(&mut file, &blob).unwrap();
// }