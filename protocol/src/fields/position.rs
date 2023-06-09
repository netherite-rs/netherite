use std::io::{Read, Result, Write};
use std::num::Wrapping;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::fields::PacketField;

#[derive(Debug, PartialEq)]
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

impl PacketField for Position {
    fn read_field<R: Read>(input: &mut R) -> Result<Self> where Self: Sized {
        let val = input.read_u64::<BigEndian>()?;
        let mut x = Wrapping(val >> 38_u64);
        let mut y = Wrapping(val & 0xFFF_u64);
        let mut z = Wrapping((val >> 12) & 0x3FFFFFF_u64);

        if x >= Wrapping(1 << 25) { x -= 1 << 26 }
        if y >= Wrapping(1 << 11) { y -= 1 << 12 }
        if z >= Wrapping(1 << 25) { z -= 1 << 26 }

        Ok(Position::new(x.0 as i32, y.0 as i16, z.0 as i32))
    }

    fn write_field<W: Write>(&self, output: &mut W) -> Result<()> {
        let value = (((self.x as i64 & 0x3FFFFFF) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | (self.y & 0xFFF_i16) as i64) as u64;
        output.write_u64::<BigEndian>(value)?;
        Ok(())
    }
}