use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

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
            let (mut socket, _) = self.listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                loop {
                    socket.read(&mut buf).await.expect("failed to read from socket");
                    // let n = match  {
                    //     Ok(0) => return,
                    //     Ok(n) => n,
                    //     Err(e) => {
                    //         eprintln!("failed to read from socket; err = {:?}", e);
                    //         return;
                    //     }
                    // };
                    println!("request: {:?}", buf)
                    // if let Err(e) = socket.write_all(&buf[0..n]).await {
                    //     eprintln!("failed to write to socket; err = {:?}", e);
                    //     return;
                    // }
                }
            });
        }
    }
}
