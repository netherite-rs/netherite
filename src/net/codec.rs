use std::io::Result;

use bytes::BytesMut;
use flume::{Iter, Receiver, Sender, TryIter};
use rsa::RsaPublicKey;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use bytebuffer::ByteBuffer;
use chat::text_component::TextComponent;
use protocol::Clientbound;
use protocol::compression::{read_packet, write_packet};
use protocol::fields::numeric::VarInt;
use protocol::fields::profile::GameProfile;

use crate::encryption::client::ClientEncryption;
use crate::net::handler::PacketHandler;
use crate::packets::login::{DisconnectLogin, SetCompressionPacket};
use crate::packets::play::DisconnectPlay;
use crate::Server;

/// A client codec is responsible for writing clientbound packets
/// to a TCP stream, with optional compression.
pub struct ClientCodec {
    threshold: Option<i32>,
    stage: ProtocolStage,
    conn: TcpStream,
    encryption: Option<ClientEncryption>,
    player_name: Option<String>,
    profile: Option<GameProfile>,
    public_key: Option<RsaPublicKey>,
    packets: (Sender<(i32, Vec<u8>)>, Receiver<(i32, Vec<u8>)>),
}

impl ClientCodec {
    pub async fn handle_handshake_packet(&mut self, id: i32, data: &mut ByteBuffer) {
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
            profile: None,
            public_key: None,
            packets: flume::unbounded(),
        }
    }

    pub fn stage(&self) -> &ProtocolStage {
        &self.stage
    }

    pub async fn enable_compression(&mut self, threshold: u32) {
        self.write_packet(&SetCompressionPacket {
            threshold: VarInt(threshold as i32)
        }).await.unwrap();
        self.threshold = Some(threshold as i32);
    }

    pub async fn disconnect(&mut self, reason: &str) {
        let reason = TextComponent::plain(reason);
        match self.stage {
            ProtocolStage::Login => self.write_packet(&DisconnectLogin { reason }).await.unwrap(),
            ProtocolStage::Play => self.write_packet(&DisconnectPlay { reason }).await.unwrap(),
            _ => {}
        }
        self.conn.shutdown().await.unwrap();
    }

    pub(crate) async fn close_connction(&mut self) {
        self.conn.shutdown().await.unwrap();
    }

    pub async fn write_packet<T: Clientbound>(&mut self, packet: &T) -> Result<()> {
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

    pub async fn accept(&mut self) {
        let mut buf = Vec::with_capacity(1024);
        match self.conn.read_buf(&mut buf).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => return,
        };

        if self.encryption().is_some() {
            self.decrypt(buf.as_mut());
        }

        let mut buf = ByteBuffer::from_vec(buf);
        while buf.has_data() {
            let threshold = self.threshold.unwrap_or(-1);
            let p = read_packet(&mut buf, threshold).map(|t| Some(t));
            if let Ok(Some(v)) = p {
                self.packets.0.send_async(v).await.unwrap();
            }
        }
    }

    pub fn incoming_packets(&self) -> Receiver<(i32, Vec<u8>)> {
        self.packets.1.clone()
    }

    pub fn enable_encryption(&mut self, shared_secret: [u8; 16]) {
        self.encryption = Some(ClientEncryption::new(shared_secret));
    }

    pub fn encrypt(&mut self, buf: &mut [u8]) {
        self.encryption.as_mut().unwrap().encrypt(buf);
    }

    pub fn decrypt(&mut self, buf: &mut [u8]) {
        self.encryption.as_mut().unwrap().decrypt(buf);
    }

    pub fn player_name(&self) -> &Option<String> {
        &self.player_name
    }

    pub fn set_player_name(&mut self, player_name: Option<String>) {
        self.player_name = player_name;
    }

    pub fn set_profile(&mut self, profile: Option<GameProfile>) {
        self.profile = profile;
    }

    pub fn public_key(&self) -> &Option<RsaPublicKey> {
        &self.public_key
    }

    pub fn set_public_key(&mut self, public_key: Option<RsaPublicKey>) {
        self.public_key = public_key;
    }

    pub fn encryption(&self) -> &Option<ClientEncryption> {
        &self.encryption
    }
}
