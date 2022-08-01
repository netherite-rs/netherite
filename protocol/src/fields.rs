use std::f32::consts::PI;
use std::io::{Error, ErrorKind, Read, Write};
use std::io::Result;
use std::num::Wrapping;
use std::ops::Deref;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use nbt::Blob;
use uuid::Uuid;

use crate::packet_io::{PacketReaderExt, PacketWriterExt};

pub trait PacketField {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized;
    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()>;
}

pub type Identifier = String;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VarLong(pub i64);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VarInt(pub i32);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Angle(pub u8);

impl Angle {
    pub fn to_deg(&self) -> f32 {
        return 360 * self.0 as f32 / 256;
    }

    pub fn to_rad(&self) -> f32 {
        return 2 * PI * a as f32 / 256;
    }
}

impl PacketField for Angle {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        Ok(Angle(input.read_u8()?))
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        output.write_u8(self.0)
    }
}

impl Deref for VarInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for VarLong {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl PacketField for Uuid {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        let most_sig = input.read_u64::<BigEndian>()?;
        let least_sig = input.read_u64::<BigEndian>()?;
        Ok(Uuid::from_u64_pair(most_sig, least_sig))
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        let (most_sig, least_sig) = self.as_u64_pair();
        output.write_u64::<BigEndian>(most_sig)?;
        output.write_u64::<BigEndian>(least_sig)?;
        Ok(())
    }
}

impl PacketField for Position {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        let val = input.read_u64::<BigEndian>()?;
        let mut x = Wrapping(val >> 38_u64);
        let mut y = Wrapping(val & 0xFFF_u64);
        let mut z = Wrapping((val >> 12) & 0x3FFFFFF_u64);

        if x >= Wrapping(1 << 25) { x -= 1 << 26 }
        if y >= Wrapping(1 << 11) { y -= 1 << 12 }
        if z >= Wrapping(1 << 25) { z -= 1 << 26 }

        Ok(Position::new(x.0 as i32, y.0 as i16, z.0 as i32))
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        let value = (((self.x as i64 & 0x3FFFFFF) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | (self.y & 0xFFF_i16) as i64) as u64;
        output.write_u64::<BigEndian>(value)
    }
}

impl<const S: usize> PacketField for [u8; S] {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        let mut buf = [0; S];
        input.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        output.write_all(&self[..])
    }
}

impl<const S: usize, T: PacketField> PacketField for [T; S] {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        let mut buf: Vec<T> = vec![];
        for _ in 0..S {
            buf.push(input.read_field()?);
        }
        match buf.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "failed to convert Vec<T> to [T; S]"))
        }
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        for i in 0..self.len() {
            output.write_field(&self[i])?;
        }
        Ok(())
    }
}

macro_rules! field_for_numeric {
    ($($p_type:ident),*)=> {
        paste::item!{
            $(impl PacketField for $p_type {
                fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
                    input.[<read_$p_type>]::<BigEndian>()
                }

                fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
                    output.[<write_$p_type>]::<BigEndian>(*self)
                }
            })*
        }
    }
}

field_for_numeric! { u16, u32, u64, u128, i16, i32, i64, i128}

impl PacketField for i8 {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        input.read_i8()
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        output.write_i8(*self)
    }
}

impl PacketField for VarLong {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        input.read_varlong()
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        output.write_varlong(&self)?;
        Ok(())
    }
}

impl PacketField for VarInt {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        input.read_varint()
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        output.write_varint(&self)?;
        Ok(())
    }
}

impl PacketField for String {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        input.read_utf8()
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        output.write_utf8(&self)?;
        Ok(())
    }
}

impl PacketField for bool {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        input.read_bool()
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        output.write_bool(*self)?;
        Ok(())
    }
}

impl PacketField for Blob {
    fn read_field(input: &mut (impl Read + ?Sized)) -> Result<Self> where Self: Sized {
        match Blob::from_reader(input) {
            Ok(v) => Ok(v),
            Err(why) => Err(Error::new(ErrorKind::InvalidData, why.to_string()))
        }
    }

    fn write_field(&self, output: &mut (impl Write + ?Sized)) -> Result<()> {
        match self.to_writer(output) {
            Ok(v) => Ok(v),
            Err(why) => Err(Error::new(ErrorKind::InvalidData, why.to_string()))
        }
    }
}