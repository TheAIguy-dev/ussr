use std::io::Read;

use bytemuck::{cast_slice_mut, zeroed_vec, Pod};
use byteorder::{ReadBytesExt, BE};

use crate::{mutf8::MString, *};

#[derive(Debug)]
pub struct Nbt {
    pub name: MString,
    pub tags: Compound,
}

#[derive(Debug)]
pub struct Compound {
    pub tags: Vec<(MString, Tag)>,
}

#[derive(Debug)]
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
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug)]
pub enum List {
    Byte(Vec<u8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<u8>>),
    MString(Vec<MString>),
    List(Vec<List>),
    Compound(Vec<Compound>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}

impl Nbt {
    pub fn read(reader: &mut impl Read) -> Result<Self, NbtReadError> {
        let root_tag: u8 = reader.read_u8()?;
        if root_tag != TAG_COMPOUND {
            return Err(NbtReadError::InvalidRootTag(root_tag));
        }

        let name: MString = read_string(reader)?;
        let tags: Compound = Compound::read(reader)?;

        Ok(Nbt { name, tags })
    }

    pub fn read_nameless(reader: &mut impl Read) -> Result<Self, NbtReadError> {
        let root_tag: u8 = reader.read_u8()?;
        if root_tag != TAG_COMPOUND {
            return Err(NbtReadError::InvalidRootTag(root_tag));
        }

        let name: MString = MString::new();
        let tags: Compound = Compound::read(reader)?;

        Ok(Nbt { name, tags })
    }
}

impl Compound {
    pub fn read(reader: &mut impl Read) -> Result<Self, NbtReadError> {
        let mut tags: Vec<(MString, Tag)> = Vec::new();

        let mut tag_id: u8 = reader.read_u8()?;
        while tag_id != TAG_END {
            let name: MString = read_string(reader)?;
            let tag: Tag = Tag::read(reader, tag_id)?;
            tags.push((name, tag));
            tag_id = reader.read_u8()?;
        }

        Ok(Compound { tags })
    }
}

impl Tag {
    pub fn read(reader: &mut impl Read, tag_id: u8) -> Result<Self, NbtReadError> {
        Ok(match tag_id {
            TAG_BYTE => Tag::Byte(reader.read_u8()?),
            TAG_SHORT => Tag::Short(reader.read_i16::<BE>()?),
            TAG_INT => Tag::Int(reader.read_i32::<BE>()?),
            TAG_LONG => Tag::Long(reader.read_i64::<BE>()?),
            TAG_FLOAT => Tag::Float(reader.read_f32::<BE>()?),
            TAG_DOUBLE => Tag::Double(reader.read_f64::<BE>()?),
            TAG_BYTE_ARRAY => Tag::ByteArray(read_array(reader)?),
            TAG_STRING => Tag::String(read_string(reader)?),
            TAG_LIST => Tag::List(List::read(reader)?),
            TAG_COMPOUND => Tag::Compound(Compound::read(reader)?),
            TAG_INT_ARRAY => Tag::IntArray(read_array(reader)?),
            TAG_LONG_ARRAY => Tag::LongArray(read_array(reader)?),
            tag_id => return Err(NbtReadError::InvalidTag(tag_id)),
        })
    }
}

impl List {
    pub fn read(reader: &mut impl Read) -> Result<Self, NbtReadError> {
        let tag_id: u8 = reader.read_u8()?;
        Ok(match tag_id {
            TAG_BYTE => List::Byte(read_array(reader)?),
            TAG_SHORT => List::Short(read_array(reader)?),
            TAG_INT => List::Int(read_array(reader)?),
            TAG_LONG => List::Long(read_array(reader)?),
            TAG_FLOAT => List::Float(read_array(reader)?),
            TAG_DOUBLE => List::Double(read_array(reader)?),
            TAG_BYTE_ARRAY => {
                let len: u32 = reader.read_u32::<BE>()?;
                let mut buf: Vec<Vec<u8>> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(read_array(reader)?);
                }
                List::ByteArray(buf)
            }
            TAG_STRING => {
                let len: u32 = reader.read_u32::<BE>()?;
                let mut buf: Vec<MString> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(read_string(reader)?);
                }
                List::MString(buf)
            }
            TAG_LIST => {
                let len: u32 = reader.read_u32::<BE>()?;
                let mut buf: Vec<List> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(List::read(reader)?);
                }
                List::List(buf)
            }
            TAG_COMPOUND => {
                let len: u32 = reader.read_u32::<BE>()?;
                let mut buf: Vec<Compound> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(Compound::read(reader)?);
                }
                List::Compound(buf)
            }
            TAG_INT_ARRAY => {
                let len: u32 = reader.read_u32::<BE>()?;
                let mut buf: Vec<Vec<i32>> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(read_array(reader)?);
                }
                List::IntArray(buf)
            }
            TAG_LONG_ARRAY => {
                let len: u32 = reader.read_u32::<BE>()?;
                let mut buf: Vec<Vec<i64>> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(read_array(reader)?);
                }
                List::LongArray(buf)
            }
            tag_id => return Err(NbtReadError::InvalidTag(tag_id)),
        })
    }
}

fn read_string(reader: &mut impl Read) -> Result<MString, NbtReadError> {
    let len: u16 = reader.read_u16::<BE>()?;
    let mut buf: Vec<u8> = vec![0; len as usize];
    reader.read_exact(&mut buf)?;
    Ok(MString::from_vec(buf))
}

pub fn read_array<T: Pod>(reader: &mut impl Read) -> Result<Vec<T>, NbtReadError> {
    let len: u32 = reader.read_u32::<BE>()?;
    let mut buf: Vec<T> = zeroed_vec(len as usize);
    reader.read_exact(cast_slice_mut(&mut buf))?;
    Ok(buf)
}
