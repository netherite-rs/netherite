use std::f32::consts::PI;
use std::io::{Read, Result, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::fields::io_ext::{PacketReaderExt, PacketWriterExt};
use crate::fields::PacketField;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VarInt(pub i32);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VarLong(pub i64);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Angle(pub u8);

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct BitSet(Vec<i64>);

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
        output.write_varint(&self)
    }
}

impl PacketField for VarLong {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        input.read_varlong()
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        output.write_varlong(&self)
    }
}

impl PacketField for Angle {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        Ok(Angle(input.read_u8()?))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        output.write_u8(self.0)?;
        Ok(())
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

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        if self.is_some() {
            output.write_u8(self.unwrap())?;
            return Ok(());
        }
        Ok(())
    }
}

impl PacketField for i8 {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        input.read_i8()
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        output.write_i8(*self)?;
        Ok(())
    }
}

macro_rules! field_for_numeric {
    ($($p_type:ident),*)=> {
        paste::item! {
            $(impl PacketField for $p_type {
                fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
                    input.[<read_$p_type>]::<BigEndian>()
                }

                fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
                    output.[<write_$p_type>]::<BigEndian>(*self)?;
                    Ok(())
                }
            })*
        }
    }
}

field_for_numeric! { u16, u32, u64, u128, i16, i32, i64, i128, f32, f64 }

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

impl PacketField for BitSet {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let value = input.read_field::<Vec<i64>>()?;
        Ok(BitSet(value))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        output.write_field(&self.0)
    }
}

impl BitSet {
    pub fn new() -> Self {
        BitSet(Vec::new())
    }

    pub fn get(&self, index: usize) -> bool {
        (self.0[index / 64] & (1 << (index % 64))) != 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if value {
            self.0[index / 64] |= 1 << (index % 64)
        } else {
            self.0[index / 64] &= !(1 << (index % 64))
        }
    }

    pub fn data(&self) -> &Vec<i64> {
        &self.0
    }
}
