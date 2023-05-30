use anyhow::Result;
use bytes::BytesMut;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{
    encryption::client::ClientEncryption,
    protocol::compression::{read_compressed_packet_buf, read_uncompressed_packet},
};

use super::codec::ProtocolStage;

pub type Packet = (i32, Vec<u8>);

pub enum Stage {
    Handshake,
    Status,
    Login,
    Play,
}

pub struct Client {
    buf: BytesMut,

    // The client's connection socket
    socket: TcpStream,

    // Optional compression threshold
    compression_threshold: Option<i32>,

    // Optional encryption
    encryption: Option<ClientEncryption>,

    // The current stage the protocol is in
    stage: ProtocolStage,
}

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            buf: BytesMut::with_capacity(1024),
            socket,
            compression_threshold: None,
            encryption: None,
            stage: ProtocolStage::Handshake,
        }
    }

    pub fn socket(&self) -> &TcpStream {
        &self.socket
    }

    pub fn socket_mut(&mut self) -> &mut TcpStream {
        &mut self.socket
    }

    pub async fn receive_next_packet(&mut self) -> Result<Option<Packet>> {
        let mut buf = &mut self.buf;
        match self.socket.read_buf(&mut buf).await {
            Ok(0) => return Ok(None),
            Ok(n) => {}
            Err(e) => return Err(e.into()),
        }

        if let Some(encryption) = &mut self.encryption {
            encryption.decrypt(&mut buf)
        }

        let packet = if let Some(_) = self.compression_threshold {
            read_compressed_packet_buf(&mut buf)
        } else {
            read_uncompressed_packet(&mut buf)
        }?;

        Ok(Some(packet))
    }

    pub fn stage(&self) -> &ProtocolStage {
        &self.stage
    }

    pub fn set_stage(&mut self, stage: ProtocolStage) {
        self.stage = stage;
    }
}
