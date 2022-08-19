use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Range {
    pub max_inclusive: i32,
    pub min_inclusive: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Int32Range {
    pub value: Range,
    #[serde(rename = "type")]
    pub r#type: String,
}

