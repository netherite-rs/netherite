extern crate core;

use std::fmt::Error;
use std::panic;
use std::path::Path;
use std::sync::Arc;
use enum_helpers::OrdinalEnum;
use enum_helpers::NamedEnum;

use crate::config::ServerProperties;
use crate::server::Server;
use enum_helpers_derive::{OrdinalEnum, NamedEnum};
use protocol::fields::generic::Ordinal;

mod server;
mod packets;
mod net;
mod encryption;
pub mod config;
mod entity;
//
// #[tokio::main]
// async fn main() {
//     let properties = match ServerProperties::read(&Path::new("server.toml")) {
//         Ok(v) => v,
//         Err(why) => {
//             eprintln!("Error parsing server.toml: {}", why.to_string());
//             return;
//         }
//     };
//
//     let server = Server::new(properties).await;
//     Server::start(Arc::new(server)).await;
// }

#[derive(OrdinalEnum, Debug)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

fn main() {
    println!("{:?}", GameMode::from_ordinal(20))
}