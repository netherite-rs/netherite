use io::Result;
use std::io;
use std::io::{Error, Read, Write};
use std::io::ErrorKind::InvalidData;
use std::num::Wrapping;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::fields::{PacketField, Position, VarInt, VarLong};

const SEGMENT_BITS: u32 = 0x7F; /* = 127 */
const CONTINUE_BIT: u32 = 0x80; /* = 128 */
const MAX_VARINT_BITS: u32 = 32;
const MAX_VARLONG_BITS: usize = 64;

pub trait PacketReaderExt: Read {
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
        let mut value: u64 = 0;
        let mut position = 0_u64;
        loop {
            let current_byte = self.read_u8()?;
            value |= ((current_byte & SEGMENT_BITS as u8) as u64) << position;
            if (current_byte & CONTINUE_BIT as u8) == 0 {
                break;
            }
            position += 7;
            if position >= MAX_VARLONG_BITS as u64 {
                return Err(Error::new(InvalidData, "VarLong too big"));
            }
        }
        Ok(VarLong(value as i64))
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

    fn read_position(&mut self) -> Result<Position> {
        let val = self.read_u64::<BigEndian>()?;
        let mut x = Wrapping(val >> 38_u64);
        let mut y = Wrapping(val & 0xFFF_u64);
        let mut z = Wrapping((val >> 12) & 0x3FFFFFF_u64);

        if x >= Wrapping(1 << 25) { x -= 1 << 26 }
        if y >= Wrapping(1 << 11) { y -= 1 << 12 }
        if z >= Wrapping(1 << 25) { z -= 1 << 26 }

        Ok(Position::new(x.0 as i32, y.0 as i16, z.0 as i32))
    }
}

pub trait PacketWriterExt: Write {
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

    fn write_position(&mut self, value: &Position) -> Result<usize> {
        let value = (((value.x as i64 & 0x3FFFFFF) << 38) | ((value.z as i64 & 0x3FFFFFF) << 12) | (value.y & 0xFFF_i16) as i64) as u64;
        self.write_u64::<BigEndian>(value)?;
        Ok(8)
    }

    fn write_field(&mut self, value: &impl PacketField) -> Result<()> {
        value.write_field(self)
    }
}

/// All types that implement `Write` get methods defined in `PacketWriterExt`
/// for free.
impl<W: Write + ?Sized> PacketWriterExt for W {}

/// All types that implement `Read` get methods defined in `PacketReaderExt`
/// for free.
impl<R: Read + ?Sized> PacketReaderExt for R {}
