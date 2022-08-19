use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BiomeRegistry {
    pub r#type: String,
    pub value: Vec<BiomeEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct BiomeEntry {
    pub name: String,
    pub id: i32,
    pub element: BiomeProperties,
}

#[derive(Serialize, Deserialize)]
pub struct BiomeProperties {
    pub precipitation: String,
    pub temperature: f32,
    pub downfall: f32,
    pub effects: Effects,
}

#[derive(Serialize, Deserialize)]
pub struct Effects {
    pub water_color: i32,
    pub mood_sound: MoodSound,
    pub water_fog_color: i32,
    pub fog_color: i32,
    pub sky_color: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MoodSound {
    pub sound: String,
    pub offset: f64,
    pub block_search_extent: i32,
    pub tick_delay: i32,
}
