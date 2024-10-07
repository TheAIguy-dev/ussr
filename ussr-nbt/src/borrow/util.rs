use std::io;

use super::{reader::Reader, writer::Writer};
use crate::{endian::RawSlice, mutf8::mstr, num::Num, NbtReadError};

#[inline]
pub(super) fn read_str<'a>(reader: &mut impl Reader<'a>) -> Result<&'a mstr, NbtReadError> {
    let len: u16 = reader.read_u16()?;
    let buf: &[u8] = reader.read_slice(len as usize)?;
    Ok(mstr::from_mutf8(buf))
}

#[inline]
pub(super) fn read_slice_with_len<'a, T: Num>(
    reader: &mut impl Reader<'a>,
) -> Result<RawSlice<'a, T>, NbtReadError> {
    let len: i32 = reader.read_i32()?;

    if len <= 0 {
        return Ok(RawSlice::new());
    }

    read_slice(reader, len as usize)
}

#[inline]
pub(super) fn read_byte_slice_with_len<'a>(reader: &mut impl Reader<'a>) -> io::Result<&'a [u8]> {
    let len: i32 = reader.read_i32()?;

    if len <= 0 {
        return Ok(&[]);
    }

    reader.read_slice(len as usize)
}

#[inline]
pub(super) fn read_slice<'a, T: Num>(
    reader: &mut impl Reader<'a>,
    len: usize,
) -> Result<RawSlice<'a, T>, NbtReadError> {
    let buf: &[u8] = reader.read_slice(len * size_of::<T>())?;
    Ok(RawSlice::from_bytes(buf))
}

#[inline]
pub(super) fn write_str(writer: &mut impl Writer, str: &mstr) {
    let len: u16 = str.len().min(u16::MAX as usize) as u16;
    writer.write_u16(len);
    writer.write_slice(&str.as_bytes()[..len as usize]);
}

#[inline]
pub(super) fn write_slice<T: Num>(writer: &mut impl Writer, slice: RawSlice<T>) {
    let len: i32 = slice.len().min(i32::MAX as usize) as i32;
    writer.write_i32(len);
    writer.write_slice(&slice.to_bytes()[..len as usize * size_of::<T>()]);
}

macro_rules! impl_tag {
    ($name:ident, $( $(@$deref:tt)? + )? $type:ty) => {
        paste! {
            #[must_use]
            #[inline]
            pub const fn $name(&self) -> Option<$type> {
                match self {
                    Tag::[< $name:camel >](val) => Some(impl_tag!(@internal { $( $($deref)? + )? } { val } { *val })),
                    _ => None,
                }
            }

            impl_tag!(@internal { $( $($deref)? + )? } {} {
                #[inline]
                pub fn [< $name _mut >](&mut self) -> Option<&mut $type> {
                    match self {
                        Tag::[< $name:camel >](val) => Some(val),
                        _ => None,
                    }
                }

                #[must_use]
                #[inline]
                pub fn [< into_ $name >](self) -> Option<$type> {
                    match self {
                        Tag::[< $name:camel >](val) => Some(val),
                        _ => None,
                    }
                }
            });

        }
    };
    ( @internal {   } { $($then:tt)* } { $($else:tt)* } ) => { $($then)* };
    ( @internal { + } { $($then:tt)* } { $($else:tt)* } ) => { $($else)* };
}
pub(super) use impl_tag;

macro_rules! impl_list {
    ($name:ident, $( $(@$deref:tt)? + )? $type:ty, $new:expr) => {
        paste! {
            #[must_use]
            #[inline]
            pub const fn [< $name s >](&self) -> Option<$type> {
                match self {
                    List::[< $name:camel >](val) => Some(impl_list!(@internal { $( $($deref)? + )? } { val } { *val })),
                    List::Empty => Some(const { $new }),
                    _ => None,
                }
            }
        }
    };
    ( @internal {   } { $($then:tt)* } { $($else:tt)* } ) => { $($then)* };
    ( @internal { + } { $($then:tt)* } { $($else:tt)* } ) => { $($else)* };
}
pub(super) use impl_list;
