use std::io::{Error, ErrorKind, Read, Result, Write, Cursor};

use bytes::{BytesMut, Buf};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

use bytebuffer::ByteBuffer;

use crate::protocol::Clientbound;
use crate::protocol::fields::numeric::VarInt;
use crate::protocol::packet_io::{PacketReaderExt, PacketWriterExt};

const MAX_DATA_LENGTH: usize = 2097152;


pub fn read_packet(input: &mut BytesMut, threshold: i32) -> Result<(i32, Vec<u8>)> {
    if threshold >= 0 {
        read_compressed_packet_buf(input)
    } else {
        read_uncompressed_packet(input)
    }
}

pub fn read_uncompressed_packet(input: &mut BytesMut) -> Result<(i32, Vec<u8>)> {
    let mut cursor = Cursor::new(&input[..]);
    let packet_length = cursor.read_varint()?.0 as usize;
    let packet_id = cursor.read_varint()?;
    let data_length = packet_length - packet_id.size();
    if data_length > MAX_DATA_LENGTH {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("the received data length {} is greater than the allowed maximum ({})", data_length, MAX_DATA_LENGTH))
        );
    }
    let mut buf = vec![0; data_length];
    cursor.read_exact(buf.as_mut_slice())?;
    input.advance(cursor.position() as usize);
    Ok((packet_id.0, buf))
}

/// Reads a compressed packet.
///
/// Returns the packet ID and data
pub fn read_compressed_packet_buf(input: &mut BytesMut) -> Result<(i32, Vec<u8>)> {
    let mut cursor = Cursor::new(&input[..]);
    let packet_length = cursor.read_varint()?.0 as usize;
    let data_length = cursor.read_varint()?.0 as usize;
    if data_length != 0 {
        let mut zlib = ZlibDecoder::new(cursor);
        let packet_id = zlib.read_varint()?;
        let mut buf = vec![0; data_length - packet_id.size()];
        zlib.read_exact(buf.as_mut_slice())?;
        let cursor = zlib.into_inner();
        input.advance(cursor.position() as usize);
        Ok((packet_id.0, buf))
    } else {
        let packet_id = cursor.read_varint()?;
        let data_length = packet_length - packet_id.size();
        let mut buf = vec![0; data_length - packet_id.size()];
        cursor.read_exact(buf.as_mut_slice())?;
        input.advance(cursor.position() as usize);
        Ok((packet_id.0, buf))
    }
}

// pub fn read_packet(input: &mut ByteBuffer, threshold: i32) -> Result<(i32, Vec<u8>)> {
//     if threshold >= 0 {
//         read_compressed_packet(input)
//     } else {
//         read_uncompressed_packet(input)
//     }
// }

// /// Reads a compressed packet.
// ///
// /// Returns the packet ID and data
// pub fn read_compressed_packet(input: &mut ByteBuffer) -> Result<(i32, Vec<u8>)> {
//     let packet_length = input.read_varint()?.0 as usize;
//     let data_length = input.read_varint()?.0 as usize;
//     if data_length != 0 {
//         let mut zlib = ZlibDecoder::new(input);
//         let packet_id = zlib.read_varint()?;
//         let mut buf = vec![0; data_length - packet_id.size()];
//         zlib.read_exact(buf.as_mut_slice())?;
//         Ok((packet_id.0, buf))
//     } else {
//         let packet_id = input.read_varint()?;
//         let data_length = packet_length - packet_id.size();
//         let mut buf = vec![0; data_length - packet_id.size()];
//         input.read_exact(buf.as_mut_slice())?;
//         Ok((packet_id.0, buf))
//     }
// }

// pub fn read_uncompressed_packet(input: &mut ByteBuffer) -> Result<(i32, Vec<u8>)> {
//     let packet_length = input.read_varint()?.0 as usize;
//     let packet_id = input.read_varint()?;
//     let data_length = packet_length - packet_id.size();
//     if data_length > MAX_DATA_LENGTH {
//         return Err(Error::new(
//             ErrorKind::InvalidData,
//             format!("the received data length {} is greater than the allowed maximum ({})", data_length, MAX_DATA_LENGTH))
//         );
//     }
//     let mut buf = vec![0; data_length];
//     input.read_exact(buf.as_mut_slice())?;
//     Ok((packet_id.0, buf))
// }

pub fn write_packet<T: Clientbound>(packet: &T, output: &mut ByteBuffer, threshold: i32) -> Result<()> {
    if threshold >= 0 {
        write_compressed_packet(packet, output, threshold)
    } else {
        write_uncompressed_packet(packet, output)
    }
}

pub fn write_uncompressed_packet<T: Clientbound>(packet: &T, output: &mut ByteBuffer) -> Result<()> {
    let id = VarInt(T::id());
    let mut length = id.size();

    let mut buf = ByteBuffer::new();
    packet.write_packet(&mut buf)?;
    length += buf.len();

    output.write_varint(&VarInt(length as i32))?;
    output.write_varint(&id)?;
    output.write_all(buf.as_slice())?;
    Ok(())
}

pub fn write_compressed_packet<T: Clientbound>(packet: &T, output: &mut ByteBuffer, threshold: i32) -> Result<()> {
    let id = VarInt(T::id());

    let mut uncompressed_data = ByteBuffer::new();
    uncompressed_data.write_varint(&id)?;
    packet.write_packet(&mut uncompressed_data)?;

    // The DataLength field: Length of uncompressed (Packet ID + Data) or 0
    let data_length = VarInt(uncompressed_data.len() as i32);

    if uncompressed_data.len() < threshold as usize {
        // + 1 for the length size being 0
        output.write_varint(&VarInt(data_length.0 + 1))?;
        output.write_varint(&VarInt(0))?;
        output.write_all(uncompressed_data.as_slice())?;
        return Ok(());
    }

    let (compressed_length, comrpessed_data) = {
        let mut writer = ZlibEncoder::new(Vec::new(), Compression::default());
        writer.write(uncompressed_data.as_slice())?;
        let result = writer.finish()?;
        (result.len(), result)
    };

    let packet_length = data_length.size() + compressed_length;
    output.write_varint(&VarInt(packet_length as i32))?;
    output.write_varint(&data_length)?;
    output.write_all(comrpessed_data.as_slice())?;
    Ok(())
}
