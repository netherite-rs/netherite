extern crate core;

use serde::de::DeserializeOwned;
use toml::de::Error;

use crate::packets_generator::Packets;

mod packets_generator;

fn main() {
    let packets = parse::<Packets>(include_bytes!("packets.toml"));
    if packets.is_err() {
        eprintln!("{}", packets.err().unwrap().to_string());
        return;
    }
    packets_generator::process(packets.unwrap());
}

fn parse<T: DeserializeOwned>(file: &[u8]) -> Result<T, Error> {
    toml::from_slice::<T>(&file)
}
