use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ChatRegistry {
    pub value: Vec<ChatRegistryEntry>,
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatRegistryEntry {
    pub name: String,
    pub id: i64,
    pub element: ChatElement,
}

#[derive(Serialize, Deserialize)]
pub struct ChatElement {
    pub chat: Option<Chat>,
    pub narration: Option<Narration>,
    pub overlay: Option<Style>,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub decoration: Option<Decoration>,
}

#[derive(Serialize, Deserialize)]
pub struct Narration {
    pub decoration: Option<Decoration>,
    pub priority: String,
}

#[derive(Serialize, Deserialize)]
pub struct Decoration {
    pub parameters: Vec<String>,
    pub translation_key: String,
    pub style: Style,
}

#[derive(Serialize, Deserialize)]
pub struct Style {
    pub italic: Option<i64>,
    pub color: Option<String>,
}

