extern crate core;

pub mod packet_io;
pub mod bound;
pub mod fields;
pub mod compression;

#[cfg(test)]
mod tests {
    use bytebuffer::ByteBuffer;
    use uuid::{uuid, Uuid};

    use crate::bound::{Clientbound, Serverbound};

    use crate::compression::{read_compressed_packet, write_compressed_packet};
    use crate::fields::generic::Json;
    use crate::fields::numeric::{VarInt, VarLong};
    use crate::fields::position::Position;
    use crate::packet_io::PacketReaderExt;
    use crate::packet_io::PacketWriterExt;

    #[test]
    fn test_varint() {
        let mut buffer = ByteBuffer::new();
        let value = 256;

        let size = buffer.write_varint(&VarInt(value)).unwrap();
        let i = buffer.read_varint().unwrap().0;

        assert_eq!(size, 2);
        assert_eq!(i, value);
    }

    #[test]
    fn test_varlong() {
        let mut buffer = ByteBuffer::new();
        let value = 256;

        let size = buffer.write_varlong(&VarLong(value)).unwrap();
        let i = buffer.read_varlong().unwrap().0;

        assert_eq!(size, 2);
        assert_eq!(i, value);
    }

    #[test]
    fn test_utf8() {
        let mut buffer = ByteBuffer::new();
        let value = String::from("Hello, world!");

        let size = buffer.write_utf8(&value).unwrap();
        let read = buffer.read_utf8().unwrap();

        assert_eq!(read, value);
        assert_eq!(size, value.len() + 1); // size + 1 byte for the length
    }

    #[test]
    fn test_uuid() {
        let mut buffer = ByteBuffer::new();
        let id = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

        buffer.write_field(&id).unwrap();
        let read = buffer.read_field::<Uuid>().unwrap();

        assert_eq!(read, id);
    }

    #[test]
    fn test_position() {
        let mut buffer = ByteBuffer::new();
        let position = Position::new(-4920, -20, 40);

        buffer.write_field(&position).unwrap();
        let read = buffer.read_field::<Position>().unwrap();
        assert_eq!(read, position);
    }

    #[test]
    fn test_byte_arrays() {
        let arr = [20_u8; 20];
        let mut buffer = ByteBuffer::new();
        buffer.write_field(&arr).unwrap();
        let x = buffer.read_field::<[u8; 20]>().unwrap();
        assert_eq!(x, arr);
    }

    #[test]
    fn test_array() {
        let arr = [
            Uuid::from_u64_pair(1, 2),
            Uuid::from_u64_pair(6, 3),
            Uuid::from_u64_pair(5, 4)
        ];
        let mut buffer = ByteBuffer::new();
        buffer.write_field(&arr).unwrap();

        let read = buffer.read_field::<[Uuid; 3]>().unwrap();
        assert_eq!(read, arr);
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(PartialEq, Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    #[test]
    fn test_json() {
        let mut buffer = ByteBuffer::new();
        let person = Person { name: String::from("Jeff"), age: 20 };
        let json = Json(person);
        buffer.write_field(&json).unwrap();
        let read = buffer.read_field::<Json<Person>>().unwrap();
        assert_eq!(read, json)
    }

    #[test]
    fn test_option() {
        let mut buffer = ByteBuffer::new();
        let option: Option<Vec<i32>> = None;
        buffer.write_field(&option).unwrap();
        let read = buffer.read_field::<Option<Vec<i32>>>().unwrap();
        assert_eq!(read, option)
    }

    #[derive(PartialEq, Debug)]
    pub struct HandshakePacket {
        pub protocol_version: VarInt,
        pub server_address: String,
        pub server_port: u16,
        pub next_state: VarInt,
    }

    impl Clientbound for HandshakePacket {
        fn write_packet(&self, output: &mut impl std::io::Write) -> std::io::Result<usize> {
            let mut size: usize = 0;
            size += PacketWriterExt::write_varint(output, &self.protocol_version)?;
            size += PacketWriterExt::write_utf8(output, &self.server_address)?;
            byteorder::WriteBytesExt::write_u16::<byteorder::BigEndian>(output, self.server_port)?;
            size += core::mem::size_of::<u16>();
            size += PacketWriterExt::write_varint(output, &self.next_state)?;
            Ok(size)
        }

        fn id(&self) -> i32 {
            0x00
        }
    }

    impl Serverbound for HandshakePacket {
        fn read_packet(input: &mut impl std::io::Read) -> HandshakePacket {
            HandshakePacket {
                protocol_version: PacketReaderExt::read_varint(input)
                    .expect("failed to read protocol_version"),
                server_address: PacketReaderExt::read_utf8(input)
                    .expect("failed to read server_address"),
                server_port: byteorder::ReadBytesExt::read_u16::<byteorder::BigEndian>(input)
                    .expect("failed to read server_port"),
                next_state: PacketReaderExt::read_varint(input)
                    .expect("failed to read next_state"),
            }
        }

        fn id(&self) -> i32 {
            0i32
        }
    }

    #[test]
    fn test_compression() {
        let packet = HandshakePacket {
            protocol_version: VarInt(655),
            server_address: "127.0.0.1".to_string(),
            server_port: 0,
            next_state: VarInt(1),
        };
        let mut buffer = ByteBuffer::new();

        write_compressed_packet(&packet, &mut buffer, 1).unwrap();

        let (id, data) = read_compressed_packet(&mut buffer).unwrap();

        let mut buffer = ByteBuffer::from_bytes(data.as_slice());
        let read = HandshakePacket::read_packet(&mut buffer);

        assert_eq!(id, Clientbound::id(&packet));
        assert_eq!(read, packet);
    }
}
