use protocol_derive::Clientbound;

use crate::chat::text_component::TextComponent;
use crate::protocol::fields::position::Position;

pub mod chunk;
pub mod section;

#[derive(Clientbound, Debug)]
#[packet(id = 0x00)]
pub struct BundleDelimiter;

#[derive(Clientbound, Debug)]
#[packet(id = 0x19)]
pub struct DisconnectPlay {
    pub reason: TextComponent,
}

#[derive(Clientbound, Debug)]
#[packet(id = 0x50)]
pub struct SetDefaultSpawnPosition {
    pub position: Position,
    pub angle: f32,
}