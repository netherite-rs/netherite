use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use rustc_serialize::base64::{MIME, ToBase64};
use serde::Deserialize;
use toml::de::Error;

#[derive(Deserialize, Debug)]
pub struct ServerProperties {
    server: ServerSection,
    status: StatusSection,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ServerSection {
    address: String,
    port: u32,
    online_mode: bool,
    compression_threshold: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct StatusSection {
    motd: String,
    max_players: usize,
    icon: String,
}

impl StatusSection {
    pub fn motd(&self) -> &str {
        &self.motd
    }

    pub fn max_players(&self) -> usize {
        self.max_players
    }

    pub fn icon(&self) -> &str {
        &self.icon
    }

    pub fn read_icon(&self) -> String {
        let mut file = File::open(&self.icon).unwrap();
        let mut vec = Vec::new();
        let _ = file.read_to_end(&mut vec);
        let base64 = vec.to_base64(MIME);
        return format!("data:image/png;base64,{}", base64.replace("\r\n", ""));
    }
}

impl ServerSection {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn online_mode(&self) -> bool {
        self.online_mode
    }

    pub fn compression_threshold(&self) -> u32 {
        self.compression_threshold
    }
}

impl ServerProperties {
    pub fn read(path: &Path) -> Result<ServerProperties, Error> {
        let string = fs::read_to_string(path).unwrap();
        toml::de::from_str(&string)
    }

    pub fn server(&self) -> &ServerSection {
        &self.server
    }

    pub fn status(&self) -> &StatusSection {
        &self.status
    }
}