use std::io::Write;
use std::io::Read;
use std::io::Result;

pub trait Serverbound {
    fn read_packet(input: &mut impl Read) -> Self;
}

pub trait Clientbound {
    fn write_packet(&self, output: &mut impl Write) -> Result<()>;
}
