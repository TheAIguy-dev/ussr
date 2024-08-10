use std::collections::HashMap;

use super::{Compound, List, Tag};
use crate::{endian::RawVec, mutf8::MString};

impl From<u8> for Tag {
    #[inline]
    fn from(value: u8) -> Self {
        Tag::Byte(value)
    }
}

impl From<i16> for Tag {
    #[inline]
    fn from(value: i16) -> Self {
        Tag::Short(value)
    }
}

impl From<i32> for Tag {
    #[inline]
    fn from(value: i32) -> Self {
        Tag::Int(value)
    }
}

impl From<i64> for Tag {
    #[inline]
    fn from(value: i64) -> Self {
        Tag::Long(value)
    }
}

impl From<f32> for Tag {
    #[inline]
    fn from(value: f32) -> Self {
        Tag::Float(value)
    }
}

impl From<f64> for Tag {
    #[inline]
    fn from(value: f64) -> Self {
        Tag::Double(value)
    }
}

impl From<Vec<u8>> for Tag {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        Tag::ByteArray(value)
    }
}

impl<S: Into<MString>> From<S> for Tag {
    #[inline]
    fn from(value: S) -> Self {
        Tag::String(value.into())
    }
}

impl From<List> for Tag {
    #[inline]
    fn from(value: List) -> Self {
        Tag::List(value)
    }
}

impl From<RawVec<i16>> for Tag {
    #[inline]
    fn from(value: RawVec<i16>) -> Self {
        Tag::List(List::Short(value))
    }
}

impl From<Vec<i16>> for Tag {
    #[inline]
    fn from(value: Vec<i16>) -> Self {
        Tag::List(List::Short(value.into()))
    }
}

impl From<RawVec<f32>> for Tag {
    #[inline]
    fn from(value: RawVec<f32>) -> Self {
        Tag::List(List::Float(value))
    }
}

impl From<Vec<f32>> for Tag {
    #[inline]
    fn from(value: Vec<f32>) -> Self {
        Tag::List(List::Float(value.into()))
    }
}

impl From<RawVec<f64>> for Tag {
    #[inline]
    fn from(value: RawVec<f64>) -> Self {
        Tag::List(List::Double(value))
    }
}

impl From<Vec<f64>> for Tag {
    #[inline]
    fn from(value: Vec<f64>) -> Self {
        Tag::List(List::Double(value.into()))
    }
}

impl From<Vec<Vec<u8>>> for Tag {
    #[inline]
    fn from(value: Vec<Vec<u8>>) -> Self {
        Tag::List(List::ByteArray(value))
    }
}

impl<S: Into<MString>> From<Vec<S>> for Tag {
    #[inline]
    fn from(value: Vec<S>) -> Self {
        Tag::List(List::String(value.into_iter().map(Into::into).collect()))
    }
}

impl From<Vec<List>> for Tag {
    #[inline]
    fn from(value: Vec<List>) -> Self {
        Tag::List(List::List(value))
    }
}

impl<S: Into<MString>> From<HashMap<S, Tag>> for Tag {
    #[inline]
    fn from(value: HashMap<S, Tag>) -> Self {
        Tag::Compound(value.into())
    }
}

impl<S: Into<MString>> From<Vec<(S, Tag)>> for Tag {
    #[inline]
    fn from(value: Vec<(S, Tag)>) -> Self {
        Tag::Compound(value.into())
    }
}

impl<S: Into<MString>> From<Vec<HashMap<S, Tag>>> for Tag {
    #[inline]
    fn from(value: Vec<HashMap<S, Tag>>) -> Self {
        Tag::List(List::Compound(value.into_iter().map(Into::into).collect()))
    }
}

impl<S: Into<MString>> From<Vec<Vec<(S, Tag)>>> for Tag {
    #[inline]
    fn from(value: Vec<Vec<(S, Tag)>>) -> Self {
        Tag::List(List::Compound(value.into_iter().map(Into::into).collect()))
    }
}

impl From<Vec<Vec<i32>>> for Tag {
    #[inline]
    fn from(value: Vec<Vec<i32>>) -> Self {
        Tag::List(List::IntArray(value.into_iter().map(Into::into).collect()))
    }
}

impl From<Vec<RawVec<i32>>> for Tag {
    #[inline]
    fn from(value: Vec<RawVec<i32>>) -> Self {
        Tag::List(List::IntArray(value))
    }
}

impl From<Vec<Vec<i64>>> for Tag {
    #[inline]
    fn from(value: Vec<Vec<i64>>) -> Self {
        Tag::List(List::LongArray(value.into_iter().map(Into::into).collect()))
    }
}

impl From<Vec<RawVec<i64>>> for Tag {
    #[inline]
    fn from(value: Vec<RawVec<i64>>) -> Self {
        Tag::List(List::LongArray(value))
    }
}

impl From<Compound> for Tag {
    #[inline]
    fn from(value: Compound) -> Self {
        Tag::Compound(value)
    }
}

impl From<RawVec<i32>> for Tag {
    #[inline]
    fn from(value: RawVec<i32>) -> Self {
        Tag::IntArray(value)
    }
}

impl From<Vec<i32>> for Tag {
    #[inline]
    fn from(value: Vec<i32>) -> Self {
        Tag::IntArray(value.into())
    }
}

impl From<RawVec<i64>> for Tag {
    #[inline]
    fn from(value: RawVec<i64>) -> Self {
        Tag::LongArray(value)
    }
}

impl From<Vec<i64>> for Tag {
    #[inline]
    fn from(value: Vec<i64>) -> Self {
        Tag::LongArray(value.into())
    }
}

impl From<Vec<u8>> for List {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        List::Byte(value)
    }
}

impl From<RawVec<i16>> for List {
    #[inline]
    fn from(value: RawVec<i16>) -> Self {
        List::Short(value)
    }
}

impl From<Vec<i16>> for List {
    #[inline]
    fn from(value: Vec<i16>) -> Self {
        List::Short(value.into())
    }
}

impl From<RawVec<i32>> for List {
    #[inline]
    fn from(value: RawVec<i32>) -> Self {
        List::Int(value)
    }
}

impl From<Vec<i32>> for List {
    #[inline]
    fn from(value: Vec<i32>) -> Self {
        List::Int(value.into())
    }
}

impl From<RawVec<i64>> for List {
    #[inline]
    fn from(value: RawVec<i64>) -> Self {
        List::Long(value)
    }
}

impl From<Vec<i64>> for List {
    #[inline]
    fn from(value: Vec<i64>) -> Self {
        List::Long(value.into())
    }
}

impl From<RawVec<f32>> for List {
    #[inline]
    fn from(value: RawVec<f32>) -> Self {
        List::Float(value)
    }
}

impl From<Vec<f32>> for List {
    #[inline]
    fn from(value: Vec<f32>) -> Self {
        List::Float(value.into())
    }
}

impl From<RawVec<f64>> for List {
    #[inline]
    fn from(value: RawVec<f64>) -> Self {
        List::Double(value)
    }
}

impl From<Vec<f64>> for List {
    #[inline]
    fn from(value: Vec<f64>) -> Self {
        List::Double(value.into())
    }
}

impl From<Vec<Vec<u8>>> for List {
    #[inline]
    fn from(value: Vec<Vec<u8>>) -> Self {
        List::ByteArray(value)
    }
}

impl<S: Into<MString>> From<Vec<S>> for List {
    #[inline]
    fn from(value: Vec<S>) -> Self {
        List::String(value.into_iter().map(Into::into).collect())
    }
}

impl From<Vec<List>> for List {
    #[inline]
    fn from(value: Vec<List>) -> Self {
        List::List(value)
    }
}

impl From<Vec<Compound>> for List {
    #[inline]
    fn from(value: Vec<Compound>) -> Self {
        List::Compound(value)
    }
}

impl From<Vec<RawVec<i32>>> for List {
    #[inline]
    fn from(value: Vec<RawVec<i32>>) -> Self {
        List::IntArray(value)
    }
}

impl From<Vec<Vec<i32>>> for List {
    #[inline]
    fn from(value: Vec<Vec<i32>>) -> Self {
        List::IntArray(value.into_iter().map(Into::into).collect())
    }
}

impl From<Vec<RawVec<i64>>> for List {
    #[inline]
    fn from(value: Vec<RawVec<i64>>) -> Self {
        List::LongArray(value)
    }
}

impl From<Vec<Vec<i64>>> for List {
    #[inline]
    fn from(value: Vec<Vec<i64>>) -> Self {
        List::LongArray(value.into_iter().map(Into::into).collect())
    }
}

impl<S: Into<MString>> From<Vec<(S, Tag)>> for Compound {
    #[inline]
    fn from(value: Vec<(S, Tag)>) -> Self {
        Compound {
            tags: value.into_iter().map(|(k, v)| (k.into(), v)).collect(),
        }
    }
}

impl<S: Into<MString>> From<HashMap<S, Tag>> for Compound {
    #[inline]
    fn from(value: HashMap<S, Tag>) -> Self {
        Compound {
            tags: value.into_iter().map(|(k, v)| (k.into(), v)).collect(),
        }
    }
}

impl TryInto<u8> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Tag::Byte(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<i16> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<i16, Self::Error> {
        match self {
            Tag::Short(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<i32> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Tag::Int(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<i64> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Tag::Long(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<f32> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Tag::Float(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<f64> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Tag::Double(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<u8>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            Tag::ByteArray(value) => Ok(value),
            Tag::List(List::Byte(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<MString> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<MString, Self::Error> {
        match self {
            Tag::String(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<List> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<List, Self::Error> {
        match self {
            Tag::List(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<Compound> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Compound, Self::Error> {
        match self {
            Tag::Compound(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<i32>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<i32>, Self::Error> {
        match self {
            Tag::IntArray(value) => Ok(value),
            Tag::List(List::Int(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<i32>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<i32>, Self::Error> {
        match self {
            Tag::IntArray(value) => Ok(value.into()),
            Tag::List(List::Int(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<i64>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<i64>, Self::Error> {
        match self {
            Tag::LongArray(value) => Ok(value),
            Tag::List(List::Long(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<i64>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<i64>, Self::Error> {
        match self {
            Tag::LongArray(value) => Ok(value.into()),
            Tag::List(List::Long(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<i16>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<i16>, Self::Error> {
        match self {
            Tag::List(List::Short(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<i16>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<i16>, Self::Error> {
        match self {
            Tag::List(List::Short(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<f32>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<f32>, Self::Error> {
        match self {
            Tag::List(List::Float(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<f32>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<f32>, Self::Error> {
        match self {
            Tag::List(List::Float(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<f64>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<f64>, Self::Error> {
        match self {
            Tag::List(List::Double(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<f64>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<f64>, Self::Error> {
        match self {
            Tag::List(List::Double(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<MString>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<MString>, Self::Error> {
        match self {
            Tag::List(List::String(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<List>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<List>, Self::Error> {
        match self {
            Tag::List(List::List(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<Compound>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<Compound>, Self::Error> {
        match self {
            Tag::List(List::Compound(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<RawVec<i32>>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<RawVec<i32>>, Self::Error> {
        match self {
            Tag::List(List::IntArray(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<Vec<i32>>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<Vec<i32>>, Self::Error> {
        match self {
            Tag::List(List::IntArray(value)) => Ok(value.into_iter().map(Into::into).collect()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<RawVec<i64>>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<RawVec<i64>>, Self::Error> {
        match self {
            Tag::List(List::LongArray(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<Vec<i64>>> for Tag {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<Vec<i64>>, Self::Error> {
        match self {
            Tag::List(List::LongArray(value)) => Ok(value.into_iter().map(Into::into).collect()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<u8>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            List::Byte(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<i16>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<i16>, Self::Error> {
        match self {
            List::Short(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<i16>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<i16>, Self::Error> {
        match self {
            List::Short(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<i32>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<i32>, Self::Error> {
        match self {
            List::Int(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<i32>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<i32>, Self::Error> {
        match self {
            List::Int(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<i64>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<i64>, Self::Error> {
        match self {
            List::Long(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<i64>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<i64>, Self::Error> {
        match self {
            List::Long(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<f32>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<f32>, Self::Error> {
        match self {
            List::Float(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<f32>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<f32>, Self::Error> {
        match self {
            List::Float(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<RawVec<f64>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<RawVec<f64>, Self::Error> {
        match self {
            List::Double(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<f64>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<f64>, Self::Error> {
        match self {
            List::Double(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<MString>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<MString>, Self::Error> {
        match self {
            List::String(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<List>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<List>, Self::Error> {
        match self {
            List::List(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<Compound>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<Compound>, Self::Error> {
        match self {
            List::Compound(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<RawVec<i32>>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<RawVec<i32>>, Self::Error> {
        match self {
            List::IntArray(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<Vec<i32>>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<Vec<i32>>, Self::Error> {
        match self {
            List::IntArray(value) => Ok(value.into_iter().map(Into::into).collect()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<RawVec<i64>>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<RawVec<i64>>, Self::Error> {
        match self {
            List::LongArray(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

impl TryInto<Vec<Vec<i64>>> for List {
    type Error = ();

    #[inline]
    fn try_into(self) -> Result<Vec<Vec<i64>>, Self::Error> {
        match self {
            List::LongArray(value) => Ok(value.into_iter().map(Into::into).collect()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
