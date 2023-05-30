use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelChunk {
    data_version: u32,
    #[serde(rename = "xPos")]
    x_pos: i32,
    #[serde(rename = "yPos")]
    y_pos: i32,
    #[serde(rename = "zPos")]
    z_pos: i32,
    heightmaps: Heightmap,
    inhabited_time: i64,
    last_update: u64,
    post_processing: Vec<Vec<i16>>,
    status: RegionStatus,
}

pub struct LevelBlockEntity {
    keep_packed: bool,
    id: String,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegionStatus {
    Empty,
    StructureStarts,
    StructureReferences,
    Biomes,
    Noise,
    Surface,
    Carvers,
    LiquidCarvers,
    Features,
    Light,
    Spawn,
    Heightmaps,
    Full
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Heightmap {
    pub motion_blocking: Vec<i64>,
    pub ocean_floor: Vec<i64>,
    pub world_surface: Vec<i64>,
    pub motion_blocking_no_leaves: Vec<i64>,
}
