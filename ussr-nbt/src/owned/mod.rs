//! This module contains the owned NBT implementation.
//!
//! Use this if you want to construct NBT structures yourself or if you don't own the data that you will be reading from.

mod impls;
mod util;

use std::io::Read;

use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use io::Write;

use crate::{endian::RawVec, mutf8::MString, NbtReadError, *};
use util::*;

/// A complete, named NBT structure.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Nbt {
    pub name: MString,
    pub compound: Compound,
}

/// A collection of named NBT tags.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Compound {
    pub tags: Vec<(MString, Tag)>,
}

/// A single NBT tag.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Tag {
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(MString),
    List(List),
    Compound(Compound),
    IntArray(RawVec<i32>),
    LongArray(RawVec<i64>),
}

/// A list of NBT tags.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum List {
    Empty,
    Byte(Vec<u8>),
    Short(RawVec<i16>),
    Int(RawVec<i32>),
    Long(RawVec<i64>),
    Float(RawVec<f32>),
    Double(RawVec<f64>),
    ByteArray(Vec<Vec<u8>>),
    String(Vec<MString>),
    List(Vec<List>),
    Compound(Vec<Compound>),
    IntArray(Vec<RawVec<i32>>),
    LongArray(Vec<RawVec<i64>>),
}

impl Nbt {
    /// Read a complete NBT structure from the given reader with default options.
    #[inline]
    pub fn read(reader: &mut impl Read) -> Result<Self, NbtReadError> {
        Self::read_with_opts(reader, ReadOpts::new())
    }

    /// Read a complete NBT structure from the given reader with custom options.
    #[inline]
    pub fn read_with_opts(reader: &mut impl Read, opts: ReadOpts) -> Result<Self, NbtReadError> {
        let root_tag: u8 = reader.read_u8()?;
        if root_tag != TAG_COMPOUND {
            return Err(NbtReadError::InvalidRootTag(root_tag));
        }

        let name: MString = if opts.name {
            read_string(reader)?
        } else {
            MString::new()
        };
        let compound: Compound = Compound::read(reader, 0, opts.depth_limit)?;

        Ok(Nbt { name, compound })
    }

    /// Write the NBT structure to the given writer with default options.
    #[inline]
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        self.write_with_opts(writer, WriteOpts::new())
    }

    /// Write the NBT structure to the given writer with custom options.
    #[inline]
    pub fn write_with_opts(&self, writer: &mut impl Write, opts: WriteOpts) -> io::Result<()> {
        writer.write_u8(TAG_COMPOUND)?;
        if opts.name {
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
    ) -> Result<Self, NbtReadError> {
        if depth >= depth_limit {
            return Err(NbtReadError::DepthLimitExceeded);
        }

        let mut tags: Vec<(MString, Tag)> = Vec::new();

        let mut tag_id: u8 = reader.read_u8()?;
        while tag_id != TAG_END {
            let name: MString = read_string(reader)?;
            let tag: Tag = Tag::read(reader, tag_id, depth + 1, depth_limit)?;
            // println!("{:?}: {:?}", name, tag);
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
    ) -> Result<Self, NbtReadError> {
        if depth >= depth_limit {
            return Err(NbtReadError::DepthLimitExceeded);
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
            tag_id => return Err(NbtReadError::InvalidTag(tag_id)),
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

    /// Get the byte value of the NBT tag if it is a byte tag.
    ///
    /// If the tag is not a byte tag, [`None`] is returned.
    #[inline]
    pub fn byte(&self) -> Option<u8> {
        match self {
            Tag::Byte(val) => Some(*val),
            _ => None,
        }
    }

    /// Get the short value of the NBT tag if it is a short tag.
    ///
    /// If the tag is not a short tag, [`None`] is returned.
    #[inline]
    pub fn short(&self) -> Option<i16> {
        match self {
            Tag::Short(val) => Some(*val),
            _ => None,
        }
    }

    /// Get the int value of the NBT tag if it is an int tag.
    ///
    /// If the tag is not an int tag, [`None`] is returned.
    #[inline]
    pub fn int(&self) -> Option<i32> {
        match self {
            Tag::Int(val) => Some(*val),
            _ => None,
        }
    }

    /// Get the long value of the NBT tag if it is a long tag.
    ///
    /// If the tag is not a long tag, [`None`] is returned.
    #[inline]
    pub fn long(&self) -> Option<i64> {
        match self {
            Tag::Long(val) => Some(*val),
            _ => None,
        }
    }

    /// Get the float value of the NBT tag if it is a float tag.
    ///
    /// If the tag is not a float tag, [`None`] is returned.
    #[inline]
    pub fn float(&self) -> Option<f32> {
        match self {
            Tag::Float(val) => Some(*val),
            _ => None,
        }
    }

    /// Get the double value of the NBT tag if it is a double tag.
    ///
    /// If the tag is not a double tag, [`None`] is returned.
    #[inline]
    pub fn double(&self) -> Option<f64> {
        match self {
            Tag::Double(val) => Some(*val),
            _ => None,
        }
    }

    /// Get the byte array value of the NBT tag if it is a byte array tag or a list of bytes.
    ///
    /// If the tag is not a byte array tag, a list of bytes, or an empty list, [`None`] is returned.
    #[inline]
    pub fn byte_array(&self) -> Option<&Vec<u8>> {
        match self {
            Tag::ByteArray(val) => Some(val),
            Tag::List(List::Byte(val)) => Some(val),
            Tag::List(List::Empty) => Some(const { &Vec::new() }),
            _ => None,
        }
    }

    /// Get the string value of the NBT tag if it is a string tag.
    ///
    /// If the tag is not a string tag, [`None`] is returned.
    #[inline]
    pub fn string(&self) -> Option<&MString> {
        match self {
            Tag::String(val) => Some(val),
            _ => None,
        }
    }

    /// Get the list value of the NBT tag if it is a list tag.
    ///
    /// If the tag is not a list tag, [`None`] is returned.
    #[inline]
    pub fn list(&self) -> Option<&List> {
        match self {
            Tag::List(val) => Some(val),
            _ => None,
        }
    }

    /// Get the compound value of the NBT tag if it is a compound tag.
    ///
    /// If the tag is not a compound tag, [`None`] is returned.
    #[inline]
    pub fn compound(&self) -> Option<&Compound> {
        match self {
            Tag::Compound(val) => Some(val),
            _ => None,
        }
    }

    /// Get the int array value of the NBT tag if it is an int array tag or a list of ints.
    ///
    /// If the tag is not an int array tag, a list of ints, or an empty list, [`None`] is returned.
    #[inline]
    pub fn int_array(&self) -> Option<&RawVec<i32>> {
        match self {
            Tag::IntArray(val) => Some(val),
            Tag::List(List::Int(val)) => Some(val),
            Tag::List(List::Empty) => Some(const { &RawVec::new() }),
            _ => None,
        }
    }

    /// Get the long array value of the NBT tag if it is a long array tag or a list of longs.
    ///
    /// If the tag is not a long array tag, a list of longs, or an empty list, [`None`] is returned.
    #[inline]
    pub fn long_array(&self) -> Option<&RawVec<i64>> {
        match self {
            Tag::LongArray(val) => Some(val),
            Tag::List(List::Long(val)) => Some(val),
            Tag::List(List::Empty) => Some(const { &RawVec::new() }),
            _ => None,
        }
    }
}

impl List {
    /// Get the ID of the elements in the NBT list.
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
    ) -> Result<Self, NbtReadError> {
        if depth >= depth_limit {
            return Err(NbtReadError::DepthLimitExceeded);
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
                let mut buf: Vec<MString> = Vec::with_capacity(len);
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
                let mut buf: Vec<RawVec<i32>> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_vec_with_len(reader)?);
                }
                List::IntArray(buf)
            }
            TAG_LONG_ARRAY => {
                let mut buf: Vec<RawVec<i64>> = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(read_vec_with_len(reader)?);
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
                let len: i32 = (vec.len() as i32).min(i32::MAX);
                writer.write_i32::<BE>(len)?;
                for v in &vec[..len as usize] {
                    write_byte_vec(writer, v)?;
                }
            }
            List::String(vec) => {
                let len: i32 = (vec.len() as i32).min(i32::MAX);
                writer.write_i32::<BE>(len)?;
                for s in &vec[..len as usize] {
                    write_string(writer, s)?;
                }
            }
            List::List(vec) => {
                let len: i32 = (vec.len() as i32).min(i32::MAX);
                writer.write_i32::<BE>(len)?;
                for l in vec {
                    l.write(writer)?;
                }
            }
            List::Compound(vec) => {
                let len: i32 = (vec.len() as i32).min(i32::MAX);
                writer.write_i32::<BE>(len)?;
                for c in &vec[..len as usize] {
                    c.write(writer)?;
                }
            }
            List::IntArray(vec) => {
                let len: i32 = (vec.len() as i32).min(i32::MAX);
                writer.write_i32::<BE>(len)?;
                for v in &vec[..len as usize] {
                    write_vec(writer, v)?;
                }
            }
            List::LongArray(vec) => {
                let len: i32 = (vec.len() as i32).min(i32::MAX);
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

    /// Get the byte array of the list if it is a byte array.
    ///
    /// If the list is not a byte array or an empty list, [`None`] is returned.
    #[inline]
    pub fn byte(&self) -> Option<&Vec<u8>> {
        match self {
            List::Byte(vec) => Some(vec),
            List::Empty => Some(const { &Vec::new() }),
            _ => None,
        }
    }

    /// Get the short array of the list if it is a short array.
    ///
    /// If the list is not a short array or an empty list, [`None`] is returned.
    #[inline]
    pub fn short(&self) -> Option<&RawVec<i16>> {
        match self {
            List::Short(vec) => Some(vec),
            List::Empty => Some(const { &RawVec::new() }),
            _ => None,
        }
    }

    /// Get the int array of the list if it is an int array.
    ///
    /// If the list is not an int array or an empty list, [`None`] is returned.
    #[inline]
    pub fn int(&self) -> Option<&RawVec<i32>> {
        match self {
            List::Int(vec) => Some(vec),
            List::Empty => Some(const { &RawVec::new() }),
            _ => None,
        }
    }

    /// Get the long array of the list if it is a long array.
    ///
    /// If the list is not a long array or an empty list, [`None`] is returned.
    #[inline]
    pub fn long(&self) -> Option<&RawVec<i64>> {
        match self {
            List::Long(vec) => Some(vec),
            List::Empty => Some(const { &RawVec::new() }),
            _ => None,
        }
    }

    /// Get the float array of the list if it is a float array.
    ///
    /// If the list is not a float array or an empty list, [`None`] is returned.
    #[inline]
    pub fn float(&self) -> Option<&RawVec<f32>> {
        match self {
            List::Float(vec) => Some(vec),
            List::Empty => Some(const { &RawVec::new() }),
            _ => None,
        }
    }

    /// Get the double array of the list if it is a double array.
    ///
    /// If the list is not a double array or an empty list, [`None`] is returned.
    #[inline]
    pub fn double(&self) -> Option<&RawVec<f64>> {
        match self {
            List::Double(vec) => Some(vec),
            List::Empty => Some(const { &RawVec::new() }),
            _ => None,
        }
    }

    /// Get the byte array list of the list if it is a byte array list.
    ///
    /// If the list is not a byte array list or an empty list, [`None`] is returned.
    #[inline]
    pub fn byte_array(&self) -> Option<&Vec<Vec<u8>>> {
        match self {
            List::ByteArray(vec) => Some(vec),
            List::Empty => Some(const { &Vec::new() }),
            _ => None,
        }
    }

    /// Get the string list of the list if it is a string list.
    ///
    /// If the list is not a string list or an empty list, [`None`] is returned.
    #[inline]
    pub fn string(&self) -> Option<&Vec<MString>> {
        match self {
            List::String(vec) => Some(vec),
            List::Empty => Some(const { &Vec::new() }),
            _ => None,
        }
    }

    /// Get the list list of the list if it is a list list.
    ///
    /// If the list is not a list list or an empty list, [`None`] is returned.
    ///
    /// List list list List list list.
    #[inline]
    pub fn list(&self) -> Option<&Vec<List>> {
        match self {
            List::List(vec) => Some(vec),
            List::Empty => Some(const { &Vec::new() }),
            _ => None,
        }
    }

    /// Get the compound list of the list if it is a compound list.
    ///
    /// If the list is not a compound list or an empty list, [`None`] is returned.
    #[inline]
    pub fn compound(&self) -> Option<&Vec<Compound>> {
        match self {
            List::Compound(vec) => Some(vec),
            List::Empty => Some(const { &Vec::new() }),
            _ => None,
        }
    }

    /// Get the int array list of the list if it is an int array list.
    ///
    /// If the list is not an int array list or an empty list, [`None`] is returned.
    #[inline]
    pub fn int_array(&self) -> Option<&Vec<RawVec<i32>>> {
        match self {
            List::IntArray(vec) => Some(vec),
            List::Empty => Some(const { &Vec::new() }),
            _ => None,
        }
    }

    /// Get the long array list of the list if it is a long array list.
    ///
    /// If the list is not a long array list or an empty list, [`None`] is returned.
    #[inline]
    pub fn long_array(&self) -> Option<&Vec<RawVec<i64>>> {
        match self {
            List::LongArray(vec) => Some(vec),
            List::Empty => Some(const { &Vec::new() }),
            _ => None,
        }
    }
}
