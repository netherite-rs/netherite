use chat::text_component::TextComponent;
use protocol::fields::numeric::VarInt;
use protocol::fields::profile::GameProfile;
use protocol::packet_io::PacketReaderExt;
use protocol_derive::{Clientbound, Serverbound};

#[derive(Serverbound, Debug)]
#[packet(id = 0x00)]
pub struct LoginStart {
    pub name: String,
    pub has_sig_data: bool,
    pub timestamp: Option<i64>,
    pub public_key: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
}

#[derive(Clientbound)]
#[packet(id = 0x00)]
pub struct Disconnect {
    reason: TextComponent,
}

#[derive(Clientbound)]
#[packet(id = 0x01)]
pub struct EncryptionRequest {
    pub(crate) server_id: String,
    pub(crate) public_key: Vec<u8>,
    pub(crate) verify_token: Vec<u8>,
}

#[derive(Clientbound)]
#[packet(id = 0x02)]
pub struct LoginSuccess {
    pub profile: GameProfile,
}

#[derive(Clientbound)]
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
    pub message_signature: Option<i64>,
}

impl protocol::bound::Serverbound for EncryptionResponse {
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
    fn id(&self) -> i32 { 0x01 }
}
