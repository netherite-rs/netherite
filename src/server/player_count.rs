use std::net::SocketAddr;
use std::sync::atomic::AtomicUsize;

use dashmap::DashMap;
use tokio::sync::mpsc::UnboundedSender;

pub struct Players {
    connected_clients: DashMap<SocketAddr, UnboundedSender<Vec<u8>>>,
}

impl Players {
    pub fn new() -> Self {
        Self { connected_clients: DashMap::new() }
    }

    pub fn connected_clients(&self) -> &DashMap<SocketAddr, UnboundedSender<Vec<u8>>> {
        &self.connected_clients
    }

    pub fn count(&self) -> usize {
        // Maybe consider keeping a track in a separate AtomicUsize?
        self.connected_clients.len()
    }

    pub fn player_joined(&self, addr: SocketAddr, packets: UnboundedSender<Vec<u8>>) {
        self.connected_clients.insert(addr, packets);
    }

    pub fn player_left(&self, addr: &SocketAddr) {
        self.connected_clients.remove(addr);
    }
}