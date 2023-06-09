pub mod profile;

use num_bigint::BigInt;
use sha1::{Digest, Sha1};
use crate::profile::GameProfile;

const HAS_JOINED: &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined";

pub fn generate_server_hash(der_key: &Vec<u8>, key: &[u8]) -> String {
    let mut hasher = Sha1::default();
    hasher.update(b"");
    hasher.update(key);
    hasher.update(der_key);
    let hex = hasher.finalize();
    let bigint = BigInt::from_signed_bytes_be(hex.as_slice());
    format!("{:x}", bigint)
}

pub async fn authenticate(player: &str, hex: &str, ip: &str) -> GameProfile {
    let response = reqwest::get(format!(
        "{}?username={}&serverId={}&ip={}",
        HAS_JOINED,
        player,
        hex,
        ip,
    )).await;
    if response.is_err() {
        panic!("couldn't authenticate player {} using Mojang.", player);
    }
    response.unwrap().json::<GameProfile>().await.unwrap()
}
