use std::io::Read;
use std::io::Result;
use std::io::Write;

pub trait Serverbound {
    fn read_packet(input: &mut impl Read) -> Self;
    fn id(&self) -> i32;
}

pub trait Clientbound {
    fn write_packet(&self, output: &mut impl Write) -> Result<()>;
    fn id(&self) -> i32;
}
