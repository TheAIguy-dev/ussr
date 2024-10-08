//! This module contains the owned NBT implementation.
//! It is significantly faster than the [`crate::owned`] module, but requires you to keep a reference to the data.
//!
//! Use this if you own the data that you will be reading from.

pub mod reader;
mod util;
pub mod writer;

use crate::{endian::RawSlice, mutf8::mstr, *};
use paste::paste;
use reader::Reader;
use util::*;
use writer::Writer;

/// A complete, named NBT structure.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Nbt<'a> {
    pub name: &'a mstr,
    pub compound: Compound<'a>,
}

/// A collection of named NBT tags.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Compound<'a> {
    pub tags: Vec<(&'a mstr, Tag<'a>)>,
}

/// A single NBT tag.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Tag<'a> {
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(&'a [u8]),
    String(&'a mstr),
    List(List<'a>),
    Compound(Compound<'a>),
    IntArray(RawSlice<'a, i32>),
    LongArray(RawSlice<'a, i64>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum List<'a> {
    Empty,
    Byte(&'a [u8]),
    Short(RawSlice<'a, i16>),
    Int(RawSlice<'a, i32>),
    Long(RawSlice<'a, i64>),
    Float(RawSlice<'a, f32>),
    Double(RawSlice<'a, f64>),
    ByteArray(Vec<&'a [u8]>),
    String(Vec<&'a mstr>),
    List(Vec<List<'a>>),
    Compound(Vec<Compound<'a>>),
    IntArray(Vec<RawSlice<'a, i32>>),
    LongArray(Vec<RawSlice<'a, i64>>),
}

impl<'a> Nbt<'a> {
    /// Read a complete NBT structure from the given reader with default options.
    #[inline]
    pub fn read(reader: &mut impl Reader<'a>) -> Result<Nbt<'a>, NbtReadError> {
        Nbt::read_with_opts(reader, ReadOpts::new())
    }

    /// Read a complete NBT structure from the given reader with custom options.
    #[inline]
    pub fn read_with_opts(
        reader: &mut impl Reader<'a>,
        opts: ReadOpts,
    ) -> Result<Nbt<'a>, NbtReadError> {
        let root_tag: u8 = reader.read_u8()?;
        if root_tag != TAG_COMPOUND {
            return Err(NbtReadError::InvalidRootTag(root_tag));
        }

        let name: &mstr = if opts.name {
            read_str(reader)?
        } else {
            mstr::new()
        };
        let compound: Compound = Compound::read(reader, 0, opts.depth_limit)?;

        Ok(Nbt { name, compound })
    }

    /// Write the NBT structure to the given writer with default options.
    #[inline]
    pub fn write(&self, writer: &mut impl Writer) {
        self.write_with_opts(writer, WriteOpts::new());
    }

    /// Write the NBT structure to the given writer with custom options.
    #[inline]
    pub fn write_with_opts(&self, writer: &mut impl Writer, opts: WriteOpts) {
        writer.write_u8(TAG_COMPOUND);
        if opts.name {
            write_str(writer, self.name);
        }
        self.compound.write(writer);
    }
}

impl<'a> Compound<'a> {
    /// Read an NBT compound from the given reader.
    #[inline]
    pub fn read(
        reader: &mut impl Reader<'a>,
        depth: u16,
        depth_limit: u16,
    ) -> Result<Compound<'a>, NbtReadError> {
        if depth >= depth_limit {
            return Err(NbtReadError::DepthLimitExceeded);
        }

        let mut tags: Vec<(&mstr, Tag)> = Vec::new();

        let mut tag_id: u8 = reader.read_u8()?;
        while tag_id != TAG_END {
            let name: &mstr = read_str(reader)?;
            let tag: Tag = Tag::read(reader, tag_id, depth + 1, depth_limit)?;
            tags.push((name, tag));
            tag_id = reader.read_u8()?;
        }

        Ok(Compound { tags })
    }

    /// Write the NBT compound to the given writer.
    #[inline]
    pub fn write(&self, writer: &mut impl Writer) {
        for (name, tag) in &self.tags {
            writer.write_u8(tag.id());
            write_str(writer, name);
            tag.write(writer);
        }

        writer.write_u8(TAG_END);
    }
}

impl<'a> Tag<'a> {
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
        reader: &mut impl Reader<'a>,
        tag_id: u8,
        depth: u16,
        depth_limit: u16,
    ) -> Result<Tag<'a>, NbtReadError> {
        if depth >= depth_limit {
            return Err(NbtReadError::DepthLimitExceeded);
        }

        Ok(match tag_id {
            TAG_BYTE => Tag::Byte(reader.read_u8()?),
            TAG_SHORT => Tag::Short(reader.read_i16()?),
            TAG_INT => Tag::Int(reader.read_i32()?),
            TAG_LONG => Tag::Long(reader.read_i64()?),
            TAG_FLOAT => Tag::Float(reader.read_f32()?),
            TAG_DOUBLE => Tag::Double(reader.read_f64()?),
            TAG_BYTE_ARRAY => Tag::ByteArray(read_byte_slice_with_len(reader)?),
            TAG_STRING => Tag::String(read_str(reader)?),
            TAG_LIST => Tag::List(List::read(reader, depth + 1, depth_limit)?),
            TAG_COMPOUND => Tag::Compound(Compound::read(reader, depth + 1, depth_limit)?),
            TAG_INT_ARRAY => Tag::IntArray(read_slice_with_len(reader)?),
            TAG_LONG_ARRAY => Tag::LongArray(read_slice_with_len(reader)?),
            tag_id => return Err(NbtReadError::InvalidTag(tag_id)),
        })
    }

    /// Write the NBT tag to the given writer.
    ///
    /// Note that this will only write up to [`i32::MAX`] elements for lists/arrays and up to [`u16::MAX`] bytes for strings.
    #[inline]
    pub fn write(&self, writer: &mut impl Writer) {
        match self {
            Tag::Byte(val) => writer.write_u8(*val),
            Tag::Short(val) => writer.write_i16(*val),
            Tag::Int(val) => writer.write_i32(*val),
            Tag::Long(val) => writer.write_i64(*val),
            Tag::Float(val) => writer.write_f32(*val),
            Tag::Double(val) => writer.write_f64(*val),
            Tag::ByteArray(slice) => writer.write_slice(slice),
            Tag::String(val) => write_str(writer, val),
            Tag::List(list) => list.write(writer),
            Tag::Compound(compound) => compound.write(writer),
            Tag::IntArray(slice) => write_slice(writer, *slice),
            Tag::LongArray(slice) => write_slice(writer, *slice),
        }
    }

    impl_tag!(byte, +u8);
    impl_tag!(short, +i16);
    impl_tag!(int, +i32);
    impl_tag!(long, +i64);
    impl_tag!(float, +f32);
    impl_tag!(double, +f64);
    impl_tag!(byte_array, &'a [u8]);
    impl_tag!(string, &'a mstr);
    impl_tag!(list, &List<'a>);
    impl_tag!(compound, &Compound<'a>);
    impl_tag!(int_array, &RawSlice<'a, i32>);
    impl_tag!(long_array, &RawSlice<'a, i64>);
}

impl<'a> List<'a> {
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
        reader: &mut impl Reader<'a>,
        depth: u16,
        depth_limit: u16,
    ) -> Result<List<'a>, NbtReadError> {
        if depth >= depth_limit {
            return Err(NbtReadError::DepthLimitExceeded);
        }

        let tag_id: u8 = reader.read_u8()?;
        let len: i32 = reader.read_i32()?;

        if len <= 0 {
            return Ok(List::Empty);
        }
        let len: usize = len as usize;

        Ok(match tag_id {
            TAG_BYTE => List::Byte(reader.read_slice(len)?),
            TAG_SHORT => List::Short(read_slice(reader, len)?),
            TAG_INT => List::Int(read_slice(reader, len)?),
            TAG_LONG => List::Long(read_slice(reader, len)?),
            TAG_FLOAT => List::Float(read_slice(reader, len)?),
            TAG_DOUBLE => List::Double(read_slice(reader, len)?),
            TAG_BYTE_ARRAY => {
                let mut buf: Vec<&[u8]> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_byte_slice_with_len(reader)?);
                }
                List::ByteArray(buf)
            }
            TAG_STRING => {
                let mut buf: Vec<&mstr> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_str(reader)?);
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
                let mut buf: Vec<RawSlice<'a, i32>> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_slice_with_len(reader)?);
                }
                List::IntArray(buf)
            }
            TAG_LONG_ARRAY => {
                let mut buf: Vec<RawSlice<'a, i64>> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_slice_with_len(reader)?);
                }
                List::LongArray(buf)
            }
            tag_id => return Err(NbtReadError::InvalidTag(tag_id)),
        })
    }

    /// Write the NBT list to the given writer.
    ///
    /// This will write the ID of the elements and the length of the list before the list itself.
    ///
    /// Note that this will only write up to [`i32::MAX`] elements for lists/arrays and up to [`u16::MAX`] bytes for strings.
    #[inline]
    pub fn write(&self, writer: &mut impl Writer) {
        writer.write_u8(self.id());

        match self {
            List::Byte(slice) => {
                let len: i32 = slice.len().min(i32::MAX as usize) as i32;
                writer.write_i32(len);
                writer.write_slice(&slice[..len as usize]);
            }
            List::Short(slice) => write_slice(writer, *slice),
            List::Int(slice) => write_slice(writer, *slice),
            List::Long(slice) => write_slice(writer, *slice),
            List::Float(slice) => write_slice(writer, *slice),
            List::Double(slice) => write_slice(writer, *slice),
            List::ByteArray(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32(len);
                for s in &vec[..len as usize] {
                    writer.write_slice(s);
                }
            }
            List::String(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32(len);
                for s in &vec[..len as usize] {
                    write_str(writer, s);
                }
            }
            List::List(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32(len);
                for l in vec {
                    l.write(writer);
                }
            }
            List::Compound(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32(len);
                for c in &vec[..len as usize] {
                    c.write(writer);
                }
            }
            List::IntArray(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32(len);
                for &s in &vec[..len as usize] {
                    write_slice(writer, s);
                }
            }
            List::LongArray(vec) => {
                let len: i32 = vec.len().min(i32::MAX as usize) as i32;
                writer.write_i32(len);
                for &s in &vec[..len as usize] {
                    write_slice(writer, s);
                }
            }
            List::Empty => {
                writer.write_i32(0);
            }
        }
    }

    impl_list!(byte, &'a [u8], &[]);
    impl_list!(short, +RawSlice<'a, i16>, RawSlice::new());
    impl_list!(int, +RawSlice<'a, i32>, RawSlice::new());
    impl_list!(long, +RawSlice<'a, i64>, RawSlice::new());
    impl_list!(float, +RawSlice<'a, f32>, RawSlice::new());
    impl_list!(double, +RawSlice<'a, f64>, RawSlice::new());
    impl_list!(byte_array, &Vec<&'a [u8]>, &Vec::new());
    impl_list!(string, &Vec<&'a mstr>, &Vec::new());
    impl_list!(list, &Vec<List<'a>>, &Vec::new());
    impl_list!(compound, &Vec<Compound<'a>>, &Vec::new());
    impl_list!(int_array, &Vec<RawSlice<'a, i32>>, &Vec::new());
    impl_list!(long_array, &Vec<RawSlice<'a, i64>>, &Vec::new());
}
