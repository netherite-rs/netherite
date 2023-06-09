use serde::{Deserialize, Serialize};

use protocol::fields::key::Key;

#[derive(Serialize, Deserialize)]
pub struct Dimension {
    key: Key,
}

//
// pub const OVERWORLD: DimensionType = DimensionType {
//     monster_spawn_light_level: 11,
//     monster_spawn_block_light_limit: 0,
//     ambient_light: 0.0,
//     infiniburn: String::from("minecraft:infiniburn_overworld"),
//     has_raids: true,
//     piglin_safe: false,
//     natural: true,
//     respawn_anchor_works: false,
//     has_skylight: true,
//     bed_works: true,
//     effects: String::from("minecraft:overworld"),
//     min_y: -64,
//     height: 384,
//     logical_height: 384,
//     coordinate_scale: 1,
//     ultrawarm: true,
//     has_ceiling: false,
// };
//
// #[derive(Serialize, Deserialize)]
// pub struct DimensionTypeRegistry {
//     pub r#type: String,
//     pub value: Vec<DimensionEntry>,
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct DimensionEntry {
//     pub name: String,
//     pub id: i32,
//     pub element: DimensionType,
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct DimensionType {
//     #[serde(default = "def_monster_spawn_light_level")]
//     pub monster_spawn_light_level: i32,
//     #[serde(default = "def_monster_spawn_block_light_limit")]
//     pub monster_spawn_block_light_limit: i32,
//     pub ambient_light: f32,
//     pub infiniburn: String,
//     pub has_raids: bool,
//     pub piglin_safe: bool,
//     pub natural: bool,
//     pub respawn_anchor_works: bool,
//     pub has_skylight: bool,
//     pub bed_works: bool,
//     pub effects: String,
//     pub min_y: i32,
//     pub height: i32,
//     pub logical_height: i32,
//     pub coordinate_scale: i32,
//     pub ultrawarm: bool,
//     pub has_ceiling: bool,
// }
//
// pub const fn def_monster_spawn_light_level() -> i32 {11}
// pub const fn def_monster_spawn_block_light_limit() -> i32 {0}