use std::io::Write;
use std::io::Read;
use std::io::Result;

pub trait Serverbound {
    fn read_packet(input: &mut impl Read) -> Self;
    fn id() -> i32;
}

pub trait Clientbound {
    fn write_packet(&self, output: &mut impl Write) -> Result<()>;
    fn id() -> i32;
}
