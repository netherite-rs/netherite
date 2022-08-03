use std::f32::consts::PI;
use std::io::{Read, Result, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::fields::PacketField;
use crate::packet_io::{PacketReaderExt, PacketWriterExt};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VarInt(pub i32);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VarLong(pub i64);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
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

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        output.write_varint(&self)?;
        Ok(())
    }
}

impl PacketField for VarLong {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        input.read_varlong()
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        output.write_varlong(&self)?;
        Ok(())
    }
}

impl PacketField for Angle {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        Ok(Angle(input.read_u8()?))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        output.write_u8(self.0)
    }
}

impl PacketField for Option<u8> {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        Ok(Some(input.read_u8()?))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        if self.is_some() {
            output.write_u8(self.unwrap())?;
        }
        Ok(())
    }
}

impl PacketField for i8 {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        input.read_i8()
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        output.write_i8(*self)
    }
}

macro_rules! field_for_numeric {
    ($($p_type:ident),*)=> {
        paste::item!{
            $(impl PacketField for $p_type {
                fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
                    input.[<read_$p_type>]::<BigEndian>()
                }

                fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
                    output.[<write_$p_type>]::<BigEndian>(*self)
                }
            })*
        }
    }
}

field_for_numeric! { u16, u32, u64, u128, i16, i32, i64, i128}
