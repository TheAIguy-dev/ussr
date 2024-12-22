#![doc = include_str!("../README.md")]
#![warn(
    clippy::all,
    clippy::cargo,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]
#![allow(
    clippy::use_self,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

pub mod borrow;
pub mod num;
pub mod owned;
mod swap_endian;

use std::io;

use thiserror::Error;

// TODO: make borrow actually usable :/
// TODO: could do a ToReader trait and use a single reader type, since all of them must be contiguous anyway
// TODO: macros to generate Decode and Encode for nbt types
// TODO: const nbt (for fun)
//? remove arrays? they are equivalent to lists and are just a hassle

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

/// Errors that can occur while decoding NBT data.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum NbtDecodeError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Root tag not a compound: {0}")]
    InvalidRootTag(u8),

    #[error("Invalid tag: {0}")]
    InvalidTag(u8),

    #[error("Depth limit exceeded")]
    DepthLimitExceeded,

    #[error("Invalid MUTF-8")]
    InvalidMutf8,
}

/// Options for decoding NBT data.
#[derive(Clone, Copy)]
pub struct DecodeOpts {
    /// The maximum depth to decode.
    /// Defaults to `128`.
    pub depth_limit: u16,

    /// Whether the root compound has name or not.
    /// Defaults to `true`.
    pub named: bool,
}

impl Default for DecodeOpts {
    #[inline]
    fn default() -> Self {
        DecodeOpts::new(128, true)
    }
}

impl DecodeOpts {
    #[must_use]
    #[inline]
    pub const fn new(depth_limit: u16, named: bool) -> DecodeOpts {
        DecodeOpts { depth_limit, named }
    }

    /// A shorthand for nameless NBT with other options defaulted.
    #[must_use]
    #[inline]
    pub const fn nameless() -> DecodeOpts {
        DecodeOpts {
            named: false,
            depth_limit: 128,
        }
    }

    #[must_use]
    #[inline]
    pub const fn with_depth_limit(mut self, depth_limit: u16) -> DecodeOpts {
        self.depth_limit = depth_limit;
        self
    }

    #[must_use]
    #[inline]
    pub const fn with_name(mut self, named: bool) -> DecodeOpts {
        self.named = named;
        self
    }
}

/// Options for encoding NBT data.
#[derive(Clone, Copy)]
pub struct EncodeOpts {
    /// Whether to encode the root compound name or not.
    /// Defaults to `true`.
    pub named: bool,
}

impl Default for EncodeOpts {
    #[inline]
    fn default() -> Self {
        EncodeOpts::new(true)
    }
}

impl EncodeOpts {
    #[must_use]
    #[inline]
    pub const fn new(named: bool) -> EncodeOpts {
        EncodeOpts { named }
    }

    /// A shorthand for nameless NBT with other options defaulted.
    #[must_use]
    #[inline]
    pub const fn nameless() -> EncodeOpts {
        EncodeOpts { named: false }
    }

    #[must_use]
    #[inline]
    pub const fn with_name(mut self, named: bool) -> EncodeOpts {
        self.named = named;
        self
    }
}
