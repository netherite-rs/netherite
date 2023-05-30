use std::io::{Read, Result};

use nbt::Blob;

use bytebuffer::ByteBuffer;
use crate::chat::text_component::TextComponent;
use crate::protocol::{Clientbound, Serverbound};
use crate::protocol::fields::key::Key;
use crate::protocol::fields::numeric::VarInt;
use crate::protocol::fields::position::Position;
use crate::protocol::fields::profile::GameProfile;
use crate::protocol::packet_io::{PacketReaderExt, PacketWriterExt};
use protocol_derive::{Clientbound, Serverbound};
use uuid::Uuid;

// #[derive(Serverbound, Debug)]
// #[packet(id = 0x00)]
// pub struct LoginStart {
//     pub name: String,
//     pub has_sig_data: bool,
//     pub timestamp: Option<i64>,
//     pub public_key: Option<Vec<u8>>,
//     pub signature: Option<Vec<u8>>,
// }

#[derive(Serverbound, Debug)]
#[packet(id = 0x00)]
pub struct LoginStart {
    pub name: String,
    pub has_uuid: bool,
    pub uuid: Option<Uuid>
}

#[derive(Clientbound, Debug)]
#[packet(id = 0x00)]
pub struct DisconnectLogin {
    pub reason: TextComponent,
}

#[derive(Clientbound, Debug)]
#[packet(id = 0x01)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
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

#[derive(Serverbound, Debug)]
#[packet(id = 0x01)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    // pub has_verify_token: bool,
    pub verify_token: Vec<u8>,
    // pub salt: Option<i64>,
    // pub message_signature: Option<Vec<u8>>,
}

// impl Serverbound for EncryptionResponse {
//     fn read_packet(input: &mut ByteBuffer) -> EncryptionResponse {
//         let shared_secret = input.read_field::<Vec<u8>>().expect("failed to read shared_secret");
//         let verify_ = input.read_bool().expect("failed to read has_verify_token");
//         EncryptionResponse {
//             shared_secret,
//             has_verify_token,
//             verify_token: if has_verify_token { input.read_field().expect("failed to read verify_token") } else { None },
//             salt: if !has_verify_token { input.read_field().expect("failed to read salt") } else { None },
//             message_signature: if !has_verify_token { input.read_field().expect("failed to read message_signature") } else { None },
//         }
//     }
//     fn id() -> i32 { 0x01 }
// }

#[derive(Clientbound, Serverbound, Debug)]
#[packet(id = 0x28)]
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
    pub death_location: Option<Position>,
}

pub struct LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: Key,
    pub data: Vec<u8>,
}

pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub successful: bool,
    pub data: Option<Vec<u8>>,
}

impl Clientbound for LoginPluginRequest {
    fn write_packet(&self, output: &mut ByteBuffer) -> Result<()> {
        output.write_varint(&self.message_id)?;
        output.write_field(&self.channel)?;
        output.write_bytes(&self.data);
        Ok(())
    }

    fn id() -> i32 { 0x04 }
}

impl Serverbound for LoginPluginResponse {
    fn read_packet(input: &mut ByteBuffer) -> Self {
        let message_id = input.read_varint().expect("failed to read 'message_id'");
        let successful = input.read_bool().expect("failed to read 'successful'");
        let data = if successful {
            let mut data = Vec::new();
            input.read_to_end(&mut data).expect("failed to read 'data'");
            Some(data)
        } else {
            None
        };
        LoginPluginResponse {
            message_id,
            successful,
            data,
        }
    }

    fn id() -> i32 { 0x02 }
}