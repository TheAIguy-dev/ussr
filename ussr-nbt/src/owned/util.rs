use std::io::Read;

use byteorder::{ReadBytesExt, BE};

use crate::{NbtReadError, *};

#[inline]
pub(super) fn read_compound(
    reader: &mut impl Read,
    data: &mut Vec<u8>,
) -> Result<(), NbtReadError> {
    let mut tag_id: u8 = reader.read_u8()?;
    while tag_id != TAG_END {
        data.push(tag_id);
        read_string(reader, data)?;
        read_tag(reader, data, tag_id)?;
        tag_id = reader.read_u8()?;
    }
    data.push(TAG_END);
    Ok(())
}

#[inline]
pub(super) fn read_tag(
    reader: &mut impl Read,
    data: &mut Vec<u8>,
    tag_id: u8,
) -> Result<(), NbtReadError> {
    match tag_id {
        TAG_BYTE => {
            data.extend_from_slice(&reader.read_u8()?.to_be_bytes());
        }
        TAG_SHORT => {
            data.extend_from_slice(&reader.read_i16::<BE>()?.to_be_bytes());
        }
        TAG_INT => {
            data.extend_from_slice(&reader.read_i32::<BE>()?.to_be_bytes());
        }
        TAG_LONG => {
            data.extend_from_slice(&reader.read_i64::<BE>()?.to_be_bytes());
        }
        TAG_FLOAT => {
            data.extend_from_slice(&reader.read_f32::<BE>()?.to_be_bytes());
        }
        TAG_DOUBLE => {
            data.extend_from_slice(&reader.read_f64::<BE>()?.to_be_bytes());
        }
        TAG_BYTE_ARRAY => {
            read_with_i32_len(reader, data, size_of::<u8>())?;
        }
        TAG_STRING => {
            read_string(reader, data)?;
        }
        TAG_LIST => read_list(reader, data)?,
        TAG_COMPOUND => read_compound(reader, data)?,
        TAG_INT_ARRAY => {
            read_with_i32_len(reader, data, size_of::<i32>())?;
        }
        TAG_LONG_ARRAY => {
            read_with_i32_len(reader, data, size_of::<i64>())?;
        }
        _ => return Err(NbtReadError::InvalidTag(tag_id)),
    }
    Ok(())
}

#[inline]
pub(super) fn read_list(reader: &mut impl Read, data: &mut Vec<u8>) -> Result<(), NbtReadError> {
    let tag_id: u8 = reader.read_u8()?;
    data.push(tag_id);
    let len: i32 = reader.read_i32::<BE>()?;
    data.extend_from_slice(&len.to_be_bytes());

    if len <= 0 {
        return Ok(());
    }
    let len: usize = len as usize;

    match tag_id {
        TAG_BYTE => read(reader, data, len, size_of::<u8>()),
        TAG_SHORT => read(reader, data, len, size_of::<i16>()),
        TAG_INT => read(reader, data, len, size_of::<i32>()),
        TAG_LONG => read(reader, data, len, size_of::<i64>()),
        TAG_FLOAT => read(reader, data, len, size_of::<f32>()),
        TAG_DOUBLE => read(reader, data, len, size_of::<f64>()),
        TAG_BYTE_ARRAY => {
            for _ in 0..len {
                read_with_i32_len(reader, data, size_of::<u8>())?;
            }
            Ok(())
        }
        TAG_STRING => {
            for _ in 0..len {
                read_string(reader, data)?;
            }
            Ok(())
        }
        TAG_LIST => {
            for _ in 0..len {
                read_list(reader, data)?;
            }
            Ok(())
        }
        TAG_COMPOUND => read_compound(reader, data),
        TAG_INT_ARRAY => {
            for _ in 0..len {
                read_with_i32_len(reader, data, size_of::<i32>())?;
            }
            Ok(())
        }
        TAG_LONG_ARRAY => {
            for _ in 0..len {
                read_with_i32_len(reader, data, size_of::<i64>())?;
            }
            Ok(())
        }
        _ => Err(NbtReadError::InvalidTag(tag_id)),
    }
}

#[inline]
pub(super) fn read_string(reader: &mut impl Read, data: &mut Vec<u8>) -> Result<(), NbtReadError> {
    let len: u16 = reader.read_u16::<BE>()?;
    data.extend_from_slice(&len.to_be_bytes());
    read(reader, data, len as usize, 1)
}

#[inline]
pub(super) fn read_with_i32_len(
    reader: &mut impl Read,
    data: &mut Vec<u8>,
    size: usize,
) -> Result<(), NbtReadError> {
    let len: i32 = reader.read_i32::<BE>()?.max(0);
    data.extend_from_slice(&len.to_be_bytes());
    read(reader, data, len as usize, size)
}

#[inline]
pub(super) fn read(
    reader: &mut impl Read,
    data: &mut Vec<u8>,
    amount: usize,
    size: usize,
) -> Result<(), NbtReadError> {
    let data_len: usize = data.len();
    data.resize(data_len + amount as usize * size, 0);
    reader.read_exact(&mut data[data_len..])?;
    Ok(())
}
