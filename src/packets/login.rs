use nbt::Blob;
use chat::text_component::TextComponent;
use protocol::fields::identifier::Key;
use protocol::fields::numeric::VarInt;
use protocol::fields::position::Position;
use protocol::fields::profile::GameProfile;
use protocol::packet_io::PacketReaderExt;
use protocol::{Clientbound, Serverbound};

#[derive(Serverbound, Debug)]
#[packet(id = 0x00)]
pub struct LoginStart {
    pub name: String,
    pub has_sig_data: bool,
    pub timestamp: Option<i64>,
    pub public_key: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
}

#[derive(Clientbound, Debug)]
#[packet(id = 0x00)]
pub struct DisconnectLogin {
    pub reason: TextComponent,
}

#[derive(Clientbound, Debug)]
#[packet(id = 0x01)]
pub struct EncryptionRequest {
    pub(crate) server_id: String,
    pub(crate) public_key: Vec<u8>,
    pub(crate) verify_token: Vec<u8>,
}

#[derive(Clientbound, Debug)]
#[packet(id = 0x02)]
pub struct LoginSuccess {
    pub profile: GameProfile,
}

#[derive(Clientbound, Debug)]
#[packet(id = 0x03)]
pub struct SetCompressionPacket {
    pub threshold: VarInt,
}

#[derive(Debug)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub has_verify_token: bool,
    pub verify_token: Option<Vec<u8>>,
    pub salt: Option<i64>,
    pub message_signature: Option<Vec<u8>>,
}

impl protocol::Serverbound for EncryptionResponse {
    fn read_packet(input: &mut impl std::io::Read) -> EncryptionResponse {
        let shared_secret = input.read_field::<Vec<u8>>().expect("failed to read shared_secret");
        let has_verify_token = input.read_bool().expect("failed to read has_verify_token");
        EncryptionResponse {
            shared_secret,
            has_verify_token,
            verify_token: if has_verify_token { input.read_field().expect("failed to read verify_token") } else { None },
            salt: if !has_verify_token { input.read_field().expect("failed to read salt") } else { None },
            message_signature: if !has_verify_token { input.read_field().expect("failed to read message_signature") } else { None },
        }
    }
    fn id() -> i32 { 0x01 }
}

#[derive(Clientbound, Serverbound, Debug)]
#[packet(id = 0x25)]
pub struct LoginPlay {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub game_mode: u8,
    pub previous_gamemode: i8,
    pub dimension_names: Vec<Key>,
    pub dimesion_codec: Blob,
    pub dimension_type: Key,
    pub dimension_name: Key,
    pub hashed_seed: i64,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    pub death_dimension_name: Option<Key>,
    pub death_location: Option<Position>
}