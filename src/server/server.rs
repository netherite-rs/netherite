use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, AtomicUsize};
use bevy::app::App;

use bytebuffer::ByteBuffer;
use bytes::Buf;
use futures::SinkExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::{Mutex, RwLock};
use tokio_stream::StreamExt;
use app::setup_netherite_app;

use protocol::Serverbound;

use crate::client::client::{Client, ProtocolStage};
use crate::config::ServerProperties;
use crate::encryption::server::ServerEncryption;
use crate::{app, packets};
use crate::packets::handler;
use crate::packets::handler::handshake::receive_handshake;
use crate::packets::handler::login::receive_login;
use crate::packets::handler::play::{join_player, receive_play};
use crate::packets::handler::status::receive_status;
use crate::packets::handshake::{self, Handshake};
use crate::server::player_count::Players;
use crate::world::worlds::Worlds;

pub(crate) static ENTITY_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Server {
    properties: ServerProperties,
    encryption: ServerEncryption,
    run_directory: PathBuf,
    players: Players,
    worlds: Worlds,
}

impl Server {
    pub fn new(
        properties: ServerProperties,
        run_directory: PathBuf,
    ) -> Self {
        Self {
            properties,
            encryption: ServerEncryption::new(),
            run_directory,
            players: Players::new(),
            worlds: Worlds::new(),
        }
    }

    pub async fn finish_login(server: Arc<Self>, client: &mut Client) {
        client.set_stage(ProtocolStage::Play);
        join_player(client, server).await;
    }

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
    // }

    // pub async fn start(run_directory: PathBuf, properties: ServerProperties) -> Self {
    //     let address = properties.address();
    //     let listener = TcpListener::bind(address).await.expect(&*format!(
    //         "failed to bind to port {} because it is already in use.",
    //         properties.server().port()
    //     ));
    //
    //     let server = Server::new(properties, run_directory);
    //
    //     let server = Arc::new(server);
    //
    //     loop {
    //         let (socket, addr) = listener.accept().await.unwrap();
    //         let server = Arc::clone(&server);
    //         tokio::spawn(async move {
    //             Server::handle_connection(server, socket, addr).await;
    //         });
    //     }
    // }

    // async fn handle_connection(server: Arc<Server>, socket: TcpStream, addr: SocketAddr) {
    //     let (tx, mut rx) = unbounded_channel();
    //     {
    //         let server = Arc::clone(&server);
    //         server.players.player_joined(addr, tx.clone());
    //     }
    //     let mut client = Client::new(socket, tx.clone());
    //     loop {
    //         select! {
    //             Some(packet) = rx.recv() => {
    //                 client.write_to_socket(packet.as_ref()).await;
    //             }
    //             result = client.read_next_packet() => match result {
    //                 Ok(Some((id,  data))) => {
    //                     let mut data = ByteBuffer::from(data);
    //                     match client.stage() {
    //                         ProtocolStage::Handshake => {
    //                             receive_handshake(id, &mut data, &mut client, Arc::clone(&server)).await;
    //                         },
    //                         ProtocolStage::Status => {
    //                             receive_status(id, &mut data, &mut client, Arc::clone(&server)).await;
    //                         },
    //                         ProtocolStage::Login => {
    //                             receive_login(id, &mut data, &mut client, Arc::clone(&server)).await;
    //                         },
    //                         ProtocolStage::Play => {
    //                             receive_play(id, &mut data, &mut client, Arc::clone(&server)).await;
    //                         },
    //                     }
    //                 },
    //                 Ok(None) => {},
    //                 Err(e) => {
    //                     // eprintln!("error: {:?}", e);
    //                 },
    //             },
    //         }
    //         // Client disconnected. Exit the loop
    //     }
    // }

    pub fn properties(&self) -> &ServerProperties {
        &self.properties
    }

    pub fn run_directory(&self) -> &PathBuf {
        &self.run_directory
    }

    pub fn encryption(&self) -> &ServerEncryption {
        &self.encryption
    }

    pub fn players(&self) -> &Players {
        &self.players
    }
}
