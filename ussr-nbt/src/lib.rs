use std::io::{self, Read, Write};

use ussr_buf::{ReadError, Readable, Writable};

#[derive(Debug)]
pub struct Nbt;

impl Readable for Nbt {
    fn read_from(_: &mut impl Read) -> Result<Self, ReadError> {
        todo!()
    }
}

impl Writable for Nbt {
    fn write_to(&self, _: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}
