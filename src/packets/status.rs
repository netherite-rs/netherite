
use serde::{Serialize, Deserialize};
use chat::text_component::TextComponent;
use protocol::fields::generic::Json;
use protocol_derive::{Clientbound, Serverbound};

#[derive(Serverbound)]
#[packet(id = 0x00)]
pub struct StatusRequest {}

#[derive(Serverbound, Clientbound)]
#[packet(id = 0x01)]
pub struct PingPacket {
    pub payload: i64,
}

#[derive(Clientbound, Debug)]
#[packet(id = 0x00)]
pub struct StatusResponse {
    pub response: Json<Response>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub version: RespVersion,
    pub players: RespPlayers,
    pub description: TextComponent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,

    #[serde(rename(serialize = "previewsChat"))]
    pub previews_chat: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespVersion {
    pub name: String,
    pub protocol: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPlayers {
    pub max: usize,
    pub online: usize,
    pub sample: Vec<RespPlayerSample>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPlayerSample {
    pub name: String,
    pub id: String,
}