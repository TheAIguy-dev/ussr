//! Support for modified UTF-8 encoding of strings.

mod impls;

use std::borrow::Cow;

use simd_cesu8::{mutf8, DecodingError};

/// An owned MUTF-8 string.
/// Note that it is not validated during parsing, and will only be validated when converting to a string.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MString {
    vec: Vec<u8>,
}

/// A MUTF-8 string slice.
/// Note that it is not validated during parsing, and will only be validated when converting to a string.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct mstr {
    slice: [u8],
}

impl MString {
    /// Creates an empty string.
    /// Does not allocate.
    #[must_use]
    #[inline]
    pub const fn new() -> MString {
        MString { vec: Vec::new() }
    }

    #[inline]
    pub(crate) const fn from_mutf8(vec: Vec<u8>) -> MString {
        MString { vec }
    }

    /// Creates a new `MString` from a `String`.
    #[must_use]
    #[inline]
    pub fn from_string(string: String) -> MString {
        MString {
            vec: match mutf8::encode(&string) {
                Cow::Borrowed(_) => string.into_bytes(),
                Cow::Owned(vec) => vec,
            },
        }
    }
}

impl mstr {
    /// Creates an empty string slice.
    #[must_use]
    #[inline]
    pub const fn new() -> &'static mstr {
        mstr::from_mutf8(&[])
    }

    /// Creates a new `mstr` from a string slice.
    #[must_use]
    #[inline]
    pub fn from_string(string: &str) -> Cow<mstr> {
        match mutf8::encode(string) {
            Cow::Borrowed(slice) => Cow::Borrowed(mstr::from_mutf8(slice)),
            Cow::Owned(vec) => Cow::Owned(MString::from_mutf8(vec)),
        }
    }

    /// Returns the number of bytes in the slice.
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.slice.len()
    }

    /// Returns `true` if the slice is empty.
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }

    /// Returns the underlying byte slice.
    #[must_use]
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.slice
    }

    #[inline]
    pub(crate) const fn from_mutf8(slice: &[u8]) -> &mstr {
        unsafe { std::mem::transmute(slice) }
    }

    /// Decodes the string slice into UTF-8.
    #[inline]
    pub fn decode(&self) -> Result<Cow<str>, DecodingError> {
        mutf8::decode(&self.slice)
    }
}
