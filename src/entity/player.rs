use crate::net::codec::ClientCodec;

struct Player {
    codec: ClientCodec,
    health: f32,
}