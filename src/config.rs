use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use base64::Engine;
use base64::engine::general_purpose;

use derive_getters::Getters;
use serde::Deserialize;

use crate::game_mode::GameMode;

#[derive(Deserialize, Getters)]
pub struct ServerProperties {
    server: ServerSection,
    status: StatusSection,
    game: GameSection,
}

#[derive(Deserialize, Getters)]
#[serde(rename_all = "kebab-case")]
pub struct GameSection {
    default_gamemode: GameMode,
    view_distance: u8,
    simulation_disance: u8,
    reduce_debug_info: bool,
    enable_respawn_screen: bool,
}

#[derive(Deserialize, Getters)]
#[serde(rename_all = "kebab-case")]
pub struct ServerSection {
    address: String,
    port: u32,
    online_mode: bool,
    compression_threshold: u32,
}

#[derive(Deserialize, Getters)]
#[serde(rename_all = "kebab-case")]
pub struct StatusSection {
    motd: String,
    max_players: usize,
    icon: String,
}

impl StatusSection {
    // TODO: cache the icon
    pub fn read_icon(&self, run_directory: &Path) -> String {
        let mut file = File::open(run_directory.join(&self.icon)).unwrap();
        let mut vec = Vec::new();
        let _ = file.read_to_end(&mut vec);
        let base64 = general_purpose::STANDARD.encode(&vec);
        return format!("data:image/png;base64,{}", base64.replace("\r\n", ""));
    }
}

impl ServerProperties {
    pub fn read(path: &Path) -> Result<ServerProperties, std::io::Error> {
        let string = fs::read_to_string(path)?;
        toml::de::from_str(&string).map_err(|why| std::io::Error::from(why))
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server().address(), self.server().port())
    }
}