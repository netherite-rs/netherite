use std::io::{Error, ErrorKind, Read, Result, Write};

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::fields::PacketField;
use crate::packet_io::{PacketReaderExt, PacketWriterExt};

/// A basic wrapper that handles reading and writing of the internal
/// type by converting them to and from JSON
#[derive(PartialEq, Debug)]
pub struct Json<T: Serialize + DeserializeOwned>(pub T);

impl<const S: usize> PacketField for [u8; S] {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let mut buf = [0; S];
        input.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        output.write_all(&self[..])
    }
}

impl<const S: usize, T: PacketField> PacketField for [T; S] {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let mut buf: Vec<T> = vec![];
        for _ in 0..S {
            buf.push(input.read_field()?);
        }
        match buf.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "failed to convert Vec<T> to [T; S]"))
        }
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        for i in 0..self.len() {
            output.write_field(&self[i])?;
        }
        Ok(())
    }
}

impl<T: PacketField> PacketField for Option<T> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let value = input.read_field::<T>()?;
        Ok(Some(value))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        if self.is_some() {
            output.write_field(self.as_ref().unwrap())?;
        }
        Ok(())
    }
}

impl<T: Serialize + DeserializeOwned> PacketField for Json<T> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let string = input.read_utf8()?;
        let value: T = serde_json::from_str(&string)?;
        Ok(Json(value))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        let string = serde_json::to_string(&self.0)?;
        output.write_utf8(&string)?;
        Ok(())
    }
}
