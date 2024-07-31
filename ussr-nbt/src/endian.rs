use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Endian {
    Big,
    Native,
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct UnalignedSlice<'a, T> {
    endian: Endian,
    ptr: *const T,
    len: usize,
    marker: PhantomData<&'a T>,
}

impl<T> UnalignedSlice<'_, T> {
    pub const fn new(ptr: *const T, len: usize) -> Self {
        Self {
            endian: Endian::Big,
            ptr,
            len,
            marker: PhantomData,
        }
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        Self {
            endian: Endian::Native,
            ptr: slice.as_ptr(),
            len: slice.len(),
            marker: PhantomData,
        }
    }
}

impl<T> From<&[T]> for UnalignedSlice<'_, T> {
    fn from(value: &[T]) -> Self {
        Self::from_slice(value)
    }
}

impl<T> Debug for UnalignedSlice<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UnalignedSlice(endian={:?}, len={:?})",
            self.endian, self.len
        )
    }
}
