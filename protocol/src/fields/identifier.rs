use std::io::{Read, Write, Result, Error, ErrorKind};
use lazy_static::lazy_static;
use regex::Regex;
use crate::fields::PacketField;
use crate::packet_io::{PacketReaderExt, PacketWriterExt};

lazy_static! {
    static ref NAMESPACE_REGEX: Regex = Regex::new(r"[a-z0-9._-]+").unwrap();
    static ref VALUE_REGEX: Regex = Regex::new("[a-z0-9.-_/]+").unwrap();
}

static MINECRAFT: &str = "minecraft";

#[derive(Debug)]
pub struct Identifier {
    namespace: String,
    value: String,
}

impl Identifier {
    pub fn minecraft(value: String) -> Self {
        Self::new(String::from(MINECRAFT), value)
    }

    pub fn new(namespace: String, value: String) -> Self {
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
        Self { namespace, value }
    }

    pub fn parse(string: &String) -> Result<Identifier> {
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

        Ok(Identifier::new(split.0.to_string(), split.1.to_string()))
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PacketField for Identifier {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let string = input.read_utf8()?;
        Identifier::parse(&string)
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        let string = format!("{}:{}", self.namespace, self.value);
        output.write_utf8(&string)
    }
}

impl From<String> for Identifier {
    fn from(v: String) -> Self {
        Identifier::parse(&v).unwrap()
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        format!("{}:{}", self.namespace, self.value)
    }
}