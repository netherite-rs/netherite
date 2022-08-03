use std::io::{Error, ErrorKind, Read, Result, Write};
use bytebuffer::ByteBuffer;

use flate2::read::ZlibDecoder;
use serde::__private::de::Content::ByteBuf;
use crate::bound::Clientbound;
use crate::fields::numeric::VarInt;

use crate::packet_io::{PacketReaderExt, PacketWriterExt};

const MAX_DATA_LENGTH: usize = 2097152;

/// Reads a compressed packet.
///
/// Returns the packet ID and data
pub fn read_compressed_packet(input: &mut impl Read) -> Result<(i32, Vec<u8>)> {
    let packet_length = input.read_varint()?.0 as usize;
    let data_length = input.read_varint()?.0 as usize;
    if data_length != 0 {
        let mut zlib = ZlibDecoder::new(input);
        let packet_id = zlib.read_varint_with_size()?;
        let mut buf = vec![0; data_length - packet_id.1];
        zlib.read_exact(buf.as_mut_slice())?;
        Ok((packet_id.0.0, buf))
    } else {
        let packet_id = input.read_varint_with_size()?;
        let data_length = packet_length - packet_id.1;
        let mut buf = vec![0; data_length - packet_id.1];
        input.read_exact(buf.as_mut_slice())?;
        Ok((packet_id.0.0, buf))
    }
}

pub fn read_uncompressed_packet(input: &mut impl Read) -> Result<(i32, Vec<u8>)> {
    let packet_length = input.read_varint()?.0 as usize;
    let packet_id = input.read_varint_with_size()?;
    let actual_length = packet_length - packet_id.1;
    if actual_length > MAX_DATA_LENGTH {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("received length {} is greater than the allowed maximum ({})", actual_length, MAX_DATA_LENGTH))
        );
    }
    let mut buf = vec![0; actual_length];

    input.read_exact(buf.as_mut_slice())?;
    Ok((packet_id.0.0, buf))
}

pub fn write_compressed_packet(packet: impl Clientbound, output: &mut impl Write) -> Result<usize> {
    let id = VarInt(packet.id());
    let mut length = id.size();
    let mut buf = ByteBuffer::new();
    packet.write_packet(&mut buf)?;
    length += buf.len();
}