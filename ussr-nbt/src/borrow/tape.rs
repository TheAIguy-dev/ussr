use std::{fmt::Debug, marker::PhantomData};

use crate::{
    NbtDecodeError, TAG_BYTE, TAG_BYTE_ARRAY, TAG_COMPOUND, TAG_DOUBLE, TAG_END, TAG_FLOAT, TAG_INT,
    TAG_INT_ARRAY, TAG_LONG, TAG_LONG_ARRAY, TAG_SHORT, TAG_STRING,
};

pub struct Tape<'a> {
    elements: Vec<TapeElement>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Tape<'a> {
    #[inline]
    pub fn new() -> Tape<'a> {
        Tape {
            elements: Vec::with_capacity(1024),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&TapeElement> {
        self.elements.get(index)
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.elements.reserve(additional);
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut TapeElement> {
        self.elements.get_mut(index)
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut TapeElement {
        self.elements.get_unchecked_mut(index)
    }

    #[inline]
    pub fn push(&mut self, element: TapeElement) {
        self.elements.push(element);
    }

    #[inline]
    pub unsafe fn push_unchecked(&mut self, element: TapeElement) {
        let len: usize = self.elements.len();
        unsafe {
            let end: *mut TapeElement = self.elements.as_mut_ptr().add(len);
            std::ptr::write(end, element);
            self.elements.set_len(len + 1);
        }

        // assert_unchecked(self.elements.len() < self.elements.capacity());
        // self.elements.push(element);
    }

    #[inline]
    pub fn into_immutable(self) -> ImmutableTape<'a> {
        ImmutableTape {
            elements: self.elements.into_boxed_slice(),
            _marker: PhantomData,
        }
    }

    #[cfg(test)]
    pub fn iter(&self) -> impl Iterator<Item = &TapeElement> {
        self.elements.iter()
    }
}

#[derive(Clone, Copy)]
pub struct TapeElement(u64);

impl TapeElement {
    #[inline]
    pub fn new(data: u64) -> TapeElement {
        TapeElement(data)
    }

    #[inline]
    pub fn new_with_kind(kind: TapeElementKind, data: u64) -> TapeElement {
        debug_assert!(data >> 56 == 0, "High byte of data not zero: {data:064b}");
        TapeElement((kind as u64) << 56 | data)
    }

    #[inline]
    pub fn new_with_len_and_offset(
        kind: TapeElementKind,
        len: usize,
        offset: usize,
    ) -> TapeElement {
        TapeElement::new_with_kind(
            kind,
            (len.min(0xFFFFFF) as u64) << 32 | offset.min(0xFFFFFFFF) as u64,
        )
    }

    #[inline]
    pub fn set_offset(&mut self, offset: usize) {
        debug_assert!(self.0 << 32 >> 32 == 0);
        self.0 = self.0 | offset.min(0xFFFFFFFF) as u64;
    }

    #[cfg(test)]
    pub fn get_kind(&self) -> Option<TapeElementKind> {
        TapeElementKind::try_from((self.0 >> 56) as u8).ok()
    }

    #[cfg(test)]
    pub fn get_data(&self) -> (TapeElementKind, u64) {
        (
            TapeElementKind::try_from((self.0 >> 56) as u8).unwrap(),
            self.0 << 8 >> 8,
        )
    }

    #[cfg(test)]
    pub fn get_data_without_kind(&self) -> u64 {
        self.0 << 8 >> 8
    }

    #[cfg(test)]
    pub fn get_ptr(&self) -> *const u8 {
        (self.0 << 8 >> 8) as *const u8
    }

    #[cfg(test)]
    pub fn get_len_and_offset(&self) -> (TapeElementKind, usize, usize) {
        (
            TapeElementKind::try_from((self.0 >> 56) as u8).unwrap(),
            (self.0 << 8 >> 40) as u32 as usize,
            self.0 as u32 as usize,
        )
    }
}

impl Debug for TapeElement {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TapeElement(0x{:016x}  {:02x}  {})",
            self.0,
            self.0 >> 56,
            TapeElementKind::try_from((self.0 >> 56) as u8)
                .map(|kind| format!("{kind:?}"))
                .unwrap_or(String::new())
        )
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // todo
pub enum TapeElementKind {
    End = TAG_END,
    Byte = TAG_BYTE,
    Short = TAG_SHORT,
    Int = TAG_INT,
    Long = TAG_LONG,
    Float = TAG_FLOAT,
    Double = TAG_DOUBLE,
    ByteArray = TAG_BYTE_ARRAY,
    String = TAG_STRING,
    Compound = TAG_COMPOUND,
    IntArray = TAG_INT_ARRAY,
    LongArray = TAG_LONG_ARRAY,

    EmptyList,
    ByteList,
    ShortList,
    IntList,
    LongList,
    FloatList,
    DoubleList,
    ByteArrayList,
    StringList,
    ListList,
    CompoundList,
    IntArrayList,
    LongArrayList,
}

impl TryFrom<u8> for TapeElementKind {
    type Error = NbtDecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (TapeElementKind::End as u8..=TapeElementKind::LongArrayList as u8).contains(&value) {
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(NbtDecodeError::InvalidTag(value))
        }
    }
}

pub struct ImmutableTape<'a> {
    elements: Box<[TapeElement]>,
    _marker: PhantomData<&'a ()>,
}

impl ImmutableTape<'_> {
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    #[cfg(test)]
    pub fn iter(&self) -> impl Iterator<Item = &TapeElement> {
        self.elements.iter()
    }
}

impl From<Tape<'_>> for ImmutableTape<'_> {
    fn from(value: Tape<'_>) -> Self {
        ImmutableTape {
            elements: value.elements.into_boxed_slice(),
            _marker: PhantomData,
        }
    }
}

impl Debug for ImmutableTape<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tape")
            .field(
                "elements",
                &self
                    .elements
                    .iter()
                    .enumerate()
                    .map(|(i, e)| format!("{i}: {e:?}"))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}
