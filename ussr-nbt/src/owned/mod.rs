// mod impls;
mod util;

use std::io::Read;

use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use io::Write;

use crate::{NbtReadError, *};
use util::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Nbt {
    has_name: bool,
    data: Vec<u8>,
}

impl Nbt {
    /// Reads NBT data from the given reader with default options.
    #[inline]
    pub fn read(reader: &mut impl Read) -> Result<Self, NbtReadError> {
        Self::read_with_opts(reader, ReadOpts::new())
    }

    /// Reads NBT data from the given reader with the given options.
    #[inline]
    pub fn read_with_opts(reader: &mut impl Read, opts: ReadOpts) -> Result<Self, NbtReadError> {
        let root_tag: u8 = reader.read_u8()?;
        if root_tag != TAG_COMPOUND {
            return Err(NbtReadError::InvalidRootTag(root_tag));
        }

        let mut data: Vec<u8> = vec![root_tag];

        if opts.name {
            read_string(reader, &mut data)?;
        }

        read_compound(reader, &mut data)?;

        Ok(Self {
            has_name: opts.name,
            data,
        })
    }

    /// Writes NBT data to the given writer with default options.
    #[inline]
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        self.write_with_opts(writer, WriteOpts::new())
    }

    /// Writes NBT data to the given writer with the given options.
    #[inline]
    pub fn write_with_opts(&self, writer: &mut impl Write, opts: WriteOpts) -> io::Result<()> {
        writer.write_u8(self.data[0])?;

        if opts.name {
            if !self.has_name {
                writer.write_all(&0u16.to_be_bytes())?;
            }
            writer.write_all(&self.data[1..])?;
        } else {
            let mut index: usize = 1;
            if self.has_name {
                let len: u16 = self.data[index..].as_ref().read_u16::<BE>()?;
                index += size_of::<u16>() + len as usize;
            }
            writer.write_all(&self.data[index..])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let buf: Vec<u8> = vec![
            10, 0, 8, 84, 101, 115, 116, 32, 78, 66, 84, 1, 0, 9, 84, 101, 115, 116, 32, 98, 121,
            116, 101, 123, 8, 0, 11, 84, 101, 115, 116, 32, 115, 116, 114, 105, 110, 103, 0, 11,
            72, 101, 108, 108, 111, 44, 32, 78, 66, 84, 33, 9, 0, 9, 84, 101, 115, 116, 32, 108,
            105, 115, 116, 5, 0, 0, 0, 3, 63, 128, 0, 0, 64, 0, 0, 0, 64, 64, 0, 0, 0,
        ];
        println!("{:?}", Nbt::read(&mut &buf[..]).unwrap());
    }
}
