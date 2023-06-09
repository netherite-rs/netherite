use std::error::Error;
use std::io::{Cursor, ErrorKind};

use anyhow::Result;
use auth::profile::GameProfile;
use bytebuffer::ByteBuffer;
use bytes::{Buf, BytesMut};
use futures::SinkExt;
use protocol::codec::{read_packet, write_packet};
use protocol::{Clientbound, Serverbound};
use rsa::RsaPublicKey;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpSocket;
use tokio::{io::AsyncReadExt, net::TcpStream};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::codec::{Framed, FramedWrite, Encoder};

use protocol::fields::numeric::VarInt;

use crate::encryption::client::ClientEncryption;
use crate::packets::login::SetCompressionPacket;
use crate::server::server::Server;

#[derive(Debug)]
pub enum ProtocolStage {
    Handshake,
    Status,
    Login,
    Play,
}

pub struct Client {
    // The client's connection socket
    socket: TcpStream,

    // The compression threshold
    compression_threshold: Option<i32>,

    // Encryption
    encryption: Option<ClientEncryption>,

    // The current stage the protocol is in
    stage: ProtocolStage,

    // The client's public key
    public_key: Option<RsaPublicKey>,

    // The player's name
    player_name: Option<String>,

    // The GameProfile of this client
    profile: Option<GameProfile>,

    // Packets sent to the client
    packets: UnboundedSender<Vec<u8>>,

    // A byte buffer used for sending and receiving data
    buf: BytesMut,
}

impl Client {
    pub fn new(socket: TcpStream, packets: UnboundedSender<Vec<u8>>) -> Self {
        Self {
            socket,
            compression_threshold: None,
            encryption: None,
            stage: ProtocolStage::Handshake,
            public_key: None,
            player_name: None,
            profile: None,
            packets,
            buf: BytesMut::with_capacity(1024),
        }
    }

    pub fn stage(&self) -> &ProtocolStage {
        &self.stage
    }

    pub fn set_stage(&mut self, stage: ProtocolStage) {
        self.stage = stage;
    }

    pub fn public_key(&self) -> &Option<RsaPublicKey> {
        &self.public_key
    }

    pub fn set_public_key(&mut self, public_key: RsaPublicKey) {
        self.public_key = Some(public_key);
    }

    pub fn compression_threshold(&self) -> Option<i32> {
        self.compression_threshold
    }

    pub fn encryption(&self) -> &Option<ClientEncryption> {
        &self.encryption
    }

    pub async fn enable_compression(&mut self, compression_threshold: u32) {
        let compression_threshold = compression_threshold as i32;
        self.send_packet(&SetCompressionPacket {
            threshold: VarInt(compression_threshold)
        }).await.unwrap();
        self.compression_threshold = Some(compression_threshold);
    }

    pub fn enable_encryption(&mut self, secret: [u8; 16]) {
        self.encryption = Some(ClientEncryption::new(secret));
    }

    pub async fn send_packet<T: Clientbound>(&mut self, packet: &T) -> Result<()> {
        let mut buf: Vec<u8> = Vec::new();
        write_packet(packet, &mut buf, self.compression_threshold.unwrap_or(-1))?;

        if let Some(encryptor) = self.encryption.as_mut() {
            encryptor.encrypt(buf.as_mut());
        }
        self.write_to_socket(buf.as_slice()).await;
        // self.packets.send(buf).unwrap();
        Ok(())
    }

    pub async fn close_connection(&mut self, server: &Server) {
        self.socket.shutdown().await.unwrap();

        server.players().player_left(self.socket.peer_addr().as_ref().unwrap());
    }

    pub fn profile(&self) -> &Option<GameProfile> {
        &self.profile
    }

    pub fn set_profile(&mut self, profile: GameProfile) {
        self.profile = Some(profile);
    }

    pub async fn read_next_packet(&mut self) -> std::io::Result<Option<(i32, Vec<u8>)>> {
        let mut buf = &mut self.buf;
        match self.socket.read_buf(&mut buf).await {
            Ok(n) if n == 0 => return Ok(None),
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        if let Some(encryptor) = self.encryption.as_mut() {
            encryptor.decrypt(buf.as_mut());
        }
        let mut cursor = Cursor::new(&buf[..]);
        let v = read_packet(
            &mut cursor,
            self.compression_threshold.unwrap_or(-1),
        ).map(|t| Some(t));

        buf.advance(cursor.position() as usize);
        return v;
    }

    pub(crate) async fn write_to_socket(&mut self, data: &[u8]) {
        self.socket.write_all(data).await.unwrap();
    }

    pub async fn parse_next_packet<T: Serverbound>(&mut self) -> std::io::Result<Option<T>> {
        let v = self.read_next_packet().await?;
        if v.is_none() {
            return Ok(None);
        }
        let (id, data) = v.unwrap();
        if id != T::id() {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                format!("Expected packet 0x{:02X}, found 0x{:02X}", T::id(), id),
            ));
        }
        let mut reader = ByteBuffer::from(&data[..]);
        let packet = T::read_packet(&mut reader);

        Ok(Some(packet))
    }

    pub fn player_name(&self) -> &Option<String> {
        &self.player_name
    }

    pub fn set_player_name(&mut self, player_name: String) {
        self.player_name = Some(player_name);
    }
}
