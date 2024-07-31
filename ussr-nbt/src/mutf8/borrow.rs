use std::fmt::Debug;

use super::mstr;

impl mstr {
    pub const fn new() -> &'static Self {
        return Self::from_slice(&[]);
    }

    pub const fn from_slice(slice: &[u8]) -> &Self {
        // SAFETY: `mstr` is a transparent wrapper around `[u8]`
        unsafe { &*(slice as *const [u8] as *const mstr) }
    }
}

impl Debug for mstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m\"{}\"", String::from_utf8_lossy(&self.slice))
    }
}

impl Default for &mstr {
    fn default() -> Self {
        mstr::new()
    }
}
