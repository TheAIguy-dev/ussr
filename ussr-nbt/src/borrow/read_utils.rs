use super::reader::Reader;
use crate::NbtDecodeError;

#[inline]
pub fn read_list<'a, T>(reader: &mut impl Reader<'a>) -> Result<*const u8, NbtDecodeError> {
    let ptr: *const u8 = unsafe { reader.ptr() };
    let len: usize = reader.read_i32()?.max(0) as usize;
    reader.read_slice(len * size_of::<T>())?;
    Ok(ptr)
}

#[inline]
pub fn read_string<'a>(reader: &mut impl Reader<'a>) -> Result<*const u8, NbtDecodeError> {
    let ptr: *const u8 = unsafe { reader.ptr() };
    let len: usize = reader.read_u16()? as usize;
    reader.read_slice(len * size_of::<u8>())?;
    Ok(ptr)
}
