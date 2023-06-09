use chat::ChatMode;
use protocol::{Clientbound, Serverbound};

use chat::text_component::TextComponent;
use protocol::fields::generic::Ordinal;
use protocol::fields::numeric::VarInt;
use protocol::fields::position::Position;

pub mod chunk;
pub mod section;

#[derive(Clientbound, Debug)]
#[packet(id = 0x00)]
pub struct BundleDelimiter;

#[derive(Clientbound, Debug)]
#[packet(id = 0x1A)]
pub struct DisconnectPlay {
    pub reason: TextComponent,
}

#[derive(Serverbound, Debug)]
#[packet(id = 0x08)]
pub struct ClientInformation {
    locale: String,
    view_distance: u8,
    chat_mode: Ordinal<ChatMode>,
    chat_colors: bool,
    display_skin_parts: u8,
    main_hand: VarInt,
}

#[derive(Clientbound, Debug)]
#[packet(id = 0x50)]
pub struct SetDefaultSpawnPosition {
    pub position: Position,
    pub angle: f32,
}