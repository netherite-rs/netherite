use std::io::{Error, ErrorKind, Read, Result, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use nbt::Blob;
use uuid::Uuid;

use crate::packet_io::{PacketReaderExt, PacketWriterExt};

pub mod numeric;
pub mod position;
pub mod generic;
pub mod identifier;
pub mod profile;

pub trait PacketField {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized;
    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize>;
}

impl PacketField for String {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        input.read_utf8()
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        output.write_utf8(&self)
    }
}

impl PacketField for bool {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        input.read_bool()
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        output.write_bool(*self)
    }
}

impl PacketField for Blob {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        match Blob::from_reader::<R>(input) {
            Ok(v) => Ok(v),
            Err(why) => Err(Error::new(ErrorKind::InvalidData, why.to_string()))
        }
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        match self.to_writer(output) {
            Ok(_) => Ok(self.len_bytes()),
            Err(why) => Err(Error::new(ErrorKind::InvalidData, why.to_string()))
        }
    }
}

impl PacketField for Uuid {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let most_sig = input.read_u64::<BigEndian>()?;
        let least_sig = input.read_u64::<BigEndian>()?;
        Ok(Uuid::from_u64_pair(most_sig, least_sig))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        let (most_sig, least_sig) = self.as_u64_pair();
        output.write_u64::<BigEndian>(most_sig)?;
        output.write_u64::<BigEndian>(least_sig)?;
        Ok(2 * core::mem::size_of::<u64>())
    }
}
