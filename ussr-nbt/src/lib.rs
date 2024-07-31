pub mod borrow;
pub mod endian;
pub mod mutf8;
pub mod owned;

use std::io;

use thiserror::Error;

// TODO: Writing
// TODO: Endianness
// TODO: Depth limits
// TODO: Serde support
// TODO: Empty lists
// TODO: Docs and examples
//? Signed lengths

const TAG_END: u8 = 0;
const TAG_BYTE: u8 = 1;
const TAG_SHORT: u8 = 2;
const TAG_INT: u8 = 3;
const TAG_LONG: u8 = 4;
const TAG_FLOAT: u8 = 5;
const TAG_DOUBLE: u8 = 6;
const TAG_BYTE_ARRAY: u8 = 7;
const TAG_STRING: u8 = 8;
const TAG_LIST: u8 = 9;
const TAG_COMPOUND: u8 = 10;
const TAG_INT_ARRAY: u8 = 11;
const TAG_LONG_ARRAY: u8 = 12;

#[derive(Debug, Error)]
pub enum NbtReadError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Invalid root tag: {0}")]
    InvalidRootTag(u8),

    #[error("Invalid tag: {0}")]
    InvalidTag(u8),
}
