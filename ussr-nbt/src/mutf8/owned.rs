use std::fmt::Debug;

use super::MString;

impl MString {
    pub const fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub const fn from_vec(vec: Vec<u8>) -> Self {
        Self { vec }
    }
}

impl Debug for MString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m\"{}\"", String::from_utf8_lossy(&self.vec))
    }
}

impl Default for MString {
    fn default() -> Self {
        Self::new()
    }
}
