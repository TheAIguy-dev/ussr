use std::{
    borrow::{Borrow, Cow},
    fmt::{Debug, Display},
    ops::Deref,
};

use super::{mstr, MString};

impl Deref for MString {
    type Target = mstr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        mstr::from_mutf8(&self.vec)
    }
}

impl AsRef<mstr> for MString {
    #[inline]
    fn as_ref(&self) -> &mstr {
        mstr::from_mutf8(&self.vec)
    }
}

impl Borrow<mstr> for MString {
    #[inline]
    fn borrow(&self) -> &mstr {
        mstr::from_mutf8(&self.vec)
    }
}

impl From<String> for MString {
    #[inline]
    fn from(value: String) -> Self {
        Self::from_string(value)
    }
}

impl From<&str> for MString {
    #[inline]
    fn from(value: &str) -> Self {
        Self::from_string(value.to_string())
    }
}

impl TryFrom<MString> for String {
    type Error = ();

    #[inline]
    fn try_from(value: MString) -> Result<Self, Self::Error> {
        value.decode().map(|s: Cow<str>| s.to_string())
    }
}

impl Debug for MString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "m\"{}\"",
            self.decode().unwrap_or(String::from_utf8_lossy(&self.vec))
        )
    }
}

impl Display for MString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.decode().unwrap_or(String::from_utf8_lossy(&self.vec)))
    }
}

impl Default for MString {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq<Vec<u8>> for MString {
    #[inline]
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.vec == *other
    }
}

impl PartialEq<&[u8]> for MString {
    #[inline]
    fn eq(&self, other: &&[u8]) -> bool {
        self.vec == *other
    }
}

impl PartialEq<mstr> for MString {
    #[inline]
    fn eq(&self, other: &mstr) -> bool {
        self.vec == other.slice
    }
}

impl PartialEq<&mstr> for MString {
    #[inline]
    fn eq(&self, other: &&mstr) -> bool {
        self.vec == other.slice
    }
}

impl PartialEq<[u8]> for MString {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.vec == other
    }
}

impl ToOwned for mstr {
    type Owned = MString;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        MString::from_mutf8(self.slice.to_vec())
    }
}

impl Debug for mstr {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "m\"{}\"",
            &self
                .decode()
                .unwrap_or(String::from_utf8_lossy(&self.slice))
        )
    }
}

impl Display for mstr {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(
            &self
                .decode()
                .unwrap_or(String::from_utf8_lossy(&self.slice)),
        )
    }
}

impl Default for &mstr {
    #[inline]
    fn default() -> Self {
        mstr::new()
    }
}

impl<'a> TryFrom<&'a mstr> for Cow<'a, str> {
    type Error = ();

    #[inline]
    fn try_from(value: &'a mstr) -> Result<Self, Self::Error> {
        value.decode()
    }
}

impl PartialEq<[u8]> for mstr {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.slice == *other
    }
}

impl PartialEq<&[u8]> for mstr {
    #[inline]
    fn eq(&self, other: &&[u8]) -> bool {
        self.slice == **other
    }
}

impl PartialEq<&mstr> for mstr {
    #[inline]
    fn eq(&self, other: &&mstr) -> bool {
        self.slice == other.slice
    }
}

impl PartialEq<Vec<u8>> for mstr {
    #[inline]
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.slice == *other
    }
}

impl PartialEq<Vec<u8>> for &mstr {
    #[inline]
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.slice == *other
    }
}

impl Deref for mstr {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}

impl AsRef<[u8]> for mstr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.slice
    }
}

impl Borrow<[u8]> for mstr {
    #[inline]
    fn borrow(&self) -> &[u8] {
        &self.slice
    }
}
