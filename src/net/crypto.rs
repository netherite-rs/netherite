use std::sync::Arc;

use aes::Aes128;
use cfb8::{Decryptor, Encryptor};

pub(crate) struct Crypto {
    pub(crate) shared_secret: [u8; 16],
    pub(crate) encryptor: Arc<Encryptor<Aes128>>,
    pub(crate) decryptor: Arc<Decryptor<Aes128>>,
}

impl Crypto {
    pub fn shared_secret(&self) -> [u8; 16] {
        self.shared_secret
    }
}