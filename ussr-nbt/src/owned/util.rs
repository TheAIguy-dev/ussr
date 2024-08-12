use std::io::{self, Read, Write};

use bytemuck::{cast_slice, cast_slice_mut, zeroed_vec};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};

use crate::{
    endian::RawVec,
    mutf8::{mstr, MString},
    num::Num,
    NbtReadError,
};

#[inline]
pub(super) fn read_string(reader: &mut impl Read) -> Result<MString, NbtReadError> {
    let len: u16 = reader.read_u16::<BE>()?;
    let mut buf: Vec<u8> = vec![0; len as usize];
    reader.read_exact(&mut buf)?;
    Ok(MString::from_mutf8(buf))
}

#[inline]
pub(super) fn read_vec_with_len<T: Num>(reader: &mut impl Read) -> Result<RawVec<T>, NbtReadError> {
    let len: i32 = reader.read_i32::<BE>()?;

    if len <= 0 {
        return Ok(RawVec::new());
    }

    read_vec(reader, len as usize)
}

#[inline]
pub(super) fn read_byte_vec_with_len(reader: &mut impl Read) -> Result<Vec<u8>, NbtReadError> {
    let len: i32 = reader.read_i32::<BE>()?;

    if len <= 0 {
        return Ok(Vec::new());
    }

    read_byte_vec(reader, len as usize)
}

#[inline]
pub(super) fn read_vec<T: Num>(
    reader: &mut impl Read,
    len: usize,
) -> Result<RawVec<T>, NbtReadError> {
    let mut buf: Vec<T> = zeroed_vec(len);
    reader.read_exact(cast_slice_mut(&mut buf))?;

    Ok(RawVec::from_big(buf))
}

#[inline]
pub(super) fn read_byte_vec(reader: &mut impl Read, len: usize) -> Result<Vec<u8>, NbtReadError> {
    let mut buf: Vec<u8> = vec![0; len];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

#[inline]
pub(super) fn write_string(writer: &mut impl Write, str: &mstr) -> io::Result<()> {
    let len: u16 = str.len().min(u16::MAX as usize) as u16;
    writer.write_u16::<BE>(len)?;
    writer.write_all(&str.as_bytes()[..len as usize])
}

#[inline]
pub(super) fn write_vec<T: Num>(writer: &mut impl Write, vec: &RawVec<T>) -> io::Result<()> {
    let len: i32 = vec.len().min(i32::MAX as usize) as i32;
    writer.write_i32::<BE>(len)?;
    writer.write_all(cast_slice(&vec.to_bytes()[..len as usize * size_of::<T>()]))
}

#[inline]
pub(super) fn write_byte_vec(writer: &mut impl Write, vec: &[u8]) -> io::Result<()> {
    let len: i32 = vec.len().min(i32::MAX as usize) as i32;
    writer.write_i32::<BE>(len)?;
    writer.write_all(&vec[..len as usize])
}

macro_rules! impl_tag {
    ($name:ident, $( $(@$deref:tt)? + )? $type:ty) => {
        paste! {
            #[inline]
            pub fn $name(&self) -> Option<impl_tag!(@internal { $( $($deref)? + )? } { &$type } { $type })> {
                match self {
                    Tag::[< $name:camel >](val) => Some(impl_tag!(@internal { $( $($deref)? + )? } { val } { *val })),
                    _ => None,
                }
            }

            #[inline]
            pub fn [< $name _mut >](&mut self) -> Option<&mut $type> {
                match self {
                    Tag::[< $name:camel >](val) => Some(val),
                    _ => None,
                }
            }

            #[inline]
            pub fn [< into_ $name >](self) -> Option<$type> {
                match self {
                    Tag::[< $name:camel >](val) => Some(val),
                    _ => None,
                }
            }
        }
    };
    ( @internal {   } { $($then:tt)* } { $($else:tt)* } ) => { $($then)* };
    ( @internal { + } { $($then:tt)* } { $($else:tt)* } ) => { $($else)* };
}
pub(super) use impl_tag;

macro_rules! impl_list {
    ($name:ident, $type:ty) => {
        paste! {
            #[inline]
            pub fn [< $name s >](&self) -> Option<&$type> {
                match self {
                    List::[< $name:camel >](val) => Some(val),
                    List::Empty => Some(const { &$type::new() }),
                    _ => None,
                }
            }

            #[inline]
            pub fn [< $name s _mut >](&mut self) -> Option<&mut $type> {
                match self {
                    List::[< $name:camel >](val) => Some(val),
                    _ => None,
                }
            }

            #[inline]
            pub fn [< into_ $name s >](self) -> Option<$type> {
                match self {
                    List::[< $name:camel >](val) => Some(val),
                    _ => None,
                }
            }
        }
    };
}
pub(super) use impl_list;
