use std::fmt::Debug;
use std::io::{Error, ErrorKind, Read, Result, Write};

use serde::de::DeserializeOwned;
use serde::Serialize;

use enum_utils::OrdinalEnum;

use crate::fields::numeric::VarInt;
use crate::fields::PacketField;
use crate::packet_io::{PacketReaderExt, PacketWriterExt};

/// A basic wrapper that handles reading and writing of the internal
/// type by converting them to and from JSON
#[derive(PartialEq, Debug)]
pub struct Json<T: Serialize + DeserializeOwned>(pub T);

/// An ordinal writes the ordinal of its inner value
#[derive(PartialEq, Debug)]
pub struct Ordinal<T: OrdinalEnum>(pub T);

/// An Option that must know beforehand whether it is present or not.
/// This is done by encoding a boolean before the actual value, indicating
/// whether it's present or not.
#[derive(PartialEq, Debug)]
pub struct KnownOption<T>(pub Option<T>);

impl<const S: usize> PacketField for [u8; S] {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let mut buf = [0; S];
        input.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        output.write_all(&self[..])?;
        Ok(self.len())
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

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        let mut size: usize = 0;
        for i in 0..self.len() {
            size += output.write_field(&self[i])?;
        }
        Ok(size)
    }
}

impl<T: PacketField> PacketField for Option<T> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let value = input.read_field::<T>();
        match value {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None)
        }
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        if self.is_some() {
            return output.write_field(self.as_ref().unwrap());
        }
        Ok(0)
    }
}

impl<T: Serialize + DeserializeOwned> PacketField for Json<T> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let string = input.read_utf8()?;
        let value: T = serde_json::from_str(&string)?;
        Ok(Json(value))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        let string = serde_json::to_string(&self.0)?;
        output.write_utf8(&string)
    }
}

impl<T: PacketField> PacketField for Vec<T> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let length = input.read_varint()?.0 as usize;
        let mut buf = Vec::with_capacity(length);
        for _ in 0..length {
            buf.push(input.read_field::<T>()?)
        }
        Ok(buf)
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        let mut size = output.write_varint(&VarInt(self.len() as i32))?;
        for elem in self {
            size += output.write_field(elem)?;
        }
        Ok(size)
    }
}

impl PacketField for Vec<u8> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let length = input.read_varint()?.0 as usize;
        let mut buf = vec![0_u8; length];
        input.read_exact(buf.as_mut_slice())?;
        Ok(buf)
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        let mut size = output.write_varint(&VarInt(self.len() as i32))?;
        output.write_all(self.as_slice())?;
        size += self.len();
        Ok(size)
    }
}

impl<T: OrdinalEnum> PacketField for Ordinal<T> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let ordinal = input.read_varint()?.0 as u32;
        let result = T::from_ordinal(ordinal);
        match result {
            Ok(v) => Ok(Ordinal(v)),
            Err(why) => Err(Error::new(ErrorKind::InvalidData, why))
        }
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        output.write_varint(&VarInt(self.0.ordinal() as i32))
    }
}

impl<T: PacketField> PacketField for KnownOption<T> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let present = input.read_bool()?;
        if present {
            return Ok(KnownOption(Some(input.read_field::<T>()?)));
        }
        return Ok(KnownOption(None));
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        let mut size = 0;
        let opt = self.0.as_ref();
        size += output.write_bool(opt.is_some())?;
        if opt.is_some() {
            size += output.write_field(opt.unwrap())?;
        }
        Ok(size)
    }
}