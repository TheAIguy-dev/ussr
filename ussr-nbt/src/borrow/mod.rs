pub mod reader;

use bytemuck::Pod;
use endian::UnalignedSlice;
use mutf8::mstr;
use reader::Reader;

use crate::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Nbt<'a> {
    pub name: &'a mstr,
    pub tags: Compound<'a>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Compound<'a> {
    pub tags: Vec<(&'a mstr, Tag<'a>)>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Tag<'a> {
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(UnalignedSlice<'a, u8>),
    String(&'a mstr),
    List(List<'a>),
    Compound(Compound<'a>),
    IntArray(UnalignedSlice<'a, i32>),
    LongArray(UnalignedSlice<'a, i64>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum List<'a> {
    Byte(UnalignedSlice<'a, u8>),
    Short(UnalignedSlice<'a, i16>),
    Int(UnalignedSlice<'a, i32>),
    Long(UnalignedSlice<'a, i64>),
    Float(UnalignedSlice<'a, f32>),
    Double(UnalignedSlice<'a, f64>),
    ByteArray(Vec<UnalignedSlice<'a, u8>>),
    String(Vec<&'a mstr>),
    List(Vec<List<'a>>),
    Compound(Vec<Compound<'a>>),
    IntArray(Vec<UnalignedSlice<'a, i32>>),
    LongArray(Vec<UnalignedSlice<'a, i64>>),
}

impl<'a> Nbt<'a> {
    pub fn read(reader: &'a mut impl Reader<'a>) -> Result<Self, NbtReadError> {
        let root_tag: u8 = reader.read_slice(1)?[0];
        if root_tag != TAG_COMPOUND {
            return Err(NbtReadError::InvalidRootTag(root_tag));
        }

        let name: &mstr = read_str(reader)?;
        let tags: Compound = Compound::read(reader)?;

        Ok(Nbt { name, tags })
    }

    pub fn read_nameless(reader: &'a mut impl Reader<'a>) -> Result<Self, NbtReadError> {
        let root_tag: u8 = reader.read_u8()?;
        if root_tag != TAG_COMPOUND {
            return Err(NbtReadError::InvalidRootTag(root_tag));
        }

        let name: &mstr = mstr::new();
        let tags: Compound = Compound::read(reader)?;

        Ok(Nbt { name, tags })
    }
}

impl<'a> Compound<'a> {
    fn read<'r>(reader: &'r mut impl Reader<'a>) -> Result<Self, NbtReadError> {
        let mut tags: Vec<(&mstr, Tag)> = Vec::new();

        let mut tag_id: u8 = reader.read_u8()?;
        while tag_id != TAG_END {
            let name: &mstr = read_str(reader)?;
            let tag: Tag = Tag::read(reader, tag_id)?;
            tags.push((name, tag));
            tag_id = reader.read_u8()?;
        }

        Ok(Compound { tags })
    }
}

impl<'a> Tag<'a> {
    fn read<'r>(reader: &'r mut impl Reader<'a>, tag_id: u8) -> Result<Self, NbtReadError> {
        Ok(match tag_id {
            TAG_BYTE => Tag::Byte(reader.read_u8()?),
            TAG_SHORT => Tag::Short(reader.read_i16()?),
            TAG_INT => Tag::Int(reader.read_i32()?),
            TAG_LONG => Tag::Long(reader.read_i64()?),
            TAG_FLOAT => Tag::Float(reader.read_f32()?),
            TAG_DOUBLE => Tag::Double(reader.read_f64()?),
            TAG_BYTE_ARRAY => Tag::ByteArray(read_slice(reader)?),
            TAG_STRING => Tag::String(read_str(reader)?),
            TAG_LIST => Tag::List(List::read(reader)?),
            TAG_COMPOUND => Tag::Compound(Compound::read(reader)?),
            TAG_INT_ARRAY => Tag::IntArray(read_slice(reader)?),
            TAG_LONG_ARRAY => Tag::LongArray(read_slice(reader)?),
            tag_id => return Err(NbtReadError::InvalidTag(tag_id)),
        })
    }
}

impl<'a> List<'a> {
    pub fn read<'r>(reader: &'r mut impl Reader<'a>) -> Result<Self, NbtReadError> {
        let tag_id: u8 = reader.read_u8()?;
        Ok(match tag_id {
            TAG_BYTE => List::Byte(read_slice(reader)?),
            TAG_SHORT => List::Short(read_slice(reader)?),
            TAG_INT => List::Int(read_slice(reader)?),
            TAG_LONG => List::Long(read_slice(reader)?),
            TAG_FLOAT => List::Float(read_slice(reader)?),
            TAG_DOUBLE => List::Double(read_slice(reader)?),
            TAG_BYTE_ARRAY => {
                let len: u32 = reader.read_u32()?;
                let mut buf: Vec<UnalignedSlice<u8>> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(read_slice(reader)?);
                }
                List::ByteArray(buf)
            }
            TAG_STRING => {
                let len: u32 = reader.read_u32()?;
                let mut buf: Vec<&mstr> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(read_str(reader)?);
                }
                List::String(buf)
            }
            TAG_LIST => {
                let len: u32 = reader.read_u32()?;
                let mut buf: Vec<List> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(List::read(reader)?);
                }
                List::List(buf)
            }
            TAG_COMPOUND => {
                let len: u32 = reader.read_u32()?;
                let mut buf: Vec<Compound> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(Compound::read(reader)?);
                }
                List::Compound(buf)
            }
            TAG_INT_ARRAY => {
                let len: u32 = reader.read_u32()?;
                let mut buf: Vec<UnalignedSlice<'a, i32>> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(read_slice(reader)?);
                }
                List::IntArray(buf)
            }
            TAG_LONG_ARRAY => {
                let len: u32 = reader.read_u32()?;
                let mut buf: Vec<UnalignedSlice<'a, i64>> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(read_slice(reader)?);
                }
                List::LongArray(buf)
            }
            tag_id => return Err(NbtReadError::InvalidTag(tag_id)),
        })
    }
}

fn read_str<'a>(reader: &mut impl Reader<'a>) -> Result<&'a mstr, NbtReadError> {
    let len: u16 = reader.read_u16()?;
    let buf: &[u8] = reader.read_slice(len as usize)?;
    Ok(mstr::from_slice(buf))
}

fn read_slice<'a, T: Pod>(
    reader: &mut impl Reader<'a>,
) -> Result<UnalignedSlice<'a, T>, NbtReadError> {
    let len: u16 = reader.read_u16()?;
    let buf: &[u8] = reader.read_slice(len as usize * size_of::<T>())?;
    Ok(UnalignedSlice::new(
        buf.as_ptr() as *const T,
        buf.len() / size_of::<T>(),
    ))
}
