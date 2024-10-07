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

// Compound -> Compound type
impl<S: From<MString>> From<Compound> for Vec<(S, Tag)> {
    #[inline]
    fn from(value: Compound) -> Self {
        value.tags.into_iter().map(|(k, v)| (k.into(), v)).collect()
    }
}
impl<K: From<MString> + Eq + Hash, S: std::hash::BuildHasher + Default> From<Compound> for HashMap<K, Tag, S> {
    #[inline]
    fn from(value: Compound) -> Self {
        value.tags.into_iter().map(|(k, v)| (k.into(), v)).collect()
    }
}
