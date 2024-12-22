mod util;

use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, BE};

use crate::*;
use util::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Nbt {
    pub name: String,
    pub compound: Compound,
}

/// A collection of named NBT tags.
#[derive(Debug, Clone, PartialEq)]
pub struct Compound {
    pub tags: Vec<(String, Tag)>,
}

/// A single NBT tag.
#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(List),
    Compound(Compound),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

/// A list of NBT tags.
#[derive(Debug, Clone, PartialEq)]
pub enum List {
    Empty,
    Byte(Vec<u8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<u8>>),
    String(Vec<String>),
    List(Vec<List>),
    Compound(Vec<Compound>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}

impl Nbt {
    /// Read a complete NBT structure from the given reader with default options.
    #[inline]
    pub fn read(reader: &mut impl Read) -> Result<Nbt, NbtDecodeError> {
        Nbt::read_with_opts(reader, DecodeOpts::default())
    }

    /// Read a complete NBT structure from the given reader with custom options.
    #[inline]
    pub fn read_with_opts(reader: &mut impl Read, opts: DecodeOpts) -> Result<Nbt, NbtDecodeError> {
        let root_tag: u8 = reader.read_u8()?;
        if root_tag != TAG_COMPOUND {
            return Err(NbtDecodeError::InvalidRootTag(root_tag));
        }

        let name: String = if opts.named {
            read_string(reader)?
        } else {
            String::new()
        };

        let compound: Compound = Compound::read(reader, 0, opts.depth_limit)?;

        Ok(Nbt { name, compound })
    }

    /// Write the NBT structure to the given writer with default options.
    #[inline]
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        self.write_with_opts(writer, EncodeOpts::default())
    }

    /// Write the NBT structure to the given writer with custom options.
    #[inline]
    pub fn write_with_opts(&self, writer: &mut impl Write, opts: EncodeOpts) -> io::Result<()> {
        writer.write_u8(TAG_COMPOUND)?;
        if opts.named {
            write_string(writer, &self.name)?;
        }
        self.compound.write(writer)?;

        Ok(())
    }
}

impl Compound {
    /// Read an NBT compound from the given reader.
    #[inline]
    pub fn read(
        reader: &mut impl Read,
        depth: u16,
        depth_limit: u16,
    ) -> Result<Compound, NbtDecodeError> {
        if depth >= depth_limit {
            return Err(NbtDecodeError::DepthLimitExceeded);
        }

        let mut tags: Vec<(String, Tag)> = Vec::new();

        let mut tag_id: u8 = reader.read_u8()?;
        while tag_id != TAG_END {
            let name: String = read_string(reader)?;
            let tag: Tag = Tag::read(reader, tag_id, depth + 1, depth_limit)?;
            tags.push((name, tag));
            tag_id = reader.read_u8()?;
        }

        Ok(Compound { tags })
    }

    /// Write the NBT compound to the given writer.
    #[inline]
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        for (name, tag) in &self.tags {
            writer.write_u8(tag.id())?;
            write_string(writer, name)?;
            tag.write(writer)?;
        }

        writer.write_u8(TAG_END)?;

        Ok(())
    }
}

impl Tag {
    /// Get the ID of the NBT tag.
    #[must_use]
    #[inline]
    pub const fn id(&self) -> u8 {
        match self {
            Tag::Byte(_) => TAG_BYTE,
            Tag::Short(_) => TAG_SHORT,
            Tag::Int(_) => TAG_INT,
            Tag::Long(_) => TAG_LONG,
            Tag::Float(_) => TAG_FLOAT,
            Tag::Double(_) => TAG_DOUBLE,
            Tag::ByteArray(_) => TAG_BYTE_ARRAY,
            Tag::String(_) => TAG_STRING,
            Tag::List(_) => TAG_LIST,
            Tag::Compound(_) => TAG_COMPOUND,
            Tag::IntArray(_) => TAG_INT_ARRAY,
            Tag::LongArray(_) => TAG_LONG_ARRAY,
        }
    }

    /// Read an NBT tag from the given reader.
    ///
    /// Note that [`TAG_END`] is not considered a valid tag.
    #[inline]
    pub fn read(
        reader: &mut impl Read,
        tag_id: u8,
        depth: u16,
        depth_limit: u16,
    ) -> Result<Tag, NbtDecodeError> {
        if depth >= depth_limit {
            return Err(NbtDecodeError::DepthLimitExceeded);
        }

        Ok(match tag_id {
            TAG_BYTE => Tag::Byte(reader.read_u8()?),
            TAG_SHORT => Tag::Short(reader.read_i16::<BE>()?),
            TAG_INT => Tag::Int(reader.read_i32::<BE>()?),
            TAG_LONG => Tag::Long(reader.read_i64::<BE>()?),
            TAG_FLOAT => Tag::Float(reader.read_f32::<BE>()?),
            TAG_DOUBLE => Tag::Double(reader.read_f64::<BE>()?),
            TAG_BYTE_ARRAY => Tag::ByteArray(read_byte_vec_with_len(reader)?),
            TAG_STRING => Tag::String(read_string(reader)?),
            TAG_LIST => Tag::List(List::read(reader, depth + 1, depth_limit)?),
            TAG_COMPOUND => Tag::Compound(Compound::read(reader, depth + 1, depth_limit)?),
            TAG_INT_ARRAY => Tag::IntArray(read_vec_with_len(reader)?),
            TAG_LONG_ARRAY => Tag::LongArray(read_vec_with_len(reader)?),
            tag_id => return Err(NbtDecodeError::InvalidTag(tag_id)),
        })
    }

    /// Write the NBT tag to the given writer.
    ///
    /// Note that this will only write up to [`i32::MAX`] elements for lists/arrays and up to [`u16::MAX`] bytes for strings.
    #[inline]
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            Tag::Byte(val) => writer.write_u8(*val),
            Tag::Short(val) => writer.write_i16::<BE>(*val),
            Tag::Int(val) => writer.write_i32::<BE>(*val),
            Tag::Long(val) => writer.write_i64::<BE>(*val),
            Tag::Float(val) => writer.write_f32::<BE>(*val),
            Tag::Double(val) => writer.write_f64::<BE>(*val),
            Tag::ByteArray(vec) => write_byte_vec(writer, vec),
            Tag::String(val) => write_string(writer, val),
            Tag::List(list) => list.write(writer),
            Tag::Compound(compound) => compound.write(writer),
            Tag::IntArray(vec) => write_vec(writer, vec),
            Tag::LongArray(vec) => write_vec(writer, vec),
        }
    }

    impl_tag!(byte, u8: Copy);
    impl_tag!(short, i16: Copy);
    impl_tag!(int, i32: Copy);
    impl_tag!(long, i64: Copy);
    impl_tag!(float, f32: Copy);
    impl_tag!(double, f64: Copy);
    impl_tag!(byte_array, Vec<u8>: !Copy);
    impl_tag!(string, String: !Copy);
    impl_tag!(list, List: !Copy);
    impl_tag!(compound, Compound: !Copy);
    impl_tag!(int_array, Vec<i32>: !Copy);
    impl_tag!(long_array, Vec<i64>: !Copy);
}

impl List {
    /// Get the ID of the elements in the NBT list.
    #[must_use]
    #[inline]
    pub const fn id(&self) -> u8 {
        match self {
            List::Empty => TAG_END,
            List::Byte(_) => TAG_BYTE,
            List::Short(_) => TAG_SHORT,
            List::Int(_) => TAG_INT,
            List::Long(_) => TAG_LONG,
            List::Float(_) => TAG_FLOAT,
            List::Double(_) => TAG_DOUBLE,
            List::ByteArray(_) => TAG_BYTE_ARRAY,
            List::String(_) => TAG_STRING,
            List::List(_) => TAG_LIST,
            List::Compound(_) => TAG_COMPOUND,
            List::IntArray(_) => TAG_INT_ARRAY,
            List::LongArray(_) => TAG_LONG_ARRAY,
        }
    }

    /// Read an NBT list from the given reader.
    ///
    /// This will read the ID of the elements and the length of the list before the list itself.
    ///
    /// Returns [`List::Empty`] if the length is less than or equal to 0.
    #[inline]
    pub fn read(
        reader: &mut impl Read,
        depth: u16,
        depth_limit: u16,
    ) -> Result<List, NbtDecodeError> {
        if depth >= depth_limit {
            return Err(NbtDecodeError::DepthLimitExceeded);
        }

        let tag_id: u8 = reader.read_u8()?;
        let len: i32 = reader.read_i32::<BE>()?;

        if len <= 0 {
            return Ok(List::Empty);
        }
        let len: usize = len as usize;

        Ok(match tag_id {
            TAG_BYTE => List::Byte(read_byte_vec(reader, len)?),
            TAG_SHORT => List::Short(read_vec(reader, len)?),
            TAG_INT => List::Int(read_vec(reader, len)?),
            TAG_LONG => List::Long(read_vec(reader, len)?),
            TAG_FLOAT => List::Float(read_vec(reader, len)?),
            TAG_DOUBLE => List::Double(read_vec(reader, len)?),
            TAG_BYTE_ARRAY => {
                let mut buf: Vec<Vec<u8>> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_byte_vec_with_len(reader)?);
                }
                List::ByteArray(buf)
            }
            TAG_STRING => {
                let mut buf: Vec<String> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_string(reader)?);
                }
                List::String(buf)
            }
            TAG_LIST => {
                let mut buf: Vec<List> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(List::read(reader, depth + 1, depth_limit)?);
                }
                List::List(buf)
            }
            TAG_COMPOUND => {
                let mut buf: Vec<Compound> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(Compound::read(reader, depth + 1, depth_limit)?);
                }
                List::Compound(buf)
            }
            TAG_INT_ARRAY => {
                let mut buf: Vec<Vec<i32>> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_vec_with_len(reader)?);
                }
                List::IntArray(buf)
            }
            TAG_LONG_ARRAY => {
                let mut buf: Vec<Vec<i64>> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_vec_with_len(reader)?);
                }
                List::LongArray(buf)
            }
            tag_id => return Err(NbtDecodeError::InvalidTag(tag_id)),
        })
    }

    /// Write the NBT list to the given writer.
    ///
    /// This will write the ID of the elements and the length of the list before the list itself.
    ///
    /// Note that this will only write up to [`i32::MAX`] elements for lists/arrays and up to [`u16::MAX`] bytes for strings.
    #[inline]
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(self.id())?;

        match self {
            List::Byte(vec) => write_byte_vec(writer, vec)?,
            List::Short(vec) => write_vec(writer, vec)?,
            List::Int(vec) => write_vec(writer, vec)?,
            List::Long(vec) => write_vec(writer, vec)?,
            List::Float(vec) => write_vec(writer, vec)?,
            List::Double(vec) => write_vec(writer, vec)?,
            List::ByteArray(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32::<BE>(len)?;
                for v in &vec[..len as usize] {
                    write_byte_vec(writer, v)?;
                }
            }
            List::String(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32::<BE>(len)?;
                for s in &vec[..len as usize] {
                    write_string(writer, s)?;
                }
            }
            List::List(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32::<BE>(len)?;
                for l in vec {
                    l.write(writer)?;
                }
            }
            List::Compound(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32::<BE>(len)?;
                for c in &vec[..len as usize] {
                    c.write(writer)?;
                }
            }
            List::IntArray(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32::<BE>(len)?;
                for v in &vec[..len as usize] {
                    write_vec(writer, v)?;
                }
            }
            List::LongArray(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32::<BE>(len)?;
                for v in &vec[..len as usize] {
                    write_vec(writer, v)?;
                }
            }
            List::Empty => {
                writer.write_i32::<BE>(0)?;
            }
        }

        Ok(())
    }

    impl_list!(byte, Vec<u8>);
    impl_list!(short, Vec<i16>);
    impl_list!(int, Vec<i32>);
    impl_list!(long, Vec<i64>);
    impl_list!(float, Vec<f32>);
    impl_list!(double, Vec<f64>);
    impl_list!(string, Vec<String>);
    impl_list!(list, Vec<List>);
    impl_list!(compound, Vec<Compound>);
    impl_list!(int_array, Vec<Vec<i32>>);
    impl_list!(long_array, Vec<Vec<i64>>);
}
