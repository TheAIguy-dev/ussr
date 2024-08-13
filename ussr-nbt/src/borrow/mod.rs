pub mod reader;
mod util;
pub mod writer;

use crate::*;
use reader::Reader;
use util::*;
use writer::Writer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Nbt<'a> {
    has_name: bool,
    data: &'a [u8],
}

impl<'a> Nbt<'a> {
    /// Reads NBT data from the given reader with default options.
    #[inline]
    pub fn read(reader: &mut impl Reader<'a>) -> Result<Self, NbtReadError> {
        Self::read_with_opts(reader, ReadOpts::new())
    }

    /// Reads NBT data from the given reader with the given options.
    #[inline]
    pub fn read_with_opts(
        reader: &mut impl Reader<'a>,
        opts: ReadOpts,
    ) -> Result<Self, NbtReadError> {
        let data: *const u8 = reader.read_slice(0)?.as_ptr();
        let mut len: usize = 0;

        let root_tag: u8 = reader.read_u8()?;
        if root_tag != TAG_COMPOUND {
            return Err(NbtReadError::InvalidRootTag(root_tag));
        }
        len += 1;

        if opts.name {
            read_string(reader, &mut len)?;
        }

        read_compound(reader, &mut len)?;

        Ok(Self {
            has_name: opts.name,
            data: unsafe { std::slice::from_raw_parts(data, len) },
        })
    }

    /// Writes NBT data to the given writer with default options.
    #[inline]
    pub fn write(&self, writer: &mut impl Writer) {
        self.write_with_opts(writer, WriteOpts::new())
    }

    /// Writes NBT data to the given writer with the given options.
    #[inline]
    pub fn write_with_opts(&self, writer: &mut impl Writer, opts: WriteOpts) {
        writer.write_u8(self.data[0]);

        if opts.name {
            if !self.has_name {
                writer.write_slice(&0u16.to_be_bytes());
            }
            writer.write_slice(&self.data[1..]);
        } else {
            let mut index: usize = 1;
            if self.has_name {
                let len: u16 = self.data[index..].as_ref().read_u16().unwrap();
                index += size_of::<u16>() + len as usize;
            }
            writer.write_slice(&self.data[index..]);
        }
    }
}
