use serde::{Deserialize, Serialize};

use crate::dimension::biome::BiomeRegistry;
use crate::dimension::chat::ChatRegistry;
use crate::dimension::dimension::DimensionTypeRegistry;

#[derive(Serialize, Deserialize)]
pub struct Registry {
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: DimensionTypeRegistry,

    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome: BiomeRegistry,

    #[serde(rename = "minecraft:chat_type")]
    pub chat_type: ChatRegistry,
}
