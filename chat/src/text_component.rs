use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

use protocol::fields::identifier::Key;
use protocol::fields::PacketField;
use protocol::packet_io::{PacketReaderExt, PacketWriterExt};
use crate::component::Component;

use crate::events::{ClickEvent, HoverEvent};
use crate::style::Color;

#[derive(Serialize, Deserialize, Debug)]
pub struct TextComponent {
    pub text: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub bold: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub italic: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub strikethrough: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub underlined: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub obfuscated: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<TextComponent>,
}

fn is_false(b: impl std::borrow::Borrow<bool>) -> bool {
    !(*b.borrow())
}

impl Component for TextComponent {}

pub struct Builder {
    component: TextComponent,
}

impl TextComponent {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl Builder {
    fn new() -> Builder {
        Builder { component: TextComponent::default() }
    }

    pub fn text(mut self, text: String) -> Builder {
        self.component.text = text;
        self
    }

    pub fn bold(mut self) -> Builder {
        self.component.bold = true;
        self
    }

    pub fn italic(mut self) -> Builder {
        self.component.italic = true;
        self
    }

    pub fn underlined(mut self) -> Builder {
        self.component.underlined = true;
        self
    }

    pub fn strikethrough(mut self) -> Builder {
        self.component.strikethrough = true;
        self
    }

    pub fn obfuscated(mut self) -> Builder {
        self.component.obfuscated = true;
        self
    }

    pub fn font(mut self, font: Key) -> Builder {
        self.component.font = Some(font.into());
        self
    }

    pub fn color(mut self, color: &impl Color) -> Builder {
        self.component.color = Some(color.as_chat_string());
        self
    }

    pub fn hover_event(mut self, event: HoverEvent) -> Builder {
        self.component.hover_event = Some(event);
        self
    }

    pub fn click_event(mut self, event: ClickEvent) -> Builder {
        self.component.click_event = Some(event);
        self
    }

    pub fn push_child(mut self, child: TextComponent) -> Builder {
        self.component.extra.push(child);
        self
    }

    pub fn build(self) -> TextComponent {
        self.component
    }
}

impl Default for TextComponent {
    fn default() -> Self {
        TextComponent {
            text: "".to_string(),
            bold: false,
            italic: false,
            strikethrough: false,
            underlined: false,
            obfuscated: false,
            font: None,
            color: None,
            hover_event: None,
            click_event: None,
            extra: vec![],
        }
    }
}

impl PacketField for TextComponent {
    fn read_field<R: Read>(input: &mut R) -> std::io::Result<Self> where Self: Sized {
        let string = input.read_utf8()?;
        let value: TextComponent = serde_json::from_str(&string)?;
        Ok(value)
    }

    fn write_field<W: Write>(&self, output: &mut W) -> std::io::Result<usize> {
        let string = serde_json::to_string(&self)?;
        output.write_utf8(&string)
    }
}