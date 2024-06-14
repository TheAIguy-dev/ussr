use std::io::Read;

use paste::paste;

use crate::{size::MAX_STRING_LENGTH, ReadError, ReadExt, Readable, VarReadable};

macro_rules! impl_readable {
    ($($type:ty),*) => {
        paste! {
            $(
                impl Readable for $type {
                    #[inline]
                    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
                        Ok(reader.[<read_ $type>]()?)
                    }
                }
            )*
        }
    };
}
impl_readable!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

/// Reads a string from the reader, with a maximum length `max_length`.
///
/// `max_length` is in characters, not bytes.
pub fn read_string(reader: &mut impl Read, max_length: usize) -> Result<String, ReadError> {
    let length: usize = usize::read_var_from(reader)?;

    if length as usize > max_length * 3 {
        return Err(ReadError::InvalidStringLength {
            max: max_length * 3,
            actual: length as usize,
        });
    }

    let mut bytes: Vec<u8> = vec![0; length as usize];
    reader.read_exact(&mut bytes)?;
    // TODO: make a fast utf-8 validator
    Ok(String::from_utf8(bytes)?)
}

impl Readable for bool {
    #[inline]
    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        Ok(reader.read_u8()? != 0)
    }
}

impl VarReadable for u32 {
    fn read_var_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        let mut value: u32 = 0;
        for i in 0..5 {
            let byte: u8 = reader.read_u8()?;
            value |= (byte as u32 & 0x7F) << (i * 7);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(ReadError::InvalidVarInt)
    }
}

impl VarReadable for i32 {
    #[inline]
    fn read_var_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        Ok(u32::read_var_from(reader)? as i32)
    }
}

impl VarReadable for u64 {
    fn read_var_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        let mut value: u64 = 0;
        for i in 0..10 {
            let byte: u8 = reader.read_u8()?;
            value |= (byte as u64 & 0x7F) << (i * 7);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(ReadError::InvalidVarLong)
    }
}

impl VarReadable for i64 {
    #[inline]
    fn read_var_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        Ok(u64::read_var_from(reader)? as i64)
    }
}

impl VarReadable for usize {
    /// Limited to 3 bytes because lengths can only have that many.
    fn read_var_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        let mut value: usize = 0;
        for i in 0..3 {
            let byte: u8 = reader.read_u8()?;
            value |= (byte as usize & 0x7F) << (i * 7);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(ReadError::InvalidVarInt)
    }
}

impl Readable for String {
    #[inline]
    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        read_string(reader, MAX_STRING_LENGTH)
    }
}
