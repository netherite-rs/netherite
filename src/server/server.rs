use bytebuffer::ByteBuffer;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::Mutex;

use crate::config::ServerProperties;
use crate::net::client::{Client, Packet};

use crate::encryption::server::ServerEncryption;
use crate::net::codec::{ClientCodec, ProtocolStage};
use crate::net::packet_handler::serverbound;
use crate::packets::handshake::{self, Handshake};
use crate::protocol::Serverbound;

use std::collections::HashMap;

pub struct Server {
    properties: ServerProperties,
    encryption: ServerEncryption,
    run_directory: PathBuf,
    pub(crate) connected_clients: HashMap<SocketAddr, UnboundedSender<Packet>>,
}

impl Server {
    pub async fn finish_login(&self, client: &mut ClientCodec) {
        client.set_stage(ProtocolStage::Play);
        serverbound::join_player(client, &self).await;

        // let file = File::open("F:/Rust/netherite-rs/run/world/region/r.1.1.mca").unwrap();
        // let region = Region::new(file, 1, 1).unwrap();
        // let blob = region.get_chunk_data(30, 30).unwrap().expect("Expected data");
        //
        // let heightmap = blob.get("Heightmaps").expect("expected a heightmaps").clone();
        //
        // let sections = {
        //     let v = blob.get("sections").unwrap();
        //     let mut vec = Vec::new();
        //     v.write_field(&mut vec).unwrap();
        //     vec
        // };
        //
        // let block_entities = {
        //     let v = blob.get("block_entities").unwrap();
        //     let mut vec = Vec::new();
        //     v.write_field(&mut vec).unwrap();
        //     vec
        // };
        //
        //
        // let packet = ChunkPacket {
        //     chunk_x: 30,
        //     chunk_z: 30,
        //     heightmaps: heightmap,
        //     data: sections,
        //     block_entities,
        //     trust_edges: false,
        //     sky_light_mask: BitSet::new(),
        //     block_light_mask: BitSet::new(),
        //     empty_sky_light_mask: BitSet::new(),
        //     empty_block_light_mask: BitSet::new(),
        //     sky_light: [0; 2048],
        //     block_light: [0; 2048],
        // };
        //
        // client.write_packet(&packet).await.unwrap();
    }

    pub async fn start(run_directory: PathBuf, properties: ServerProperties) -> Self {
        let address = properties.server_address();
        let listener = TcpListener::bind(address).await.expect(&*format!(
            "failed to bind to port {} because it is already in use.",
            properties.server().port()
        ));

        let server = Self {
            properties,
            encryption: ServerEncryption::new(),
            run_directory,
            connected_clients: HashMap::new(),
        };

        let server = Arc::new(Mutex::new(server));

        loop {
            let (socket, addr) = listener.accept().await.unwrap();
            let server = Arc::clone(&server);
            tokio::spawn(async move {
                Server::handle_connection(server, socket, addr).await;
            });
        }
    }

    // pub async fn handle_connection(
    //     &self,
    //     server: Arc<Mutex<Server>>,
    //     socket: TcpStream,
    //     addr: SocketAddr,
    // ) {
    //     let (tx, rx) = unbounded_channel();
    //     let mut client = ClientCodec::new(tx, rx);
    //     loop {
    //         client.accept().await;
    //         for (id, data) in client.incoming_packets().try_iter() {
    //             let mut data = ByteBuffer::from_vec(data);
    //             let stage = client.stage();
    //             match stage {
    //                 ProtocolStage::Handshake => client.handle_handshake_packet(id, &mut data).await,
    //                 ProtocolStage::Status => {
    //                     client
    //                         .handle_status_packet(id, &mut data, server.deref())
    //                         .await
    //                 }
    //                 ProtocolStage::Login => {
    //                     client
    //                         .handle_login_packet(id, &mut data, server.deref())
    //                         .await
    //                 }
    //                 ProtocolStage::Play => {
    //                     client
    //                         .handle_play_packet(id, &mut data, server.deref())
    //                         .await
    //                 }
    //             }
    //         }
    //     }
    // }

    pub async fn handle_connection(
        server: Arc<Mutex<Server>>,
        socket: TcpStream,
        addr: SocketAddr,
    ) {
        let (tx, mut rx) = unbounded_channel();
        let server = Arc::clone(&server);
        {
            let mut server = server.lock().await;
            server.connected_clients.insert(addr, tx.clone());
        }
        let mut client = Client::new(socket);
        loop {
            select! {
                Some((id, data)) = rx.recv() => {
                    client.socket_mut().write(data.as_slice()).await.unwrap();
                }
                Ok(Some((id, data))) = client.receive_next_packet() => {
                    let mut data = ByteBuffer::from_vec(data);
                    match client.stage() {
                        ProtocolStage::Handshake => {
                            if id == 0x00 {
                                let packet: Handshake = Handshake::read_packet(&mut data);
                                let next_state = packet.next_state.0;
                                match next_state {
                                    handshake::STATUS => client.set_stage(ProtocolStage::Status),
                                    handshake::LOGIN => client.set_stage(ProtocolStage::Login),
                                    v => panic!(
                                        "invalid state in handshake packet. Expected 1 (status) or 2 (login), found {}",
                                        v
                                    ),
                                }
                            }
                        },
                        ProtocolStage::Status => {

                        },
                        ProtocolStage::Login => {

                        },
                        ProtocolStage::Play => {

                        },
                    }
                }
            }
        }
        //     loop {
        //         client.accept().await;
        //         for (id, data) in client.incoming_packets().try_iter() {
        //             let mut data = ByteBuffer::from_vec(data);
        //             let stage = client.stage();
        //             match stage {
        //                 ProtocolStage::Handshake => client.handle_handshake_packet(id, &mut data).await,
        //                 ProtocolStage::Status => {
        //                     client
        //                         .handle_status_packet(id, &mut data, server.deref())
        //                         .await
        //                 }
        //                 ProtocolStage::Login => {
        //                     client
        //                         .handle_login_packet(id, &mut data, server.deref())
        //                         .await
        //                 }
        //                 ProtocolStage::Play => {
        //                     client
        //                         .handle_play_packet(id, &mut data, server.deref())
        //                         .await
        //                 }
        //             }
        //         }
        //     }
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
