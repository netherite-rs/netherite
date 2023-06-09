use std::sync::Arc;

use bytebuffer::ByteBuffer;

use protocol::{Clientbound, Serverbound};

use crate::client::client::{Client, ProtocolStage};
use crate::packets::handshake;
use crate::packets::handshake::Handshake;
use crate::server::server::Server;

pub async fn receive_handshake(id: i32, data: &mut ByteBuffer, client: &mut Client, _server: Arc<Server>) {
    if id == Handshake::id() {
        let packet = Handshake::read_packet(data);
        handle_handshake(packet, client).await;
    }
}

async fn handle_handshake(packet: Handshake, client: &mut Client) {
    let next_state = packet.next_state.0;
    match next_state {
        handshake::STATUS => client.set_stage(ProtocolStage::Status),
        handshake::LOGIN => client.set_stage(ProtocolStage::Login),
        v => panic!("invalid state in handshake packet. expected 1 (status) or 2 (login), found {}", v),
    }
}