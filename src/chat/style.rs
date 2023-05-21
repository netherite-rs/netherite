use std::fmt;
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use crate::protocol::fields::key::Key;

lazy_static! {
    static ref UNIFORM_FONT: Key = Key::minecraft("uniform");
    static ref ALT_FONT: Key = Key::minecraft("alt");
    static ref DEFAULT_FONT: Key = Key::minecraft("default");
}

/// Represents a color that can be represented in a [TextComponent].
///
/// Implementations:
/// - NamedTextColor
/// - RgbColor
pub trait Color {
    fn as_chat_string(&self) -> String;
}

pub struct RgbColor {
    color: i32,
}

impl RgbColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        let mut rgb = red as i32;
        rgb = (rgb << 8) + green as i32;
        rgb = (rgb << 8) + blue as i32;
        Self { color: rgb }
    }

    pub fn from_hex(hex: i32) -> Self {
        Self { color: hex }
    }

    pub fn hex(&self) -> i32 {
        self.color
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        let r = (self.color >> 16) & 0xFF;
        let g = (self.color >> 8) & 0xFF;
        let b = (self.color >> 0) & 0xFF;
        (r as u8, g as u8, b as u8)
    }
}

impl Color for RgbColor {
    fn as_chat_string(&self) -> String {
        let (r, g, b) = self.rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

#[derive(Debug)]
pub enum NamedTextColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

impl fmt::Display for NamedTextColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Color for NamedTextColor {
    fn as_chat_string(&self) -> String {
        self.to_string().to_case(Case::Snake)
    }
}
