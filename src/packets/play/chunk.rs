use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use nbt::{Blob, Value};
use protocol::Clientbound;

use protocol::fields::numeric::{BitSet, VarInt};
use protocol::fields::PacketField;
use protocol::fields::io_ext::{PacketReaderExt, PacketWriterExt};

#[derive(Clientbound)]
#[packet(id = 0x21)]
pub struct ChunkPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Value,
    pub data: Vec<u8>,
    pub block_entities: Vec<u8>,
    pub trust_edges: bool,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light: [u8; 2048],
    pub block_light: [u8; 2048],
}

pub struct BlockEntity {
    section: u8,
    y: u16,
    block_type: VarInt,
    data: Blob,
}

impl BlockEntity {
    pub fn new(block_x: u8, block_z: u8, y: u16, block_type: VarInt, data: Blob) -> Self {
        let section = ((block_x & 15) << 4) | (block_z & 15);
        Self {
            section,
            y,
            block_type,
            data,
        }
    }
}

impl PacketField for BlockEntity {
    fn read_field<R: Read>(input: &mut R) -> std::io::Result<Self>
        where
            Self: Sized,
    {
        let section = input.read_u8().expect("failed to read 'section'");
        let y = input.read_u16::<BigEndian>().expect("failed to read 'y'");
        let block_type = input.read_varint().expect("failed to read 'block_type'");
        let data = input.read_field::<Blob>().expect("failed to read 'data'");
        Ok(BlockEntity {
            section,
            y,
            block_type,
            data,
        })
    }

    fn write_field<W: Write>(&self, output: &mut W) -> std::io::Result<()> {
        output.write_u8(self.section)?;
        output.write_u16::<BigEndian>(self.y)?;
        output.write_varint(&self.block_type)?;
        output.write_field(&self.data)?;
        Ok(())
    }
}

pub struct LightEntry {
    light: Vec<u8>,
}

impl LightEntry {
    pub fn new(light: Vec<u8>) -> Self {
        Self { light }
    }
}

impl PacketField for LightEntry {
    fn read_field<R: Read>(input: &mut R) -> std::io::Result<Self>
        where
            Self: Sized,
    {
        return Ok(LightEntry {
            light: input.read_field().expect("failed to read 'light'"),
        });
    }

    fn write_field<W: Write>(&self, output: &mut W) -> std::io::Result<()> {
        output.write_field(&self.light)?;
        Ok(())
    }
}
