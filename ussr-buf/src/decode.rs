use std::io::Read;

use byteorder::{ReadBytesExt, BE};
use paste::paste;
use ussr_nbt::{owned::Nbt, DecodeOpts};
use uuid::Uuid;

use crate::{DecodeError, MAX_STRING_LENGTH};

pub trait Decode: Sized {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError>;
}

pub trait VarDecode: Sized {
    fn var_decode(reader: &mut impl Read) -> Result<Self, DecodeError>;
}

pub trait DecodeExt {
    fn decode<T: Decode>(&mut self) -> Result<T, DecodeError>;
    fn var_decode<T: VarDecode>(&mut self) -> Result<T, DecodeError>;
}

impl<R: Read> DecodeExt for R {
    fn decode<T: Decode>(&mut self) -> Result<T, DecodeError> {
        T::decode(self)
    }
    fn var_decode<T: VarDecode>(&mut self) -> Result<T, DecodeError> {
        T::var_decode(self)
    }
}

/// Decodes a string from the reader with a maximum length.
///
/// `max_length` is in characters, not bytes.
/// In practice, the maximum length of the string in bytes is `max_length * 3` (plus 3 bytes for the length).
pub fn decode_string(reader: &mut impl Read, max_length: usize) -> Result<String, DecodeError> {
    let length: usize = usize::var_decode(reader)?;

    if length > max_length * 3 {
        return Err(DecodeError::InvalidStringLength(length));
    }

    let mut bytes: Vec<u8> = vec![0; length];
    reader.read_exact(&mut bytes)?;

    // TODO: make a fast utf-8 validator
    Ok(String::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8)?)
}

macro_rules! impl_decode {
    ($($type:ty),*) => {
        paste! {
            $(
                impl Decode for $type {
                    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
                        Ok(reader.[<read_ $type>]::<BE>()?)
                    }
                }
            )*
        }
    };
}
impl_decode!(u16, u32, u64, u128, i16, i32, i64, i128, f32, f64);

impl Decode for u8 {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(reader.read_u8()?)
    }
}

impl Decode for i8 {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(reader.read_i8()?)
    }
}

impl Decode for bool {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(reader.read_u8()? != 0)
    }
}

impl VarDecode for u32 {
    fn var_decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        let mut value: u32 = 0;

        for i in 0..5 {
            let byte: u8 = reader.read_u8()?;
            value |= (byte as u32 & 0x7F) << (i * 7);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }

        Err(DecodeError::InvalidVarInt)
    }
}

impl VarDecode for i32 {
    fn var_decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(u32::var_decode(reader)? as i32)
    }
}

impl VarDecode for u64 {
    fn var_decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        let mut value: u64 = 0;

        for i in 0..10 {
            let byte: u8 = reader.read_u8()?;
            value |= (byte as u64 & 0x7F) << (i * 7);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }

        Err(DecodeError::InvalidVarLong)
    }
}

impl VarDecode for i64 {
    fn var_decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(u64::var_decode(reader)? as i64)
    }
}

impl VarDecode for usize {
    /// Convenience implementation. Uses `u32::var_decode` (VarInt).
    fn var_decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(u32::var_decode(reader)? as usize)
    }
}

impl Decode for String {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        decode_string(reader, MAX_STRING_LENGTH)
    }
}

impl Decode for Nbt {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(Nbt::read_with_opts(reader, DecodeOpts::nameless())?)
    }
}

impl Decode for Uuid {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(Uuid::from_u128(reader.read_u128::<BE>()?))
    }
}

impl<T: Decode> Decode for Vec<T> {
    /// Will use `VarReadable` for the length and `Readable` for the elements.
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok((0..usize::var_decode(reader)?)
            .map(|_| T::decode(reader))
            .collect::<Result<_, _>>()?)
    }
}

impl<T: Decode> Decode for Option<T> {
    /// Will decode a `bool` followed by a `T` if the `bool` is `true`.
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(bool::decode(reader)?
            .then(|| T::decode(reader))
            .transpose()?)
    }
}
