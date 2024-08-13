use super::reader::Reader;
use crate::*;

#[inline]
pub fn read_compound<'a>(
    reader: &mut impl Reader<'a>,
    len: &mut usize,
) -> Result<(), NbtReadError> {
    let mut tag_id: u8 = reader.read_u8()?;
    while tag_id != TAG_END {
        *len += size_of::<u8>();
        read_string(reader, len)?;
        read_tag(reader, len, tag_id)?;
        tag_id = reader.read_u8()?;
    }
    *len += size_of::<u8>();
    Ok(())
}

#[inline]
pub fn read_tag<'a>(
    reader: &mut impl Reader<'a>,
    len: &mut usize,
    tag_id: u8,
) -> Result<(), NbtReadError> {
    match tag_id {
        TAG_BYTE => {
            reader.read_u8()?;
            *len += size_of::<u8>();
        }
        TAG_SHORT => {
            reader.read_i16()?;
            *len += size_of::<i16>();
        }
        TAG_INT => {
            reader.read_i32()?;
            *len += size_of::<i32>();
        }
        TAG_LONG => {
            reader.read_i64()?;
            *len += size_of::<i64>();
        }
        TAG_FLOAT => {
            reader.read_f32()?;
            *len += size_of::<f32>();
        }
        TAG_DOUBLE => {
            reader.read_f64()?;
            *len += size_of::<f64>();
        }
        TAG_BYTE_ARRAY => {
            read_with_i32_len(reader, len, size_of::<u8>())?;
        }
        TAG_STRING => {
            read_string(reader, len)?;
        }
        TAG_LIST => read_list(reader, len)?,
        TAG_COMPOUND => read_compound(reader, len)?,
        TAG_INT_ARRAY => {
            read_with_i32_len(reader, len, size_of::<i32>())?;
        }
        TAG_LONG_ARRAY => {
            read_with_i32_len(reader, len, size_of::<i64>())?;
        }
        _ => return Err(NbtReadError::InvalidTag(tag_id)),
    }
    Ok(())
}

#[inline]
pub fn read_list<'a>(reader: &mut impl Reader<'a>, len: &mut usize) -> Result<(), NbtReadError> {
    let tag_id: u8 = reader.read_u8()?;
    *len += size_of::<u8>();
    let length: i32 = reader.read_i32()?;
    *len += size_of::<i32>();

    if length <= 0 {
        return Ok(());
    }
    let length: usize = length as usize;

    match tag_id {
        TAG_BYTE => read(reader, len, length, size_of::<u8>()),
        TAG_SHORT => read(reader, len, length, size_of::<i16>()),
        TAG_INT => read(reader, len, length, size_of::<i32>()),
        TAG_LONG => read(reader, len, length, size_of::<i64>()),
        TAG_FLOAT => read(reader, len, length, size_of::<f32>()),
        TAG_DOUBLE => read(reader, len, length, size_of::<f64>()),
        TAG_BYTE_ARRAY => {
            for _ in 0..length {
                read_with_i32_len(reader, len, size_of::<u8>())?;
            }
            Ok(())
        }
        TAG_STRING => {
            for _ in 0..length {
                read_string(reader, len)?;
            }
            Ok(())
        }
        TAG_LIST => {
            for _ in 0..length {
                read_list(reader, len)?;
            }
            Ok(())
        }
        TAG_COMPOUND => read_compound(reader, len),
        TAG_INT_ARRAY => {
            for _ in 0..length {
                read_with_i32_len(reader, len, size_of::<i32>())?;
            }
            Ok(())
        }
        TAG_LONG_ARRAY => {
            for _ in 0..length {
                read_with_i32_len(reader, len, size_of::<i64>())?;
            }
            Ok(())
        }
        _ => Err(NbtReadError::InvalidTag(tag_id)),
    }
}

#[inline]
pub fn read_string<'a>(reader: &mut impl Reader<'a>, len: &mut usize) -> Result<(), NbtReadError> {
    let length: u16 = reader.read_u16()?;
    reader.read_slice(length as usize)?;
    *len += size_of::<u16>() + length as usize;
    Ok(())
}

#[inline]
pub fn read_with_i32_len<'a>(
    reader: &mut impl Reader<'a>,
    len: &mut usize,
    size: usize,
) -> Result<(), NbtReadError> {
    let length: i32 = reader.read_i32()?.max(0);
    read(reader, len, length as usize, size)
}

#[inline]
pub fn read<'a>(
    reader: &mut impl Reader<'a>,
    len: &mut usize,
    amount: usize,
    size: usize,
) -> Result<(), NbtReadError> {
    reader.read_slice(amount * size)?;
    *len += amount;
    Ok(())
}
