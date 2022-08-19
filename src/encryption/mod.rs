pub mod encryption;


#[cfg(test)]
mod tests {
    use aes::Aes128;
    use aes::cipher::KeyIvInit;
    use bytebuffer::ByteBuffer;
    use bytes::Buf;
    use cfb8::cipher::AsyncStreamCipher;
    use cfb8::{Decryptor, Encryptor};
    use nbt::Blob;
    use rand::{Rng, thread_rng};

    use protocol::compression::{read_compressed_packet, write_compressed_packet};
    use protocol::fields::identifier::Key;
    use protocol::fields::numeric::VarInt;
    use protocol::fields::position::Position;
    use protocol::bound::{Serverbound};
    use crate::encryption::encryption::EncryptionHandler;
    use crate::packets::login::LoginPlay;

    type EncryptAes128 = Encryptor<Aes128>;
    type DecryptAes128 = Decryptor<Aes128>;

    #[test]
    fn encrypt() {
        let data = b"i am an exceptionally good boy".to_vec();
        let encryption_handler = EncryptionHandler::new();
        let encrypted = encryption_handler.encrypt(&data).unwrap();

        let decrypted = encryption_handler.decrypt(&encrypted).unwrap();
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_encrypted_packets() {
        let mut registry_codec = ByteBuffer::from_bytes(include_bytes!("../../codecs/registry_codec.nbt"));
        let registry_codec = Blob::from_reader(&mut registry_codec).unwrap();

        let key = &thread_rng().gen::<[u8; 16]>().into();
        let mut buf = Vec::new();
        let packet = LoginPlay {
            entity_id: 0,
            is_hardcore: false,
            game_mode: 0,
            previous_gamemode: -1,
            dimension_names: vec![
                Key::new("dimension", "world"),
                Key::new("dimension", "world_nether"),
                Key::new("dimension", "world_the_end"),
            ],
            dimesion_codec: registry_codec,
            dimension_type: Key::minecraft("overworld"),
            dimension_name: Key::new("dimension", "world"),
            hashed_seed: -20,
            max_players: VarInt(5),
            view_distance: VarInt(12),
            simulation_distance: VarInt(12),
            reduced_debug_info: false,
            enable_respawn_screen: false,
            is_debug: false,
            is_flat: false,
            has_death_location: true,
            death_dimension_name: Some(Key::new("dimension", "world")),
            death_location: Some(Position {
                x: 10,
                y: 10,
                z: 10,
            }),
        };
        write_compressed_packet(&packet, &mut buf, 256).unwrap();
        let original = buf.to_vec();

        EncryptAes128::new(key, key).encrypt(&mut buf);
        DecryptAes128::new(key, key).decrypt(&mut buf);
        let read_packet = read_compressed_packet(&mut buf.reader()).unwrap().1;
        let read = LoginPlay::read_packet(&mut read_packet.reader());
        println!("read  : {:?}", read);
        println!("actual: {:?}", packet);
        assert_eq!(buf.to_vec(), original);
    }
}