use std::fmt::Formatter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

use enum_helpers_derive::{NamedEnum, OrdinalEnum};

#[derive(NamedEnum, OrdinalEnum, Debug)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Serialize for GameMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            GameMode::Survival => serializer.serialize_str("survival"),
            GameMode::Creative => serializer.serialize_str("creative"),
            GameMode::Adventure => serializer.serialize_str("adventure"),
            GameMode::Spectator => serializer.serialize_str("spectator")
        }
    }
}

struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(String::from(v))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: Error {
        Ok(String::from(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Ok(v)
    }
}

impl<'de> Deserialize<'de> for GameMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let result = deserializer.deserialize_string(StrVisitor)?;
        let string = result.as_str();
        match string {
            "survival" => Ok(GameMode::Survival),
            "creative" => Ok(GameMode::Creative),
            "adventure" => Ok(GameMode::Adventure),
            "spectator" => Ok(GameMode::Spectator),
            v => Err(D::Error::custom(format!("invalid gamemode: {}. Expected 'survival', 'creative', 'adventure' or 'spectator'.", v)))
        }
    }
}