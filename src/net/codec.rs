use std::borrow::Borrow;
use std::io::Result;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use bytebuffer::ByteBuffer;
use bytes::{Buf, BytesMut};
use cfb8::{Decryptor, Encryptor};
use cfb8::cipher::{AsyncStreamCipher, KeyIvInit};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use protocol::bound::Clientbound;
use protocol::compression::{read_packet, write_packet};
use protocol::fields::profile::GameProfile;

use crate::net::crypto::Crypto;
use crate::net::handler::PacketHandler;
use crate::Server;

/// A client codec is responsible for writing clientbound packets
/// to a TCP stream, with optional compression.
pub struct ClientCodec {
    threshold: Option<i32>,
    stage: ProtocolStage,
    conn: TcpStream,
    encryption: Option<Crypto>,
    player_name: Option<String>,
    profile: Option<GameProfile>
}

impl ClientCodec {
    pub async fn handle_handshake_packet(&mut self, id: i32, data: &mut ByteBuffer, server: &Server) {
        PacketHandler::handle_handshake_packet(self, id, data).await;
    }

    pub async fn handle_status_packet(&mut self, id: i32, data: &mut ByteBuffer, server: &Server) {
        PacketHandler::handle_status_packet(self, id, data, server).await;
    }

    pub async fn handle_login_packet(&mut self, id: i32, data: &mut ByteBuffer, server: &Server) {
        PacketHandler::handle_login_packet(self, id, data, server).await;
    }

    pub async fn handle_play_packet(&mut self, id: i32, data: &mut ByteBuffer, server: &Server) {
        PacketHandler::handle_play_packet(self, id, data, server).await;
    }
}

pub enum ProtocolStage {
    Handshake,
    Status,
    Login,
    Play,
}

impl ClientCodec {
    pub fn new(conn: TcpStream) -> Self {
        Self {
            threshold: None,
            stage: ProtocolStage::Handshake,
            conn,
            encryption: None,
            player_name: None,
            profile: None
        }
    }

    pub fn threshold(&self) -> Option<i32> {
        self.threshold
    }

    pub fn stage(&self) -> &ProtocolStage {
        &self.stage
    }

    pub fn conn(&self) -> &TcpStream {
        &self.conn
    }

    pub fn set_threshold(&mut self, threshold: i32) {
        self.threshold = Some(threshold);
    }

    pub async fn write_packet(&mut self, packet: &impl Clientbound) -> Result<()> {
        let mut buf: Vec<u8> = Vec::new();
        write_packet(packet, &mut buf, self.threshold.unwrap_or(-1))?;

        if self.encryption().is_some() {
            self.encrypt(buf.as_mut_slice());
        }

        self.conn.write_all(buf.as_slice()).await?;
        Ok(())
    }

    pub fn set_stage(&mut self, stage: ProtocolStage) {
        self.stage = stage;
    }

    pub async fn read_next_packet(&mut self) -> Result<Option<(i32, Vec<u8>)>> {
        let mut buf = BytesMut::with_capacity(1024);

        match self.conn.read_buf(&mut buf).await {
            Ok(n) if n == 0 => return Ok(None),
            Ok(n) => n,
            Err(e) => return Err(e)
        };

        if self.encryption().is_some() {
            self.decrypt(buf.as_mut());
        }

        read_packet(&mut buf.reader(), self.threshold.unwrap_or(-1))
            .map(|t| Some(t))
    }

    pub fn set_encryption(&mut self, shared_secret: [u8; 16]) {
        let key = &shared_secret.into();
        self.encryption = Some(Crypto {
            shared_secret,
            encryptor: Arc::new(Encryptor::new(key, key)),
            decryptor: Arc::new(Decryptor::new(key, key)),
        });
    }

    pub fn encryption(&self) -> &Option<Crypto> {
        &self.encryption
    }

    pub fn encrypt(&self, buf: &mut [u8]) {
        let encryptor = self.encryption.as_ref().unwrap().encryptor.clone();
        encryptor.deref().clone().encrypt(buf)
    }

    pub fn decrypt(&self, buf: &mut [u8]) {
        let decryptor = self.encryption.as_ref().unwrap().decryptor.clone();
        decryptor.deref().clone().decrypt(buf)
    }

    pub fn player_name(&self) -> &Option<String> {
        &self.player_name
    }

    pub fn set_player_name(&mut self, player_name: Option<String>) {
        self.player_name = player_name;
    }

    pub fn profile(&self) -> &Option<GameProfile> {
        &self.profile
    }
    
    pub fn set_profile(&mut self, profile: Option<GameProfile>) {
        self.profile = profile;
    }
}
