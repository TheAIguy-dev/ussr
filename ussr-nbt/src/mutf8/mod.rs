mod borrow;
mod owned;

/// An owned MUTF-8 string.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MString {
    vec: Vec<u8>,
}

/// A MUTF-8 string slice.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct mstr {
    slice: [u8],
}
