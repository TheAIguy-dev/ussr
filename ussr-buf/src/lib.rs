#[cfg(feature = "async")]
pub mod async_decode;
#[cfg(feature = "async")]
pub mod async_encode;
pub mod decode;
pub mod encode;

use std::io;

use thiserror::Error;
use ussr_nbt::NbtDecodeError;

#[cfg(feature = "async")]
pub use async_decode::{
    decode_string as async_decode_string, Decode as AsyncDecode, DecodeExt as AsyncDecodeExt,
    VarDecode as AsyncVarDecode,
};
#[cfg(feature = "async")]
pub use async_encode::{
    Encode as AsyncEncode, EncodeExt as AsyncEncodeExt, VarEncode as AsyncVarEncode,
};
pub use decode::*;
pub use encode::*;
#[cfg(all(feature = "derive", feature = "async"))]
pub use ussr_buf_derive::{AsyncDecode, AsyncEncode};
#[cfg(feature = "derive")]
pub use ussr_buf_derive::{Decode, Encode};

/// The maximum length of a string in characters.
pub const MAX_STRING_LENGTH: usize = 32767;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Invalid VarInt")]
    InvalidVarInt,

    #[error("Invalid VarLong")]
    InvalidVarLong,

    #[error("Invalid UTF-8")]
    InvalidUtf8,

    #[error("Invalid string length")]
    InvalidStringLength(usize),

    #[error("Invalid enum variant")]
    InvalidEnumVariant,

    #[error("Error reading NBT: {0}")]
    Nbt(NbtDecodeError),
}

impl From<NbtDecodeError> for DecodeError {
    fn from(e: NbtDecodeError) -> DecodeError {
        match e {
            NbtDecodeError::Io(e) => DecodeError::Io(e),
            e => DecodeError::Nbt(e),
        }
    }
}
