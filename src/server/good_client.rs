use std::ptr::NonNull;
use flume::{Receiver, Sender};
use tokio::net::TcpStream;
use crate::encryption::client::ClientEncryption;
use crate::net::codec::ProtocolStage;
use crate::protocol::fields::position::Position;
use crate::protocol::fields::profile::GameProfile;

pub struct Player {
    client_net: ClientNet,
    profile: Option<GameProfile>,
    position: Option<Position>,
}

impl Player {
    pub fn new(client_net: ClientNet) -> Self {
        Self {
            client_net,
            profile: None,
            position: None
        }
    }
}

/// Represents a client. This client sends packets to the server
pub struct ClientNet {
    incoming_packets: Sender<(i32, Vec<u8>)>,
    encryption: Option<ClientEncryption>,
    stage: ProtocolStage,
    threshold: Option<i32>,
}

impl ClientNet {
    pub fn new(incoming_packets: Sender<(i32, Vec<u8>)>) -> Self {
        Self {
            incoming_packets,
            encryption: None,
            stage: ProtocolStage::Handshake,
            threshold: None,
        }
    }
}
