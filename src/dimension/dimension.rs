use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DimensionTypeRegistry {
    pub r#type: String,
    pub value: Vec<DimensionEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct DimensionEntry {
    pub name: String,
    pub id: i32,
    pub element: DimensionType,
}

#[derive(Serialize, Deserialize)]
pub struct DimensionType {
    pub piglin_safe: i8,
    pub has_raids: i8,
    pub monster_spawn_light_level: i32,
    pub monster_spawn_block_light_limit: Option<i32>,
    pub natural: i8,
    pub ambient_light: f32,
    pub infiniburn: String,
    pub respawn_anchor_works: i8,
    pub has_skylight: i8,
    pub bed_works: i8,
    pub effects: String,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub coordinate_scale: f64,
    pub ultrawarm: i8,
    pub has_ceiling: i8,
}