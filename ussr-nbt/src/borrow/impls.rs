use std::borrow::Cow;

use super::{Compound, List, Tag};
use crate::{endian::RawSlice, mutf8::mstr};

impl TryInto<u8> for Tag<'_> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Tag::Byte(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryInto<i16> for Tag<'_> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<i16, Self::Error> {
        match self {
            Tag::Short(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryInto<i32> for Tag<'_> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Tag::Int(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryInto<i64> for Tag<'_> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Tag::Long(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryInto<f32> for Tag<'_> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Tag::Float(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryInto<f64> for Tag<'_> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Tag::Double(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<&'a [u8]> for Tag<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<&'a [u8], Self::Error> {
        match self {
            Tag::ByteArray(value) => Ok(value),
            Tag::List(List::Byte(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(&[]),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<&'a mstr> for Tag<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<&'a mstr, Self::Error> {
        match self {
            Tag::String(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<List<'a>> for Tag<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<List<'a>, Self::Error> {
        match self {
            Tag::List(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Compound<'a>> for Tag<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Compound<'a>, Self::Error> {
        match self {
            Tag::Compound(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<RawSlice<'a, i32>> for Tag<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawSlice<'a, i32>, Self::Error> {
        match self {
            Tag::IntArray(value) => Ok(value),
            Tag::List(List::Int(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(RawSlice::new()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Cow<'a, [i32]>> for Tag<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Cow<'a, [i32]>, Self::Error> {
        match self {
            Tag::IntArray(value) => Ok(value.into()),
            Tag::List(List::Int(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok((&[]).into()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<RawSlice<'a, i64>> for Tag<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawSlice<'a, i64>, Self::Error> {
        match self {
            Tag::LongArray(value) => Ok(value),
            Tag::List(List::Long(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(RawSlice::new()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Cow<'a, [i64]>> for Tag<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Cow<'a, [i64]>, Self::Error> {
        match self {
            Tag::LongArray(value) => Ok(value.into()),
            Tag::List(List::Long(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok((&[]).into()),
            _ => Err(()),
        }
    }
}

impl<'a> TryInto<&'a [u8]> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<&'a [u8], Self::Error> {
        match self {
            List::Byte(value) => Ok(value),
            List::Empty => Ok(&[]),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<RawSlice<'a, i16>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawSlice<'a, i16>, Self::Error> {
        match self {
            List::Short(value) => Ok(value),
            List::Empty => Ok(RawSlice::new()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Cow<'a, [i16]>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Cow<'a, [i16]>, Self::Error> {
        match self {
            List::Short(value) => Ok(value.into()),
            List::Empty => Ok((&[]).into()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<RawSlice<'a, i32>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawSlice<'a, i32>, Self::Error> {
        match self {
            List::Int(value) => Ok(value),
            List::Empty => Ok(RawSlice::new()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Cow<'a, [i32]>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Cow<'a, [i32]>, Self::Error> {
        match self {
            List::Int(value) => Ok(value.into()),
            List::Empty => Ok((&[]).into()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<RawSlice<'a, i64>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawSlice<'a, i64>, Self::Error> {
        match self {
            List::Long(value) => Ok(value),
            List::Empty => Ok(RawSlice::new()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Cow<'a, [i64]>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Cow<'a, [i64]>, Self::Error> {
        match self {
            List::Long(value) => Ok(value.into()),
            List::Empty => Ok((&[]).into()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<RawSlice<'a, f32>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawSlice<'a, f32>, Self::Error> {
        match self {
            List::Float(value) => Ok(value),
            List::Empty => Ok(RawSlice::new()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Cow<'a, [f32]>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Cow<'a, [f32]>, Self::Error> {
        match self {
            List::Float(value) => Ok(value.into()),
            List::Empty => Ok((&[]).into()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<RawSlice<'a, f64>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawSlice<'a, f64>, Self::Error> {
        match self {
            List::Double(value) => Ok(value),
            List::Empty => Ok(RawSlice::new()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Cow<'a, [f64]>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Cow<'a, [f64]>, Self::Error> {
        match self {
            List::Double(value) => Ok(value.into()),
            List::Empty => Ok((&[]).into()),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Vec<&'a mstr>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<&'a mstr>, Self::Error> {
        match self {
            List::String(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Vec<List<'a>>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<List<'a>>, Self::Error> {
        match self {
            List::List(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Vec<Compound<'a>>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<Compound<'a>>, Self::Error> {
        match self {
            List::Compound(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Vec<RawSlice<'a, i32>>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<RawSlice<'a, i32>>, Self::Error> {
        match self {
            List::IntArray(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Vec<Cow<'a, [i32]>>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<Cow<'a, [i32]>>, Self::Error> {
        match self {
            List::IntArray(value) => Ok(value.into_iter().map(|s| s.to_slice()).collect()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Vec<RawSlice<'a, i64>>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<RawSlice<'a, i64>>, Self::Error> {
        match self {
            List::LongArray(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<Vec<Cow<'a, [i64]>>> for List<'a> {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<Cow<'a, [i64]>>, Self::Error> {
        match self {
            List::LongArray(value) => Ok(value.into_iter().map(|s| s.to_slice()).collect()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
