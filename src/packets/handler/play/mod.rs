use std::sync::Arc;
use std::sync::atomic::Ordering::SeqCst;
use bytebuffer::ByteBuffer;
use enum_utils::OrdinalEnum;
use nbt::Blob;
use protocol::fields::key::Key;
use protocol::fields::numeric::VarInt;
use protocol::fields::position::Position;
use crate::client::client::{Client, ProtocolStage};
use crate::packets::login::LoginPlay;
use crate::server::server;
use crate::server::server::Server;

pub async fn receive_play(id: i32, data: &mut ByteBuffer, client: &mut Client, server: Arc<Server>) {}

pub(crate) async fn join_player(client: &mut Client,
                                server: Arc<Server>) {
    let mut registry_codec = ByteBuffer::from_bytes(
        include_bytes!("../../../../codecs/1.19.4/registry-1.19.4.nbt")
    );
    let registry_codec = Blob::from_reader(&mut registry_codec).unwrap();
    let play = LoginPlay {
        entity_id: server::ENTITY_ID_COUNTER.fetch_add(1, SeqCst) as i32,
        is_hardcore: false,
        game_mode: server.properties().game().default_gamemode().ordinal() as u8,
        previous_gamemode: -1,
        dimension_names: vec![
            Key::new("dimension", "world"),
            Key::new("dimension", "world_nether"),
            Key::new("dimension", "world_the_end"),
        ],
        dimesion_codec: registry_codec,
        dimension_type: Key::minecraft("overworld"),
        dimension_name: Key::new("dimension", "world"),
        hashed_seed: -20,
        max_players: VarInt(*server.properties().status().max_players() as i32),
        view_distance: VarInt(*server.properties().game().view_distance() as i32),
        simulation_distance: VarInt(*server.properties().game().simulation_disance() as i32),
        reduced_debug_info: *server.properties().game().reduce_debug_info(),
        enable_respawn_screen: *server.properties().game().enable_respawn_screen(),
        is_debug: false,
        is_flat: false,
        has_death_location: true,
        death_dimension_name: Some(Key::new("dimension", "world")),
        death_location: Some(Position {
            x: 10,
            y: 10,
            z: 10,
        }),
    };
    print!("Still !Finished!");
    client.send_packet(&play).await.unwrap();
    print!("Finished!");
}