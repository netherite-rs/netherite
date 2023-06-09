extern crate core;
// Re-export as #[derive(Clientbound, Serverbound)].
#[cfg(feature = "protocol_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate protocol_derive;

use std::io::Result;

use bytebuffer::ByteBuffer;

#[cfg(feature = "protocol_derive")]
#[doc(hidden)]
pub use protocol_derive::*;

pub mod codec;
pub mod fields;

pub trait Serverbound {
    fn read_packet(input: &mut ByteBuffer) -> Self;
    fn id() -> i32;
}

pub trait Clientbound {
    fn write_packet(&self, output: &mut ByteBuffer) -> Result<()>;
    fn id() -> i32;
}

#[cfg(test)]
mod tests {
    use bytebuffer::ByteBuffer;


    use uuid::{uuid, Uuid};

    use crate::fields::generic::Json;
    use crate::fields::io_ext::{PacketReaderExt, PacketWriterExt};
    use crate::fields::numeric::{VarInt, VarLong};
    use crate::fields::position::Position;
    use crate::{Clientbound, Serverbound};
    use crate::codec::{read_compressed_packet, write_compressed_packet};

    #[test]
    fn test_varint() {
        let mut buffer = ByteBuffer::new();
        let value = 256;

        let _ = buffer.write_varint(&VarInt(value)).unwrap();
        let i = buffer.read_varint().unwrap().0;

        assert_eq!(i, value);
    }

    #[test]
    fn test_varlong() {
        let mut buffer = ByteBuffer::new();
        let value = 256;

        buffer.write_varlong(&VarLong(value)).unwrap();
        let i = buffer.read_varlong().unwrap().0;

        assert_eq!(i, value);
    }

    #[test]
    fn test_utf8() {
        let mut buffer = ByteBuffer::new();
        let value = String::from("Hello, world!");

        buffer.write_utf8(&value).unwrap();
        let read = buffer.read_utf8().unwrap();

        assert_eq!(read, value);
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
            Uuid::from_u64_pair(5, 4),
        ];
        let mut buffer = ByteBuffer::new();
        buffer.write_field(&arr).unwrap();

        let read = buffer.read_field::<[Uuid; 3]>().unwrap();
        assert_eq!(read, arr);
    }

    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    #[test]
    fn test_json() {
        let mut buffer = ByteBuffer::new();
        let person = Person {
            name: String::from("Jeff"),
            age: 20,
        };
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
        fn write_packet(&self, output: &mut ByteBuffer) -> std::io::Result<()> {
            output.write_varint(&self.protocol_version)?;
            output.write_utf8(&self.server_address)?;
            output.write_u16(self.server_port);
            output.write_varint(&self.next_state)?;
            Ok(())
        }

        fn id() -> i32 {
            0x00
        }
    }

    impl Serverbound for HandshakePacket {
        fn read_packet(input: &mut ByteBuffer) -> Self {
            HandshakePacket {
                protocol_version: input
                    .read_varint()
                    .expect("failed to read protocol_version"),
                server_address: input.read_utf8().expect("failed to read server_address"),
                server_port: input.read_u16().expect("failed to read server_port"),
                next_state: input.read_varint().expect("failed to read next_state"),
            }
        }

        fn id() -> i32 {
            0x00
        }
    }

    #[test]
    fn test_compression() {
        let mut buf = ByteBuffer::new();

        let packet = HandshakePacket {
            protocol_version: VarInt(655),
            server_address: "127.0.0.1".to_string(),
            server_port: 0,
            next_state: VarInt(1),
        };

        write_compressed_packet(&packet, &mut buf, 0).unwrap();

        let (id, data) = read_compressed_packet(&mut buf).unwrap();

        let mut buf = ByteBuffer::from(data);
        let read = HandshakePacket::read_packet(&mut buf);
        assert_eq!(id, 0x00);
        assert_eq!(read, packet);
    }
}
