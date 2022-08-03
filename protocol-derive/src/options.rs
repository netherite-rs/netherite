use darling::{FromDeriveInput, FromMeta};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(packet))]
pub struct PacketInfo {
    pub id: Option<i32>,
    // pub stage: PacketStage,
}

pub enum PacketStage {
    HANDSHAKE,
    STATUS,
    LOGIN,
    PLAY
}

impl FromMeta for PacketStage {}