use std::io::Error;
use std::io::{ErrorKind, Read, Write};

use serde::de::value::BoolDeserializer;
use uuid::Uuid;

use crate::fields::PacketField;
use crate::packet_io::{PacketReaderExt, PacketWriterExt};

#[derive(Debug)]
pub struct GameProfile {
    pub username: String,
    pub uuid: Uuid,
    pub properties: Vec<Property>,
}

impl PacketField for GameProfile {
    fn read_field<R: Read>(input: &mut R) -> std::io::Result<Self> where Self: Sized {
        let uuid: Uuid = input.read_field()?;
        let username = input.read_utf8()?;
        if username.len() > 16 {
            return Err(Error::new(ErrorKind::InvalidData, "username cannot be longer than 16 characters."));
        }
        let properties = input.read_field()?;
        Ok(GameProfile {
            username,
            uuid,
            properties,
        })
    }

    fn write_field<W: Write>(&self, output: &mut W) -> std::io::Result<usize> {
        let mut size = output.write_field(&self.uuid)?;
        size += output.write_utf8(&self.username)?;
        size += output.write_field(&self.properties)?;
        Ok(size)
    }
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}

impl PacketField for Property {
    fn read_field<R: Read>(input: &mut R) -> std::io::Result<Self> where Self: Sized {
        let name = input.read_utf8().expect("failed to read 'name'");
        let value = input.read_utf8().expect("failed to read 'value'");
        let is_signed = input.read_bool().expect("failed to read 'is_signed'");
        let signature = input.read_field().expect("failed to read 'is_signed'");
        Ok(Property {
            name,
            value,
            is_signed,
            signature,
        })
    }

    fn write_field<W: Write>(&self, output: &mut W) -> std::io::Result<usize> {
        let mut size = output.write_utf8(&self.name)?;
        size += output.write_utf8(&self.value)?;
        size += output.write_bool(self.is_signed)?;
        size += output.write_field(&self.signature)?;
        Ok(size)
    }
}