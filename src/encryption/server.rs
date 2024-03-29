use rand::Rng;
use rsa::pkcs8::EncodePublicKey;
use rsa::{errors::Result, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

pub struct ServerEncryption {
    private_key: RsaPrivateKey,
    pub(crate) public_key: RsaPublicKey,
    verify_token: Vec<u8>,
}

impl ServerEncryption {
    pub fn new() -> ServerEncryption {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 1024).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        let verify_token = rand::thread_rng().gen::<[u8; 4]>().to_vec();
        ServerEncryption {
            private_key,
            public_key,
            verify_token,
        }
    }

    pub fn encrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>> {
        let mut rng = rand::thread_rng();
        self.public_key.encrypt(
            &mut rng,
            Pkcs1v15Encrypt,
            data.as_slice(),
        )
    }

    pub fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>> {
        self.private_key.decrypt(Pkcs1v15Encrypt, data.as_slice())
    }

    pub fn public_key_encoded(&self) -> Vec<u8> {
        self.public_key
            .to_public_key_der()
            .unwrap()
            .as_ref()
            .to_vec()
    }

    pub fn verify_token(&self) -> Vec<u8> {
        self.verify_token.to_vec()
    }

    pub fn compare_verify_tokens(&self, token: &Vec<u8>) -> bool {
        &self.verify_token == token
    }
}
