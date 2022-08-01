pub mod packet_io;
pub mod bound;
pub mod fields;

#[cfg(test)]
mod tests {
    use bytebuffer::ByteBuffer;
    use uuid::{uuid, Uuid};
    use crate::fields::{Position, VarInt, VarLong};

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

        buffer.write_position(&position).unwrap();
        let read = buffer.read_position().unwrap();
        // assert_eq!(read, position);
    }

    #[test]
    fn test_byte_arrays() {
        let arr = [20_u8; 20];
        let mut buffer = ByteBuffer::new();
        buffer.write_field(&arr).unwrap();
        println!("{:?}", buffer.to_bytes());
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
}
