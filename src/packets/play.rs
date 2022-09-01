use chat::text_component::TextComponent;
use protocol::Clientbound;

#[derive(Clientbound, Debug)]
#[packet(id = 0x19)]
pub struct DisconnectPlay {
    pub reason: TextComponent,
}