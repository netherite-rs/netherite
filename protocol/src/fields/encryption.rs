use aes::Aes128;
use cfb8::{Decryptor, Encryptor};
use cfb8::cipher::{AsyncStreamCipher, KeyIvInit};

pub struct ClientEncryption {
    secret: [u8; 16],
    encoder_iv: [u8; 16],
    decoder_iv: [u8; 16],
}

impl ClientEncryption {
    pub fn new(secret: [u8; 16]) -> Self {
        ClientEncryption {
            secret,
            encoder_iv: secret,
            decoder_iv: secret,
        }
    }

    pub fn encrypt(&mut self, buffer: &mut [u8]) {
        let len = buffer.len();
        let count = len.min(16);
        let encryptor: Encryptor<Aes128> = Encryptor::new_from_slices(&self.secret, &self.encoder_iv)
            .expect("invalid key size");
        encryptor.encrypt(buffer);
        self.encoder_iv.rotate_left(count);
        self.encoder_iv[16 - count..].copy_from_slice(&buffer[len - count..]);
    }

    pub fn decrypt(&mut self, buffer: &mut [u8]) {
        let len = buffer.len();
        let count = len.min(16);
        let ciphertext = buffer[len - count..].to_vec();
        let decryptor: Decryptor<Aes128> = Decryptor::new_from_slices(&self.secret, &self.decoder_iv)
            .expect("invalid key size");
        decryptor.decrypt(buffer);
        self.decoder_iv.rotate_left(count);
        self.decoder_iv[16 - count..].copy_from_slice(&ciphertext);
    }

    pub fn secret(&self) -> [u8; 16] {
        self.secret
    }
}