use std::io::{Error, ErrorKind, Read, Result, Write};

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::protocol::fields::PacketField;
use crate::protocol::packet_io::{PacketReaderExt, PacketWriterExt};
use crate::util::StringVisitor;

lazy_static! {
    static ref NAMESPACE_REGEX: Regex = Regex::new(r"[a-z0-9._-]+").unwrap();
    static ref VALUE_REGEX: Regex = Regex::new("[a-z0-9.-_/]+").unwrap();
}

static MINECRAFT: &str = "minecraft";

#[derive(Debug)]
pub struct Key {
    namespace: String,
    value: String,
}

impl Key {
    pub fn minecraft(value: &str) -> Self {
        Self::new(MINECRAFT, value)
    }

    pub fn new(namespace: &str, value: &str) -> Self {
        if !NAMESPACE_REGEX.is_match(&namespace) {
            panic!("Namespace '{}' can only contain lowercase alphabet, underscores, dots, dashes and numbers.", namespace)
        }
        if !VALUE_REGEX.is_match(&value) {
            panic!("Value '{}' can only contain lowercase alphabet, underscores, dots, dashes, slashes and numbers.", namespace)
        }
        let string = format!("{}:{}", namespace, value);
        if string.len() >= 256 {
            panic!("Identifiers must be less than 256 characters")
        }
        Self { namespace: namespace.to_string(), value: value.to_string() }
    }

    pub fn parse(string: &str) -> Result<Key> {
        if string.len() >= 256 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Identifiers must be less than 256 characters. Found: {}", string.len()))
            );
        }
        let split = match string.split_once(':') {
            None => return Err(Error::new(ErrorKind::InvalidData, format!("Identifier does not contain ':'. String: {:?}", string))),
            Some(v) => v
        };
        if !NAMESPACE_REGEX.is_match(split.0) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Namespace '{}' can only contain lowercase alphabet, underscores, dots, dashes and numbers.", split.0))
            );
        }
        if !VALUE_REGEX.is_match(split.1) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Value '{}' can only contain lowercase alphabet, underscores, dots, dashes, slashes and numbers.", split.1))
            );
        }

        Ok(Key::new(split.0, split.1))
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PacketField for Key {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let string = input.read_utf8()?;
        Key::parse(&string)
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        let string = format!("{}:{}", self.namespace, self.value);
        output.write_utf8(&string)
    }
}

impl From<String> for Key {
    fn from(v: String) -> Self {
        Key::parse(&v).unwrap()
    }
}

impl Into<String> for Key {
    fn into(self) -> String {
        format!("{}:{}", self.namespace, self.value)
    }
}

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer {
        let string = format!("{}:{}", self.namespace, self.value);
        serializer.serialize_str(&string)
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: Deserializer<'de> {
        let value = deserializer.deserialize_str(StringVisitor)?;
        return Ok(Key::from(value));
    }
}