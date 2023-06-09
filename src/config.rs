use std::fs;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use base64::Engine;
use base64::engine::general_purpose;
use derive_getters::Getters;
use crate::game_mode::GameMode;
use serde::Deserialize;
use chat::style::RgbColor;
use chat::text_component::TextComponent;

#[derive(Deserialize, Getters)]
pub struct ServerProperties {
    server: ServerSection,
    game: GameSection,
    status: StatusSection,
    icon: String,
    description: TextComponent,
    address: String,
}

impl ServerProperties {
    pub fn new(
        server: ServerSection,
        game: GameSection,
        status: StatusSection,
        icon: String,
        description: TextComponent,
        address: String,
    ) -> Self {
        Self { server, game, status, icon, description, address }
    }

    pub fn from_file(path: &Path) -> ServerProperties {
        let config = fs::read_to_string(path).unwrap();
        let props: RawServerProps = toml::de::from_str(&config)
            .expect(&*format!("failed to read {:?}", &path.to_str()));
        let icon = props.read_icon(path.parent().unwrap());
        let description = TextComponent::builder()
            .text(props.status.motd.to_string())
            .color(&RgbColor::new(230, 47, 70))
            .build();
        let address = format!("{}:{}", props.server.address, props.server.port);
        ServerProperties {
            server: props.server,
            game: props.game,
            status: props.status,
            icon,
            description,
            address,
        }
    }
}

#[derive(Deserialize)]
struct RawServerProps {
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

impl RawServerProps {
    fn read_icon(&self, run_directory: &Path) -> String {
        let mut file = File::open(run_directory.join(&self.status.icon))
            .expect(&*format!("no such server icon file: {}", self.status.icon));
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf);
        let base64 = general_purpose::STANDARD.encode(&buf);
        return format!("data:image/png;base64,{}", base64.replace("\r\n", ""));
    }
}