use std::sync::Arc;
use bytebuffer::ByteBuffer;
use auth::profile::GameProfile;
use protocol::Serverbound;
use crate::client::client::Client;
use crate::packets::login::{EncryptionRequest, EncryptionResponse, LoginPluginResponse, LoginStart, LoginSuccess};
use crate::server;
use crate::server::server::Server;

mod encryption;

pub async fn receive_login(id: i32, data: &mut ByteBuffer, client: &mut Client, server: Arc<Server>) {
    if id == LoginStart::id() {
        let packet = LoginStart::read_packet(data);
        handle_login_start(packet, client, server).await;
    } else if id == EncryptionResponse::id() {
        let packet = EncryptionResponse::read_packet(data);
        encryption::handle_encryption_response(packet, client, server).await;
    } else if id == LoginPluginResponse::id() {

    }
}

async fn handle_login_start(
    packet: LoginStart,
    client: &mut Client,
    server: Arc<Server>,
) {
    let name = packet.name;
    let public_key = server.encryption().public_key.clone();
    // let public_key = RsaPublicKey::from_public_key_der(public_key.as_slice()).unwrap();
    client.set_public_key(public_key);
    if *server.properties().server().online_mode() {
        client.send_packet(&EncryptionRequest {
            server_id: "".to_string(),
            public_key: server.encryption().public_key_encoded(),
            verify_token: server.encryption().verify_token(),
        }).await.unwrap();
    } else {
        client.enable_compression(*server.properties().server().compression_threshold())
            .await;
        let success = LoginSuccess {
            profile: GameProfile::offline(&name),
        };
        client.send_packet(&success).await.unwrap();
        client.set_profile(success.profile);
        Server::finish_login(server, client).await;
    }
    client.set_player_name(name);
}
