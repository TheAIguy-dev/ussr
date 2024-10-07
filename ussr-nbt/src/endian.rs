//! Lazy wrappers around collections that contain information about endianness.

use std::{borrow::Cow, fmt::Debug};

use bytemuck::{cast_slice, cast_slice_mut, zeroed_vec};

use crate::{num::Num, swap_endian::swap_endian};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Endian<B, N> {
    Big(B),
    Native(N),
}

/// A lazy wrapper around `Vec<T>` that contains information about endianness.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RawVec<T: Num> {
    // Vec<T> is used in big endian to have it well-aligned
    vec: Endian<Vec<T>, Vec<T>>,
}

/// A lazy wrapper around `&[T]` that contains information about endianness.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RawSlice<'a, T: Num> {
    slice: Endian<&'a [u8], &'a [T]>,
}

impl<B, N> Debug for Endian<B, N> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endian::Big(_) => write!(f, "Big"),
            Endian::Native(_) => write!(f, "Native"),
        }
    }
}

impl<T: Num> RawVec<T> {
    /// Creates an empty vector.
    /// Does not allocate.
    #[must_use]
    #[inline]
    pub const fn new() -> RawVec<T> {
        RawVec {
            vec: Endian::Native(Vec::new()),
        }
    }

    /// Creates a `RawVec` from a vector in native endian.
    #[must_use]
    #[inline]
    pub const fn from_vec(vec: Vec<T>) -> RawVec<T> {
        RawVec {
            //? Maybe it's better to swap endianness right away?
            //? If so, then we don't even need [`Endian`] here.
            vec: Endian::Native(vec),
        }
    }

    #[inline]
    pub(crate) const fn from_big(vec: Vec<T>) -> RawVec<T> {
        RawVec {
            vec: Endian::Big(vec),
        }
    }

    /// Returns the number of elements of type `T` in the vector.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        match &self.vec {
            Endian::Big(vec) | Endian::Native(vec) => vec.len(),
        }
    }

    /// Returns `true` if the vector contains no elements.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Converts the vector into a big endian byte slice.
    #[must_use]
    #[inline]
    pub fn to_bytes(&self) -> Cow<[u8]> {
        match &self.vec {
            Endian::Big(vec) => Cow::Borrowed(cast_slice(vec)),
            Endian::Native(vec) => {
                #[cfg(target_endian = "little")]
                {
                    let mut vec: Vec<u8> = cast_slice(vec).to_owned();
                    swap_endian(cast_slice_mut::<u8, T>(&mut vec));
                    Cow::Owned(vec)
                }
                #[cfg(not(target_endian = "little"))]
                Cow::Borrowed(cast_slice(vec))
            }
        }
    }

    /// Creates a copy of the vector and converts it to native endian.
    #[must_use]
    #[inline]
    pub fn to_vec(&self) -> Vec<T> {
        self.clone().into_vec()
    }

    /// Converts the vector into native endian.
    #[must_use]
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        match self.vec {
            Endian::Big(mut vec) => {
                #[cfg(target_endian = "little")]
                swap_endian(&mut vec);
                vec
            }
            Endian::Native(vec) => vec,
        }
    }
}

impl<T: Num> Default for RawVec<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Num> From<Vec<T>> for RawVec<T> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}

impl<T: Num> From<RawVec<T>> for Vec<T> {
    #[inline]
    fn from(vec: RawVec<T>) -> Self {
        vec.into_vec()
    }
}

impl<T: Num + Debug> Debug for RawVec<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.to_vec().iter()).finish()
    }
}

impl<'a, T: Num> RawSlice<'a, T> {
    /// Creates an empty slice.
    #[must_use]
    #[inline]
    pub const fn new() -> RawSlice<'static, T> {
        RawSlice {
            slice: Endian::Native(&[]),
        }
    }

    /// Creates a `RawSlice` from a slice in native endian.
    #[inline]
    pub const fn from_slice(slice: &'a [T]) -> RawSlice<'a, T> {
        RawSlice {
            slice: Endian::Native(slice),
        }
    }

    #[inline]
    pub(crate) const fn from_bytes(slice: &'a [u8]) -> RawSlice<'a, T> {
        RawSlice {
            slice: Endian::Big(slice),
        }
    }

    /// Returns the number of elements of type `T` in the slice.
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        match self.slice {
            Endian::Big(slice) => slice.len() / size_of::<T>(),
            Endian::Native(slice) => slice.len(),
        }
    }

    /// Returns `true` if the slice contains no elements.
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Converts the slice into a native endian slice of `T`.
    #[must_use]
    #[inline]
    pub fn to_slice(&self) -> Cow<'a, [T]> {
        match self.slice {
            Endian::Big(slice) => {
                #[cfg(target_endian = "little")]
                {
                    let mut vec: Vec<T> = zeroed_vec(slice.len() / size_of::<T>());
                    cast_slice_mut(&mut vec).copy_from_slice(slice);
                    swap_endian(&mut vec);
                    Cow::Owned(vec)
                }
                #[cfg(not(target_endian = "little"))]
                Cow::Borrowed(cast_slice(slice))
            }
            Endian::Native(slice) => Cow::Borrowed(slice),
        }
    }

    /// Converts the slice into a big endian byte slice.
    #[must_use]
    #[inline]
    pub fn to_bytes(&self) -> Cow<'a, [u8]> {
        match self.slice {
            Endian::Big(slice) => Cow::Borrowed(slice),
            Endian::Native(slice) => {
                let mut vec: Vec<u8> = cast_slice(slice).to_owned();
                #[cfg(target_endian = "little")]
                swap_endian(cast_slice_mut::<u8, T>(&mut vec));
                Cow::Owned(vec)
            }
        }
    }

    /// Converts the slice into a raw vector.
    /// Does not swap endianness.
    // This is basically [`ToOwned::to_owned`], but we can't actually implement it because [`Clone`] is implemented.
    #[must_use]
    #[inline]
    pub fn to_raw_vec(&self) -> RawVec<T> {
        match self.slice {
            Endian::Big(slice) => {
                let mut vec: Vec<T> = zeroed_vec(slice.len() / size_of::<T>());
                cast_slice_mut(&mut vec).copy_from_slice(slice);
                RawVec::from_big(vec)
            }
            Endian::Native(slice) => RawVec::from_vec(slice.to_vec()),
        }
    }
}

impl<'a, T: Num> Default for RawSlice<'a, T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: Num> From<&'a [T]> for RawSlice<'a, T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<'a, T: Num> From<RawSlice<'a, T>> for Cow<'a, [T]> {
    #[inline]
    fn from(slice: RawSlice<'a, T>) -> Self {
        slice.to_slice()
    }
}

impl<T: Num + Debug> Debug for RawSlice<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.to_slice().iter()).finish()
    }
}
