use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use bevy::{app::App, MinimalPlugins, log::LogPlugin, prelude::Events, DefaultPlugins};
use bevy::app::{PluginGroup, ScheduleRunnerPlugin};
use bevy::prelude::{apply_system_buffers, Commands, Deref, Res, ResMut, Resource};
use bytebuffer::ByteBuffer;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::sync::oneshot::Sender;
use crate::client::client::{Client, ProtocolStage};
use crate::config::ServerProperties;
use crate::encryption::server::ServerEncryption;
use crate::packets::handler::handshake::receive_handshake;
use crate::packets::handler::login::receive_login;
use crate::packets::handler::play::receive_play;
use crate::packets::handler::status::receive_status;
use crate::server::server::Server;
use bevy::prelude::IntoSystemConfigs;

pub fn setup_netherite_app() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(50))),
        )
        .add_plugin(LogPlugin::default())
        .add_event::<TokioEvent>()
        .add_startup_systems((setup_tokio, apply_system_buffers, setup_listener).chain())
        .run();
}

fn keep_alive(server: Res<NetheriteServer>) {
    println!("{:?}", server.run_directory());
}

#[derive(Debug)]
enum TokioEvent {
    NewConnection,
    LostConnection,
}

#[derive(Resource, Deref)]
struct TokioRuntime(Runtime);

#[derive(Resource, Deref)]
struct TokioEventStream<T>(UnboundedReceiver<T>);

#[derive(Resource, Deref)]
struct NetheriteServer(Arc<Server>);

/// Create a resource that contains the Tokio [Runtime]
fn setup_tokio(mut commands: Commands) {
    let runtime = Runtime::new().expect("failed to start tokio runtime");
    commands.insert_resource(TokioRuntime(runtime))
}

fn setup_listener(commands: Commands, runtime: Res<TokioRuntime>) {
    runtime.spawn(async move {
        let directory = Path::new("run").to_path_buf();
        let properties = ServerProperties::from_file(&directory.join("server.toml"));
        start(commands, directory, properties).await;
    });
}

// fn event_receiver(mut stream: ResMut<TokioEventStream<TokioEvent>>) {
//     while let Ok(event) = stream.try_recv() {
//         match event {
//             TokioEvent::NewConnection => {
//
//             }
//             TokioEvent::LostConnection => {
//
//             }
//         }
//     }
// }

async fn start(mut commands: Commands, run_directory: PathBuf, properties: ServerProperties) {
    let address = properties.address();
    let listener = TcpListener::bind(address).await.expect(&*format!(
        "failed to bind to port {} because it is already in use.",
        properties.server().port()
    ));

    let server = Server::new(properties, run_directory);

    let server = Arc::new(server);

    commands.insert_resource(NetheriteServer(server.clone()));

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let server = Arc::clone(&server);
        tokio::spawn(async move {
            handle_client(server, socket, addr).await;
        });
    }
}

async fn handle_client(server: Arc<Server>, socket: TcpStream, addr: SocketAddr) {
    let (tx, mut rx) = unbounded_channel();
    {
        let server = Arc::clone(&server);
        server.players().player_joined(addr, tx.clone());
    }
    let mut client = Client::new(socket, tx.clone());
    loop {
        select! {
                Some(packet) = rx.recv() => {
                    client.write_to_socket(packet.as_ref()).await;
                }
                result = client.read_next_packet() => match result {
                    Ok(Some((id,  data))) => {
                        let mut data = ByteBuffer::from(data);
                        match client.stage() {
                            ProtocolStage::Handshake => {
                                receive_handshake(id, &mut data, &mut client, Arc::clone(&server)).await;
                            },
                            ProtocolStage::Status => {
                                receive_status(id, &mut data, &mut client, Arc::clone(&server)).await;
                            },
                            ProtocolStage::Login => {
                                receive_login(id, &mut data, &mut client, Arc::clone(&server)).await;
                            },
                            ProtocolStage::Play => {
                                receive_play(id, &mut data, &mut client, Arc::clone(&server)).await;
                            },
                        }
                    },
                    Ok(None) => {},
                    Err(e) => {
                        // eprintln!("error: {:?}", e);
                    },
                },
            }
        // Client disconnected. Exit the loop
    }
}