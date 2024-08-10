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
pub(super) fn write_slice<'a, T: Num>(writer: &mut impl Writer, slice: RawSlice<'a, T>) {
    let len: i32 = (slice.len() as i32).min(i32::MAX);
    writer.write_i32(len);
    writer.write_slice(&slice.to_bytes()[..len as usize * size_of::<T>()]);
}
