use std::fs::File;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::Mutex;

use bytebuffer::ByteBuffer;

use crate::encryption::server::ServerEncryption;
use crate::net::codec::{ClientCodec, ProtocolStage};
use crate::net::packet_handler::serverbound;
use crate::region::region::Region;
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
        serverbound::join_player(client, &self).await;

        let file = File::open("F:/Rust/netherite-rs/run/world/region/r.1.1.mca").unwrap();
        let region = Region::new(file, 1, 1).unwrap();
        let data = region.get_chunk_data(30, 30).unwrap().expect("Expected data");

    }

    pub async fn new(run_directory: PathBuf, properties: ServerProperties) -> Self {
        let address = properties.server_address();
        let listener = TcpListener::bind(address).await.expect(&*format!(
            "failed to bind to port {} because it is already in use.",
            properties.server().port()
        ));

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
                let client = ClientCodec::new(socket);
                let arc = Arc::new(Mutex::new(client));
                loop {
                    let mut client = arc.lock().await;
                    client.accept().await;
                    for (id, data) in client.incoming_packets().try_iter() {
                        let mut data = ByteBuffer::from_vec(data);
                        let stage = client.stage();
                        match stage {
                            ProtocolStage::Handshake => {
                                client.handle_handshake_packet(id, &mut data).await
                            }
                            ProtocolStage::Status => {
                                client
                                    .handle_status_packet(id, &mut data, server.deref())
                                    .await
                            }
                            ProtocolStage::Login => {
                                client
                                    .handle_login_packet(id, &mut data, server.deref())
                                    .await
                            }
                            ProtocolStage::Play => {
                                client
                                    .handle_play_packet(id, &mut data, server.deref())
                                    .await
                            }
                        }
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
