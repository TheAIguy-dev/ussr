mod io_ext;
mod read;
mod write;

use std::{
    io::{self, Read, Write},
    string::FromUtf8Error,
};

use thiserror::Error;

pub use io_ext::*;

//? Maybe I don't need to implement Error.
#[derive(Debug, Error)]
pub enum ReadError {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error("Invalid VarInt")]
    InvalidVarInt,

    #[error("Invalid VarLong")]
    InvalidVarLong,

    #[error("Invalid UTF-8")]
    InvalidUtf8(#[from] FromUtf8Error),

    #[error("Invalid string length: max {max}, got {actual}")]
    InvalidStringLength { max: usize, actual: usize },

    #[error("Invalid enum variant")]
    InvalidEnumVariant,
    // #[error("Parse error: {0}")]
    // ParseError(&'static str),

    // #[error("Connection closed")]
    // ConnectionClosed,
}

/// A trait for reading types from a buffer.
pub trait Readable: Sized {
    fn read_from(buf: &mut impl Read) -> Result<Self, ReadError>;
}

/// A trait for reading variable-length types from a buffer.
pub trait VarReadable: Sized {
    fn read_var_from(buf: &mut impl Read) -> Result<Self, ReadError>;
}

/// A trait for writing types to a buffer.
pub trait Writable: Sized {
    fn write_to(&self, buf: &mut impl Write) -> io::Result<()>;
}

/// A trait for writing variable-length types to a buffer.
pub trait VarWritable: Sized {
    fn write_var_to(&self, buf: &mut impl Write) -> io::Result<()>;
}
