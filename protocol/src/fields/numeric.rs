use std::f32::consts::PI;
use std::io::{Read, Result, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::fields::PacketField;
use crate::packet_io::{PacketReaderExt, PacketWriterExt};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VarInt(pub i32);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VarLong(pub i64);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Angle(pub u8);

impl Angle {
    pub fn to_deg(&self) -> f32 {
        360.0 * self.0 as f32 / 256.0
    }

    pub fn to_rad(&self) -> f32 {
        2.0 * PI * self.0 as f32 / 256.0
    }
}

impl PacketField for VarInt {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        input.read_varint()
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        output.write_varint(&self)
    }
}

impl PacketField for VarLong {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        input.read_varlong()
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        output.write_varlong(&self)
    }
}

impl PacketField for Angle {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        Ok(Angle(input.read_u8()?))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        output.write_u8(self.0)?;
        Ok(1)
    }
}

impl PacketField for Option<u8> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let v = input.read_u8();
        match v {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None)
        }
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        if self.is_some() {
            output.write_u8(self.unwrap())?;
            return Ok(1);
        }
        Ok(0)
    }
}

impl PacketField for i8 {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        input.read_i8()
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
        output.write_i8(*self)?;
        Ok(1)
    }
}

macro_rules! field_for_numeric {
    ($($p_type:ident),*)=> {
        paste::item!{
            $(impl PacketField for $p_type {
                fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
                    input.[<read_$p_type>]::<BigEndian>()
                }

                fn write_field<W: Write>(&self, output: &mut W) -> Result<usize> {
                    output.[<write_$p_type>]::<BigEndian>(*self)?;
                    Ok(core::mem::size_of::<[<$p_type>]>())
                }
            })*
        }
    }
}

field_for_numeric! { u16, u32, u64, u128, i16, i32, i64, i128}

impl VarInt {
    pub fn size(&self) -> usize {
        let mut value = self.0 as u32;
        let mut size: usize = 1;
        loop {
            if (value & !0x7F) == 0 {
                return size;
            }
            value >>= 7;
            size += 1;
        }
    }
}

impl VarLong {
    pub fn size(&self) -> usize {
        let mut value = self.0 as u64;
        let mut size: usize = 1;
        loop {
            if (value & !0x7F) == 0 {
                return size;
            }
            value >>= 7;
            size += 1;
        }
    }
}