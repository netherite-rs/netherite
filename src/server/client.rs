use tokio::net::{TcpSocket, TcpStream};

pub struct Client {
    stream: TcpStream,
    threshold: Option<i32>,
}