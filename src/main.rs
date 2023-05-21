extern crate core;

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use crate::config::ServerProperties;
use crate::server::server::Server;

// Re-export as #[derive(Clientbound, Serverbound)].
#[cfg(feature = "protocol_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate protocol_derive;

use std::io::{Read, Result, Write};

use bytebuffer::ByteBuffer;
#[cfg(feature = "protocol_derive")]
#[doc(hidden)]
pub use protocol_derive::*;
use crate::region::region::Region;

mod config;
mod dimension;
mod encryption;
mod game_mode;
mod net;
mod packets;
mod region;
mod server;
mod world;
mod util;
mod chat;
mod protocol;

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
//
// fn main() {
//     // println!("{}", serde_json::to_string(&range).unwrap());
//     let file = File::open("F:/Rust/netherite-rs/run/world/region/r.1.1.mca").unwrap();
//     let region = Region::new(file, 1, 1).unwrap();
//     let chunk_data = region.get_chunk_data(30, 30).unwrap().unwrap();
// }
