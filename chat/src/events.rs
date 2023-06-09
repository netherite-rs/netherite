use serde::{Deserialize, Serialize};

use crate::serializers::{ComponentSerializer, JsonSerializer};
use crate::text_component::TextComponent;

#[derive(Serialize, Deserialize, Debug)]
pub struct HoverEvent {
    action: String,
    value: String,
}

impl HoverEvent {
    pub fn show_text(text: TextComponent) -> HoverEvent {
        HoverEvent {
            action: String::from("show_text"),
            value: JsonSerializer::serialize(&text).unwrap(),
        }
    }

    // fn show_entity(entity_type: Identifier, name: TextComponent, id: Uuid) -> HoverEvent {}
    //
    // fn show_item(item_type: Identifier, count: i32, nbt: Blob) -> HoverEvent {
    // }
    pub fn action(&self) -> &str {
        &self.action
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClickEvent {
    action: String,
    value: String,
}

impl ClickEvent {
    pub fn open_url(url: String) -> ClickEvent {
        ClickEvent {
            action: String::from("open_url"),
            value: url,
        }
    }

    pub fn run_command(command: String) -> ClickEvent {
        ClickEvent {
            action: String::from("run_command"),
            value: command,
        }
    }

    pub fn suggest_command(command: String) -> ClickEvent {
        ClickEvent {
            action: String::from("suggest_command"),
            value: command,
        }
    }

    pub fn change_page(page: u32) -> ClickEvent {
        ClickEvent {
            action: String::from("change_page"),
            value: format!("{}", page),
        }
    }

    pub fn copy_to_clipboard(text: String) -> ClickEvent {
        ClickEvent {
            action: String::from("copy_to_clipboard"),
            value: text,
        }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}