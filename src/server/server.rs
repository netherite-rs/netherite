use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::net::TcpListener;

use bytebuffer::ByteBuffer;

use crate::encryption::server::ServerEncryption;
use crate::net::codec::{ClientCodec, ProtocolStage};
use crate::ServerProperties;

pub struct Server {
    listener: TcpListener,
    properties: ServerProperties,
    encryption: ServerEncryption,
    run_directory: PathBuf,
}

impl Server {
    
    pub async fn finish_login(&self, client: &mut ClientCodec) {
        client.set_stage(ProtocolStage::Play);
    }
    
    pub async fn new(run_directory: PathBuf, properties: ServerProperties) -> Self {
        let address = format!("{}:{}", properties.server().address(), properties.server().port());
        let listener = TcpListener::bind(address)
            .await
            .expect(&*format!("failed to bind to port {} because it is already in use.", properties.server().port()));

        Self {
            listener,
            properties,
            encryption: ServerEncryption::new(),
            run_directory,
        }
    }

    pub async fn start(server: Arc<Self>) {
        loop {
            let server = server.clone();
            let (socket, _) = server.listener.accept().await.unwrap();
            tokio::spawn(async move {
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
                    let mut data = ByteBuffer::from_vec(data);

                    let stage = client_codec.stage();
                    match stage {
                        ProtocolStage::Handshake => client_codec.handle_handshake_packet(id, &mut data).await,
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

    pub fn run_directory(&self) -> &PathBuf {
        &self.run_directory
    }

    pub fn encryption(&self) -> &ServerEncryption {
        &self.encryption
    }
}
