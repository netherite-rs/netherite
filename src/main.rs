#![allow(unused_imports)]
#![allow(dead_code)]

extern crate core;
// Re-export as #[derive(Clientbound, Serverbound)].
#[cfg(feature = "protocol_derive")]
#[macro_use]
extern crate protocol_derive;

pub use std::io::Read;
use std::path::Path;

use crate::config::ServerProperties;
use crate::server::server::Server;
#[cfg(feature = "protocol_derive")]
#[doc(hidden)]
pub use protocol_derive::*;
use crate::app::setup_netherite_app;

mod config;
mod dimension;
mod encryption;
mod game_mode;
mod client;
mod packets;
mod region;
mod server;
mod server_cfg;
mod util;
mod world;
mod app;

//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let directory = Path::new("run").to_path_buf();
//     let server = GoodServer::from_directory(directory).unwrap();
//     let server = Arc::new(Mutex::new(server));
//     GoodServer::start(server).await;
//     Ok(())
// }
//
// #[tokio::main]
// async fn main() {
//     let directory = Path::new("run").to_path_buf();
//     let properties = ServerProperties::from_file(&directory.join("server.toml"));
//     Server::start(directory, properties).await;
// }

fn main() {
    setup_netherite_app();
}

// fn main() {
//     // println!("{}", serde_json::to_string(&range).unwrap());
//     let mut file = File::open("F:/Rust/netherite-rs/run/world/region/r.1.1.mca").unwrap();
//     let region = Region::new(file, 1, 1).unwrap();
//     let data = region.get_chunk_data(30, 30).unwrap().unwrap();
//     // let data = Blob::from_reader(&mut file).unwrap();
//     let string = serde_json::to_string(&data).unwrap();
//     let mut chunk_json = File::create("F:/Rust/netherite-rs/run/r-1.1.json").unwrap();
//     chunk_json.write(&string.as_bytes()).unwrap();
// }
