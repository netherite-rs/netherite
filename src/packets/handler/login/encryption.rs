use std::sync::Arc;
use crate::client::client::Client;
use crate::packets::login::{EncryptionResponse, LoginSuccess};
use crate::server::server::Server;

pub(crate) async fn handle_encryption_response(
    packet: EncryptionResponse,
    client: &mut Client,
    server: Arc<Server>,
) {
    let shared_secret = server.encryption().decrypt(&packet.shared_secret).unwrap();
    let verify_token = packet.verify_token;
    let verify_token = server.encryption().decrypt(&verify_token).unwrap();
    if !server.encryption().compare_verify_tokens(&verify_token) {
        panic!("verify tokens do not match.");
    }

    client.enable_encryption(shared_secret.try_into().unwrap());
    let hex = auth::generate_server_hash(
        &server.encryption().public_key_encoded(),
        &client.encryption().as_ref().unwrap().secret(),
    );
    let profile = auth::authenticate(
        client.player_name().as_ref().unwrap(),
        &hex,
        server.properties().server().address(),
    ).await;

    client.enable_compression(*server.properties().server().compression_threshold())
        .await;

    let success = LoginSuccess { profile };
    client.send_packet(&success).await.unwrap();
    client.set_profile(success.profile);

    Server::finish_login(server, client).await;
}
