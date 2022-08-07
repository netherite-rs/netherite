use bytebuffer::ByteBuffer;
use tokio::net::TcpListener;

use net::codec::ClientCodec;

use crate::net;
use crate::net::codec::ProtocolStage;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(port: i32) -> Server {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).await
            .expect(&*format!("failed to bind to port {} because it is already in use.", port));
        Server { listener }
    }

    pub async fn start(&self) {
        loop {
            let (socket, _) = self.listener.accept().await.unwrap();
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
                    let mut data = ByteBuffer::from_bytes(data.as_slice());

                    match *client_codec.stage() {
                        ProtocolStage::Handshake => client_codec.handle_handshake_packet(id, &mut data).await,
                        ProtocolStage::Status => client_codec.handle_status_packet(id, &mut data).await,
                        ProtocolStage::Login => client_codec.handle_login_packet(id, &mut data).await,
                        ProtocolStage::Play => client_codec.handle_play_packet(id, &mut data).await,
                    }
                }
            });
        }
    }
}
