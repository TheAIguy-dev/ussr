use std::io::Read;

use byteorder::{ReadBytesExt, BE};
use paste::paste;
use ussr_nbt::owned::Nbt;
#[cfg(feature = "uuid")]
use uuid::Uuid;

use crate::{size::MAX_STRING_LENGTH, ReadError, Readable, VarReadable};

macro_rules! impl_readable {
    ($($type:ty),*) => {
        paste! {
            $(
                impl Readable for $type {
                    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
                        Ok(reader.[<read_ $type>]::<BE>()?)
                    }
                }
            )*
        }
    };
}
impl_readable!(u16, u32, u64, u128, i16, i32, i64, i128, f32, f64);

/// Reads a string from the reader, with a maximum length `max_length`.
///
/// `max_length` is in characters, not bytes.
pub fn read_string(reader: &mut impl Read, max_length: usize) -> Result<String, ReadError> {
    let length: usize = usize::read_var_from(reader)?;

    if length > max_length * 3 {
        return Err(ReadError::InvalidStringLength {
            max: max_length * 3,
            actual: length,
        });
    }

    let mut bytes: Vec<u8> = vec![0; length];
    reader.read_exact(&mut bytes)?;
    // TODO: make a fast utf-8 validator
    Ok(String::from_utf8(bytes).map_err(|_| ReadError::InvalidUtf8)?)
}

/// Read an array from the reader, prefixed with its length as a fixed-size readable type `L`.
pub fn read_array<L, T>(reader: &mut impl Read) -> Result<Vec<T>, ReadError>
where
    L: Readable + Into<usize>,
    T: Readable,
{
    let length: usize = L::read_from(reader)?.into();
    let mut array: Vec<T> = Vec::with_capacity(length);
    for _ in 0..length {
        array.push(T::read_from(reader)?);
    }
    Ok(array)
}

/// Read an array from the reader, prefixed with its length as a variable-sized readable type `L`.
pub fn read_var_array<L, T>(reader: &mut impl Read) -> Result<Vec<T>, ReadError>
where
    L: VarReadable + Into<usize>,
    T: VarReadable,
{
    let length: usize = L::read_var_from(reader)?.into();
    let mut array: Vec<T> = Vec::with_capacity(length);
    for _ in 0..length {
        array.push(T::read_var_from(reader)?);
    }
    Ok(array)
}

impl Readable for u8 {
    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        Ok(reader.read_u8()?)
    }
}

impl Readable for i8 {
    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        Ok(reader.read_i8()?)
    }
}

impl Readable for bool {
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
    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        read_string(reader, MAX_STRING_LENGTH)
    }
}

impl Readable for Nbt {
    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        Ok(Nbt::read(reader)?)
    }
}

#[cfg(feature = "uuid")]
impl Readable for Uuid {
    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError> {
        Ok(Uuid::from_u128(reader.read_u128::<BE>()?))
    }
}
