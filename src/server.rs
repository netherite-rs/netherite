use std::ops::{Deref, Receiver};
use std::sync::Arc;

use bytebuffer::ByteBuffer;
use tokio::net::TcpListener;

use net::codec::ClientCodec;
use crate::config::ServerProperties;
use crate::encryption::encryption::EncryptionHandler;

use crate::net;
use crate::net::codec::ProtocolStage;

pub struct Server {
    listener: TcpListener,
    properties: ServerProperties,
    encryption: EncryptionHandler,
    new_players: Receiver<Player>
}

impl Server {
    pub async fn new(properties: ServerProperties) -> Server {
        let address = format!("{}:{}", properties.server().address(), properties.server().port());
        let listener = TcpListener::bind(address).await
            .expect(&*format!("failed to bind to port {} because it is already in use.", properties.server().port()));
        Server {
            listener,
            properties,
            encryption: EncryptionHandler::new(),
        }
    }

    pub async fn start(server: Arc<Self>) {
        loop {
            let server = server.clone();
            let (socket, _) = server.listener.accept().await.unwrap(); // <------- code never reaches this
            tokio::spawn(async move {
                // let server = server; // move inside
                let mut client_codec = ClientCodec::new(socket);
                loop {
                    let rs = client_codec.read_next_packet().await;
                    if rs.is_err() {
                        eprintln!("failed to read packet: {}", rs.err().unwrap());
                        return;
                    }
                    let read = rs.unwrap();
                    if read.is_none() {
                        return;
                    }
                    let (id, data) = read.unwrap();
                    let mut data = ByteBuffer::from_bytes(data.as_slice());

                    let stage = client_codec.stage();
                    match stage {
                        ProtocolStage::Handshake => client_codec.handle_handshake_packet(id, &mut data, server.deref()).await,
                        ProtocolStage::Status => client_codec.handle_status_packet(id, &mut data, server.deref()).await,
                        ProtocolStage::Login => client_codec.handle_login_packet(id, &mut data, server.deref()).await,
                        ProtocolStage::Play => client_codec.handle_play_packet(id, &mut data, server.deref()).await,
                    }
                }
            });
        }
    }

    pub fn properties(&self) -> &ServerProperties {
        &self.properties
    }

    pub fn encryption(&self) -> &EncryptionHandler {
        &self.encryption
    }
}
