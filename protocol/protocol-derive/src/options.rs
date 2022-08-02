use darling::FromDeriveInput;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(packet))]
pub struct PacketInfo {
    pub id: Option<i32>,
}
