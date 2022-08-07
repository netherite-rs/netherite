use io::Result;
use std::io;
use std::io::{Error, Read, Write};
use std::io::ErrorKind::InvalidData;

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::fields::numeric::{VarInt, VarLong};
use crate::fields::PacketField;

const SEGMENT_BITS: u32 = 0x7F; /* = 127 */
const CONTINUE_BIT: u32 = 0x80; /* = 128 */
const MAX_VARINT_BITS: usize = 32;
const MAX_VARLONG_BITS: usize = 64;

pub trait PacketReaderExt: Read + Sized {

    fn read_varint(&mut self) -> Result<VarInt> {
        let mut value: u32 = 0;
        let mut position = 0_u32;
        loop {
            let current_byte = self.read_u8()?;
            value |= ((current_byte & SEGMENT_BITS as u8) as u32) << position;
            if (current_byte & CONTINUE_BIT as u8) == 0 {
                break;
            }
            position += 7;
            if position >= MAX_VARINT_BITS as u32 {
                return Err(Error::new(InvalidData, "VarInt too big"));
            }
        }
        Ok(VarInt(value as i32))
    }

    fn read_varlong(&mut self) -> Result<VarLong> {
        self.read_varlong_with_size().map(|t| t.0)
    }

    fn read_varlong_with_size(&mut self) -> Result<(VarLong, usize)> {
        let mut value: u64 = 0;
        let mut size = 0_u64;
        loop {
            let current_byte = self.read_u8()?;
            value |= ((current_byte & SEGMENT_BITS as u8) as u64) << size;
            if (current_byte & CONTINUE_BIT as u8) == 0 {
                break;
            }
            size += 7;
            if size >= MAX_VARLONG_BITS as u64 {
                return Err(Error::new(InvalidData, "VarLong too big"));
            }
        }
        Ok((VarLong(value as i64), (size / 7) as usize))
    }

    fn read_utf8(&mut self) -> Result<String> {
        let size = self.read_varint()?.0 as usize;
        let mut string = vec![0; size];
        self.read_exact(string.as_mut_slice())?;
        match String::from_utf8(string) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(InvalidData, e.to_string()))
        }
    }

    fn read_bool(&mut self) -> Result<bool> {
        let v = self.read_u8()?;
        Ok(v == 1)
    }

    fn read_field<T: PacketField>(&mut self) -> Result<T> {
        T::read_field(self)
    }
}

pub trait PacketWriterExt: Write + Sized {
    fn write_varint(&mut self, value: &VarInt) -> Result<usize> {
        let mut value = value.0 as u32;
        let mut size: usize = 1;
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                self.write_u8(value as u8)?;
                return Ok(size);
            }
            self.write_u8(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8)?;
            value >>= 7;
            size += 1;
        }
    }

    fn write_varlong(&mut self, value: &VarLong) -> Result<usize> {
        let mut value = value.0 as u64;
        let mut size: usize = 1;
        loop {
            if (value & (!SEGMENT_BITS) as u64) == 0 {
                self.write_u8(value as u8)?;
                return Ok(size);
            }
            self.write_u8(((value & SEGMENT_BITS as u64) | CONTINUE_BIT as u64) as u8)?;
            value >>= 7u64;
            size += 1;
        }
    }

    fn write_utf8(&mut self, value: &String) -> Result<usize> {
        let bytes = value.as_bytes();
        let size = self.write_varint(&VarInt(bytes.len() as i32))?;
        self.write_all(bytes)?;
        Ok(size + bytes.len())
    }

    fn write_bool(&mut self, value: bool) -> Result<usize> {
        self.write_u8(if value { 1 } else { 0 })?;
        Ok(1)
    }

    fn write_field(&mut self, value: &impl PacketField) -> Result<usize> {
        value.write_field(self)
    }
}

/// All types that implement `Write` get methods defined in `PacketWriterExt`
/// for free.
impl<W: Write> PacketWriterExt for W {}

/// All types that implement `Read` get methods defined in `PacketReaderExt`
/// for free.
impl<R: Read> PacketReaderExt for R {}
