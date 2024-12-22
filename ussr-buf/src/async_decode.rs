use std::future::Future;

use futures_lite::AsyncReadExt;
use paste::paste;
use ussr_nbt::owned::Nbt;
use uuid::Uuid;

use crate::{DecodeError, MAX_STRING_LENGTH};

pub trait Decode: Sized {
    fn decode(
        reader: &mut (impl AsyncReadExt + Unpin + Send),
    ) -> impl Future<Output = Result<Self, DecodeError>> + Send;
}

pub trait VarDecode: Sized {
    fn var_decode(
        reader: &mut (impl AsyncReadExt + Unpin + Send),
    ) -> impl Future<Output = Result<Self, DecodeError>> + Send;
}

pub trait DecodeExt {
    fn decode<T: Decode>(&mut self) -> impl Future<Output = Result<T, DecodeError>> + Send;
    fn var_decode<T: VarDecode>(&mut self) -> impl Future<Output = Result<T, DecodeError>> + Send;
}

impl<R: AsyncReadExt + Unpin + Send> DecodeExt for R {
    fn decode<T: Decode>(&mut self) -> impl Future<Output = Result<T, DecodeError>> + Send {
        T::decode(self)
    }
    fn var_decode<T: VarDecode>(&mut self) -> impl Future<Output = Result<T, DecodeError>> + Send {
        T::var_decode(self)
    }
}

/// Decodes a string from the reader with a maximum length.
///
/// `max_length` is in characters, not bytes.
/// In practice, the maximum length of the string in bytes is `max_length * 3` (plus 3 bytes for the length).
pub async fn decode_string(
    reader: &mut (impl AsyncReadExt + Unpin + Send),
    max_length: usize,
) -> Result<String, DecodeError> {
    let length: usize = usize::var_decode(reader).await?;

    if length > max_length * 3 {
        return Err(DecodeError::InvalidStringLength(length));
    }

    let mut bytes: Vec<u8> = vec![0; length];
    reader.read_exact(&mut bytes).await?;

    // TODO: make a fast utf-8 validator
    Ok(String::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8)?)
}

macro_rules! impl_decode {
    ($($type:ty),*) => {
        paste! {
            $(
                impl Decode for $type {
                    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
                        let mut buf = [0; size_of::<$type>()];
                        reader.read_exact(&mut buf).await?;
                        Ok(<$type>::from_be_bytes(buf))
                    }
                }
            )*
        }
    };
}
impl_decode!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl Decode for bool {
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        Ok(u8::decode(reader).await? != 0)
    }
}

impl VarDecode for u32 {
    async fn var_decode(
        reader: &mut (impl AsyncReadExt + Unpin + Send),
    ) -> Result<Self, DecodeError> {
        let mut value: u32 = 0;

        for i in 0..5 {
            let byte: u8 = u8::decode(reader).await?;
            value |= (byte as u32 & 0x7F) << (i * 7);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }

        Err(DecodeError::InvalidVarInt)
    }
}

impl VarDecode for i32 {
    async fn var_decode(
        reader: &mut (impl AsyncReadExt + Unpin + Send),
    ) -> Result<Self, DecodeError> {
        Ok(u32::var_decode(reader).await? as i32)
    }
}

impl VarDecode for u64 {
    async fn var_decode(
        reader: &mut (impl AsyncReadExt + Unpin + Send),
    ) -> Result<Self, DecodeError> {
        let mut value: u64 = 0;

        for i in 0..10 {
            let byte: u8 = u8::decode(reader).await?;
            value |= (byte as u64 & 0x7F) << (i * 7);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }

        Err(DecodeError::InvalidVarLong)
    }
}

impl VarDecode for i64 {
    async fn var_decode(
        reader: &mut (impl AsyncReadExt + Unpin + Send),
    ) -> Result<Self, DecodeError> {
        Ok(u64::var_decode(reader).await? as i64)
    }
}

impl VarDecode for usize {
    /// Convenience implementation. Uses `u32::var_decode` (VarInt).
    async fn var_decode(
        reader: &mut (impl AsyncReadExt + Unpin + Send),
    ) -> Result<Self, DecodeError> {
        Ok(u32::var_decode(reader).await? as usize)
    }
}

impl Decode for String {
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        decode_string(reader, MAX_STRING_LENGTH).await
    }
}

impl Decode for Nbt {
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        todo!()
    }
}

impl Decode for Uuid {
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        Ok(Uuid::from_u128(u128::decode(reader).await?))
    }
}

impl<T: Decode + Send> Decode for Vec<T> {
    /// Will use `VarReadable` for the length and `Readable` for the elements.
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        let length: usize = usize::var_decode(reader).await?;
        let mut buf: Vec<T> = Vec::with_capacity(length);

        for _ in 0..length {
            buf.push(T::decode(reader).await?);
        }

        Ok(buf)
    }
}

impl<T: Decode + Send> Decode for Option<T> {
    async fn decode(reader: &mut (impl AsyncReadExt + Unpin + Send)) -> Result<Self, DecodeError> {
        if bool::decode(reader).await? {
            Ok(Some(T::decode(reader).await?))
        } else {
            Ok(None)
        }
    }
}
