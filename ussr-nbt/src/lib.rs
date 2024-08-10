#![doc = include_str!("../README.md")]

pub mod borrow;
pub mod endian;
pub mod mutf8;
pub mod num;
pub mod owned;
mod swap_endian;

use std::io;

use thiserror::Error;

pub const TAG_END: u8 = 0;
pub const TAG_BYTE: u8 = 1;
pub const TAG_SHORT: u8 = 2;
pub const TAG_INT: u8 = 3;
pub const TAG_LONG: u8 = 4;
pub const TAG_FLOAT: u8 = 5;
pub const TAG_DOUBLE: u8 = 6;
pub const TAG_BYTE_ARRAY: u8 = 7;
pub const TAG_STRING: u8 = 8;
pub const TAG_LIST: u8 = 9;
pub const TAG_COMPOUND: u8 = 10;
pub const TAG_INT_ARRAY: u8 = 11;
pub const TAG_LONG_ARRAY: u8 = 12;

/// Errors that can occur while reading NBT data.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum NbtReadError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Root tag not a compound: {0}")]
    InvalidRootTag(u8),

    #[error("Invalid tag: {0}")]
    InvalidTag(u8),

    #[error("Depth limit exceeded")]
    DepthLimitExceeded,
}

/// Options for reading NBT data.
//? Automatically handle endianess?
//? Check for duplicate keys?
pub struct ReadOpts {
    /// The maximum depth to read.
    /// Defaults to `128`.
    pub depth_limit: u16,

    /// Whether to read the root compound name or not.
    /// Defaults to `true`.
    pub name: bool,
}

/// Options for writing NBT data.
pub struct WriteOpts {
    /// Whether to write the root compound name or not.
    /// Defaults to `true`.
    pub name: bool,
}

impl Default for ReadOpts {
    #[inline]
    fn default() -> Self {
        Self {
            depth_limit: 128,
            name: true,
        }
    }
}

impl ReadOpts {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_depth_limit(mut self, depth_limit: u16) -> Self {
        self.depth_limit = depth_limit;
        self
    }

    #[inline]
    pub fn with_name(mut self, name: bool) -> Self {
        self.name = name;
        self
    }
}

impl Default for WriteOpts {
    #[inline]
    fn default() -> Self {
        Self { name: true }
    }
}

impl WriteOpts {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_name(mut self, name: bool) -> Self {
        self.name = name;
        self
    }
}
