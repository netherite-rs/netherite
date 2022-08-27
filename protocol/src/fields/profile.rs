use std::io::{ErrorKind, Read, Write};
use std::io::Error;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::fields::generic::KnownOption;
use crate::fields::PacketField;
use crate::packet_io::{PacketReaderExt, PacketWriterExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameProfile {
    pub name: String,
    pub id: Uuid,
    pub properties: Vec<Property>,
}

impl GameProfile {
    pub fn new(name: String, id: Uuid) -> Self {
        Self { name, id, properties: vec![] }
    }

    pub fn offline(name: &str) -> Self {
        GameProfile {
            name: name.to_string(),
            id: GameProfile::offline_mode_uuid(name),
            properties: vec![],
        }
    }

    fn offline_mode_uuid(username: &str) -> Uuid {
        let digest = md5::compute(format!("OfflinePlayer:{}", username).as_bytes());
        let mut builder = uuid::Builder::from_bytes(digest.try_into().unwrap());
        builder
            .set_variant(uuid::Variant::RFC4122)
            .set_version(uuid::Version::Md5);
        builder.into_uuid()
    }
}

impl PacketField for GameProfile {
    fn read_field<R: Read>(input: &mut R) -> std::io::Result<Self> where Self: Sized {
        let id: Uuid = input.read_field()?;
        let name = input.read_utf8()?;
        if name.len() > 16 {
            return Err(Error::new(ErrorKind::InvalidData, "username cannot be longer than 16 characters."));
        }
        let properties = input.read_field()?;
        Ok(GameProfile {
            name,
            id,
            properties,
        })
    }

    fn write_field<W: Write>(&self, output: &mut W) -> std::io::Result<usize> {
        let mut size = output.write_field(&self.id)?;
        size += output.write_utf8(&self.name)?;
        size += output.write_field(&self.properties)?;
        Ok(size)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl PacketField for Property {
    fn read_field<R: Read>(input: &mut R) -> std::io::Result<Self> where Self: Sized {
        let name = input.read_utf8().expect("failed to read 'name'");
        let value = input.read_utf8().expect("failed to read 'value'");
        let signature = input.read_field::<KnownOption<String>>().expect("failed to read 'signature'");
        Ok(Property {
            name,
            value,
            signature: signature.0,
        })
    }

    fn write_field<W: Write>(&self, output: &mut W) -> std::io::Result<usize> {
        let mut size = output.write_utf8(&self.name)?;
        size += output.write_utf8(&self.value)?;
        match &self.signature {
            Some(v) => {
                size += output.write_bool(true)?;
                size += output.write_utf8(v)?;
            }
            None => {
                size += output.write_bool(false)?;
            }
        }
        Ok(size)
    }
}