use std::ops::Index;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Section {
    #[serde(rename = "Y")]
    y: i8,
    // block_states: nbt
}
