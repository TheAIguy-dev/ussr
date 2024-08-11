use std::{collections::HashMap, hash::Hash};

use super::{Compound, List, Tag};
use crate::{endian::RawVec, mutf8::MString};

// Tag type -> Tag
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
impl From<RawVec<i64>> for Tag {
    #[inline]
    fn from(value: RawVec<i64>) -> Self {
        Tag::LongArray(value)
    }
}

// List type -> List
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
impl From<RawVec<i32>> for List {
    #[inline]
    fn from(value: RawVec<i32>) -> Self {
        List::Int(value)
    }
}
impl From<RawVec<i64>> for List {
    #[inline]
    fn from(value: RawVec<i64>) -> Self {
        List::Long(value)
    }
}
impl From<RawVec<f32>> for List {
    #[inline]
    fn from(value: RawVec<f32>) -> Self {
        List::Float(value)
    }
}
impl From<RawVec<f64>> for List {
    #[inline]
    fn from(value: RawVec<f64>) -> Self {
        List::Double(value)
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
impl From<Vec<RawVec<i64>>> for List {
    #[inline]
    fn from(value: Vec<RawVec<i64>>) -> Self {
        List::LongArray(value)
    }
}

// Convenience type -> Tag
impl From<Vec<i32>> for Tag {
    #[inline]
    fn from(value: Vec<i32>) -> Self {
        Tag::IntArray(value.into())
    }
}
impl From<Vec<i64>> for Tag {
    #[inline]
    fn from(value: Vec<i64>) -> Self {
        Tag::LongArray(value.into())
    }
}

// Convenience type -> List
impl From<Vec<i16>> for List {
    #[inline]
    fn from(value: Vec<i16>) -> Self {
        List::Short(value.into())
    }
}
impl From<Vec<i32>> for List {
    #[inline]
    fn from(value: Vec<i32>) -> Self {
        List::Int(value.into())
    }
}
impl From<Vec<i64>> for List {
    #[inline]
    fn from(value: Vec<i64>) -> Self {
        List::Long(value.into())
    }
}
impl From<Vec<f32>> for List {
    #[inline]
    fn from(value: Vec<f32>) -> Self {
        List::Float(value.into())
    }
}
impl From<Vec<f64>> for List {
    #[inline]
    fn from(value: Vec<f64>) -> Self {
        List::Double(value.into())
    }
}
impl From<Vec<Vec<i32>>> for List {
    #[inline]
    fn from(value: Vec<Vec<i32>>) -> Self {
        List::IntArray(value.into_iter().map(Into::into).collect())
    }
}
impl From<Vec<Vec<i64>>> for List {
    #[inline]
    fn from(value: Vec<Vec<i64>>) -> Self {
        List::LongArray(value.into_iter().map(Into::into).collect())
    }
}

// List type -> Tag
impl From<RawVec<i16>> for Tag {
    #[inline]
    fn from(value: RawVec<i16>) -> Self {
        Tag::List(List::Short(value))
    }
}
impl From<RawVec<f32>> for Tag {
    #[inline]
    fn from(value: RawVec<f32>) -> Self {
        Tag::List(List::Float(value))
    }
}
impl From<RawVec<f64>> for Tag {
    #[inline]
    fn from(value: RawVec<f64>) -> Self {
        Tag::List(List::Double(value))
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
impl From<Vec<Compound>> for Tag {
    #[inline]
    fn from(value: Vec<Compound>) -> Self {
        Tag::List(List::Compound(value))
    }
}
impl From<Vec<RawVec<i32>>> for Tag {
    #[inline]
    fn from(value: Vec<RawVec<i32>>) -> Self {
        Tag::List(List::IntArray(value))
    }
}
impl From<Vec<RawVec<i64>>> for Tag {
    #[inline]
    fn from(value: Vec<RawVec<i64>>) -> Self {
        Tag::List(List::LongArray(value))
    }
}

// Convenience list type -> Tag
impl From<Vec<i16>> for Tag {
    #[inline]
    fn from(value: Vec<i16>) -> Self {
        Tag::List(List::Short(value.into()))
    }
}
impl From<Vec<f32>> for Tag {
    #[inline]
    fn from(value: Vec<f32>) -> Self {
        Tag::List(List::Float(value.into()))
    }
}
impl From<Vec<f64>> for Tag {
    #[inline]
    fn from(value: Vec<f64>) -> Self {
        Tag::List(List::Double(value.into()))
    }
}
impl From<Vec<Vec<i32>>> for Tag {
    #[inline]
    fn from(value: Vec<Vec<i32>>) -> Self {
        Tag::List(List::IntArray(value.into_iter().map(Into::into).collect()))
    }
}
impl From<Vec<Vec<i64>>> for Tag {
    #[inline]
    fn from(value: Vec<Vec<i64>>) -> Self {
        Tag::List(List::LongArray(value.into_iter().map(Into::into).collect()))
    }
}

// Compound type -> Compound
impl From<Vec<(MString, Tag)>> for Compound {
    #[inline]
    fn from(value: Vec<(MString, Tag)>) -> Self {
        Compound { tags: value }
    }
}
impl From<HashMap<MString, Tag>> for Compound {
    #[inline]
    fn from(value: HashMap<MString, Tag>) -> Self {
        Compound {
            tags: value.into_iter().collect(),
        }
    }
}

// Tag -> Tag type
impl TryFrom<Tag> for u8 {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::Byte(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for i16 {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::Short(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for i32 {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::Int(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for i64 {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::Long(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for f32 {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::Float(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for f64 {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::Double(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<u8> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::ByteArray(value) => Ok(value),
            Tag::List(List::Byte(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for MString {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::String(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for List {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Compound {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::Compound(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for RawVec<i32> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::IntArray(value) => Ok(value),
            Tag::List(List::Int(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for RawVec<i64> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::LongArray(value) => Ok(value),
            Tag::List(List::Long(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

// List -> List type
impl TryFrom<List> for Vec<u8> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Byte(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for RawVec<i16> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Short(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for RawVec<i32> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Int(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for RawVec<i64> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Long(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for RawVec<f32> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Float(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for RawVec<f64> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Double(value) => Ok(value),
            List::Empty => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<Vec<u8>> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::ByteArray(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl<S: From<MString>> TryFrom<List> for Vec<S> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::String(value) => Ok(value.into_iter().map(S::from).collect()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<List> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::List(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<Compound> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Compound(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<RawVec<i32>> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::IntArray(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<RawVec<i64>> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::LongArray(value) => Ok(value),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

// Tag -> convenience type
impl TryFrom<Tag> for Vec<i32> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::IntArray(value) => Ok(value.into()),
            Tag::List(List::Int(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<i64> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::LongArray(value) => Ok(value.into()),
            Tag::List(List::Long(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

// List -> convenience type
impl TryFrom<List> for Vec<i16> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Short(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<i32> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Int(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<i64> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Long(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<f32> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Float(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<f64> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::Double(value) => Ok(value.into()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<Vec<i32>> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::IntArray(value) => Ok(value.into_iter().map(Into::into).collect()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<List> for Vec<Vec<i64>> {
    type Error = ();

    #[inline]
    fn try_from(value: List) -> Result<Self, Self::Error> {
        match value {
            List::LongArray(value) => Ok(value.into_iter().map(Into::into).collect()),
            List::Empty => Ok(vec![]),
            _ => Err(()),
        }
    }
}

// Tag -> list type
impl TryFrom<Tag> for RawVec<i16> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::Short(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for RawVec<f32> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::Float(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for RawVec<f64> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::Double(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<Vec<u8>> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::ByteArray(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl<S: From<MString>> TryFrom<Tag> for Vec<S> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::String(value)) => Ok(value.into_iter().map(S::from).collect()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<List> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::List(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<Compound> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::Compound(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<RawVec<i32>> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::IntArray(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<RawVec<i64>> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::LongArray(value)) => Ok(value),
            Tag::List(List::Empty) => Ok(vec![].into()),
            _ => Err(()),
        }
    }
}

// Tag -> convenience list type
impl TryFrom<Tag> for Vec<i16> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::Short(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<f32> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::Float(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<f64> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::Double(value)) => Ok(value.into()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<Vec<i32>> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::IntArray(value)) => Ok(value.into_iter().map(Into::into).collect()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}
impl TryFrom<Tag> for Vec<Vec<i64>> {
    type Error = ();

    #[inline]
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(List::LongArray(value)) => Ok(value.into_iter().map(Into::into).collect()),
            Tag::List(List::Empty) => Ok(vec![]),
            _ => Err(()),
        }
    }
}

// Compound -> Compound type
impl<S: From<MString>> From<Compound> for Vec<(S, Tag)> {
    #[inline]
    fn from(value: Compound) -> Self {
        value.tags.into_iter().map(|(k, v)| (k.into(), v)).collect()
    }
}
impl<S: From<MString> + Eq + Hash> From<Compound> for HashMap<S, Tag> {
    #[inline]
    fn from(value: Compound) -> Self {
        value.tags.into_iter().map(|(k, v)| (k.into(), v)).collect()
    }
}
