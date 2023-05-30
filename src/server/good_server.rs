use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use crate::server_cfg::ServerProperties;

pub struct GoodServer {
    props: ServerProperties,
    run_directory: PathBuf,
}

impl GoodServer {
    pub fn new(run_directory: PathBuf, properties: ServerProperties) -> Self {
        Self { props: properties, run_directory }
    }

    pub fn from_directory(directory: PathBuf) -> anyhow::Result<Self> {
        let props_file = directory.join("server.toml");
        let props = ServerProperties::from_file(&props_file);
        Ok(GoodServer::new(directory, props))
    }

    async fn bind_to_port(&self) -> TcpListener {
        TcpListener::bind(self.props().address()).await.expect(&*format!(
            "failed to bind to port {} because it is already in use.", self.props().server().port()
        ))
    }

    pub async fn start(server: Arc<Mutex<Self>>) -> io::Result<()> {
        let server = server.lock().await;
        let listener = server.bind_to_port().await;
        loop {
            let (stream, _) = listener.accept().await.unwrap();

        }
    }

    pub fn props(&self) -> &ServerProperties {
        &self.props
    }
    pub fn run_directory(&self) -> &PathBuf {
        &self.run_directory
    }
}