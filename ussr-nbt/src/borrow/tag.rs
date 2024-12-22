use std::{hint::unreachable_unchecked, marker::PhantomData};

use super::{
    compound::Compound,
    list::List,
    read_utils::{read_list, read_string},
    reader::Reader,
    stack::Stack,
    tape::{Tape, TapeElement},
};
use crate::{
    borrow::{stack::StackElement, tape::TapeElementKind},
    NbtDecodeError, TAG_BYTE, TAG_BYTE_ARRAY, TAG_COMPOUND, TAG_DOUBLE, TAG_END, TAG_FLOAT, TAG_INT,
    TAG_INT_ARRAY, TAG_LIST, TAG_LONG, TAG_LONG_ARRAY, TAG_SHORT, TAG_STRING,
};

pub struct Tag<'a> {
    _marker: PhantomData<&'a ()>,
}

impl Tag<'_> {
    #[inline]
    pub(crate) unsafe fn read<'a>(
        reader: &mut impl Reader<'a>,
        tape: &mut Tape<'a>,
        stack: &mut Stack,
    ) -> Result<(), NbtDecodeError> {
        let StackElement::Compound { len, index } = stack.peek_unchecked_mut() else {
            unreachable_unchecked()
        };

        let tag_id: u8 = reader.read_u8()?;
        tape.reserve(3);

        if tag_id == TAG_END {
            *tape.get_unchecked_mut(*index) =
                TapeElement::new_with_len_and_offset(TapeElementKind::Compound, *len, tape.len());
            tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::End,
                *index as u64,
            ));
            stack.pop_unchecked();
            return Ok(());
        }

        *len += 1;
        tape.push_unchecked(TapeElement::new(read_string(reader)? as u64));

        match tag_id {
            TAG_BYTE => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::Byte,
                reader.read_u8()? as u64,
            )),
            TAG_SHORT => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::Short,
                reader.read_u16()? as u64,
            )),
            TAG_INT => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::Int,
                reader.read_u32()? as u64,
            )),
            TAG_LONG => {
                let long: u64 = reader.read_u64()?;
                tape.push_unchecked(TapeElement::new_with_kind(
                    TapeElementKind::Long,
                    long >> 32,
                ));
                tape.push_unchecked(TapeElement::new_with_kind(
                    TapeElementKind::Long,
                    long << 32 >> 32,
                ));
            }
            TAG_FLOAT => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::Float,
                reader.read_f32()?.to_bits() as u64,
            )),
            TAG_DOUBLE => {
                let double: u64 = reader.read_f64()?.to_bits();
                tape.push_unchecked(TapeElement::new_with_kind(
                    TapeElementKind::Double,
                    double >> 32,
                ));
                tape.push_unchecked(TapeElement::new_with_kind(
                    TapeElementKind::Double,
                    double << 32 >> 32,
                ));
            }
            TAG_BYTE_ARRAY => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::ByteArray,
                read_list::<u8>(reader)? as u64,
            )),
            TAG_STRING => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::String,
                read_string(reader)? as u64,
            )),
            TAG_LIST => List::read(reader, tape, stack)?,
            TAG_COMPOUND => Compound::read(reader, tape, stack)?,
            TAG_INT_ARRAY => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::IntArray,
                read_list::<i32>(reader)? as u64,
            )),
            TAG_LONG_ARRAY => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::LongArray,
                read_list::<i64>(reader)? as u64,
            )),
            _ => return Err(NbtDecodeError::InvalidTag(tag_id)),
        }

        Ok(())
    }
}
