pub mod read;
pub mod size;
pub mod write;

use std::io::{self, Read, Write};

use thiserror::Error;
use ussr_nbt::NbtReadError;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Invalid VarInt")]
    InvalidVarInt,

    #[error("Invalid VarLong")]
    InvalidVarLong,

    #[error("Invalid UTF-8")]
    InvalidUtf8,

    #[error("Invalid string length")]
    InvalidStringLength { max: usize, actual: usize },

    #[error("Invalid enum variant")]
    InvalidEnumVariant,

    #[error(transparent)]
    Nbt(NbtReadError),
}

impl From<NbtReadError> for ReadError {
    fn from(e: NbtReadError) -> Self {
        match e {
            NbtReadError::Io(e) => Self::Io(e),
            e => Self::Nbt(e),
        }
    }
}

/// NOTE: This trait will likely be removed.
/// A trait for getting the size of a type in bytes when serialized.
pub trait Size {
    const SIZE: usize;
}

/// NOTE: This trait will likely be removed.
/// A trait for getting the size of a variable-length type in bytes when serialized.
pub trait VarSize {
    const MIN_SIZE: usize;
    const MAX_SIZE: usize;
}

/// A trait for reading types from a reader.
pub trait Readable: Sized {
    fn read_from(reader: &mut impl Read) -> Result<Self, ReadError>;
}

/// A trait for reading variable-length types from a reader.
pub trait VarReadable: Sized {
    fn read_var_from(buf: &mut impl Read) -> Result<Self, ReadError>;
}

/// A trait for writing types to a writer.
pub trait Writable: Sized {
    fn write_to(&self, writer: &mut impl Write) -> io::Result<()>;
}

/// A trait for writing variable-length types to a writer.
pub trait VarWritable: Sized {
    fn write_var_to(&self, writer: &mut impl Write) -> io::Result<()>;
}
