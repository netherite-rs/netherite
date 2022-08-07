use std::fmt::Debug;
use std::io::{empty, Result};

use bytebuffer::ByteBuffer;
use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use protocol::bound::Clientbound;
use protocol::compression::{read_packet, write_packet};

use crate::net::handler::PacketHandler;

/// A client codec is responsible for writing clientbound packets
/// to a TCP stream, with optional compression.
pub struct ClientCodec {
    threshold: Option<i32>,
    stage: ProtocolStage,
    conn: TcpStream,
}

impl ClientCodec {
    pub async fn handle_handshake_packet(&mut self, id: i32, data: &mut ByteBuffer) {
        PacketHandler::handle_handshake_packet(self, id, data).await;
    }

    pub async fn handle_status_packet(&mut self, id: i32, data: &mut ByteBuffer) {
        PacketHandler::handle_status_packet(self, id, data).await;
    }

    pub async fn handle_login_packet(&mut self, id: i32, data: &mut ByteBuffer) {
        PacketHandler::handle_login_packet(self, id, data).await;
    }

    pub async fn handle_play_packet(&mut self, id: i32, data: &mut ByteBuffer) {
        PacketHandler::handle_play_packet(self, id, data).await;
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
        read_packet(&mut buf.reader(), self.threshold.unwrap_or(-1))
            .map(|t| Some(t))
    }
}
