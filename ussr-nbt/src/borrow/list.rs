use std::{hint::unreachable_unchecked, marker::PhantomData};

use crate::{
    NbtDecodeError, TAG_BYTE, TAG_BYTE_ARRAY, TAG_COMPOUND, TAG_DOUBLE, TAG_FLOAT, TAG_INT,
    TAG_INT_ARRAY, TAG_LIST, TAG_LONG, TAG_LONG_ARRAY, TAG_SHORT, TAG_STRING,
};

use super::{
    read_utils::{read_list, read_string},
    reader::Reader,
    stack::{Stack, StackElement},
    tape::{Tape, TapeElement, TapeElementKind},
};

pub struct List<'a> {
    _marker: PhantomData<&'a ()>,
}

impl List<'_> {
    pub(crate) unsafe fn read<'a>(
        reader: &mut impl Reader<'a>,
        tape: &mut Tape<'a>,
        stack: &mut Stack,
    ) -> Result<(), NbtDecodeError> {
        let tag_id: u8 = reader.read_u8()?;

        match tag_id {
            TAG_BYTE => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::ByteList,
                read_list::<u8>(reader)? as u64,
            )),
            TAG_SHORT => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::ShortList,
                read_list::<i16>(reader)? as u64,
            )),
            TAG_INT => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::IntList,
                read_list::<i32>(reader)? as u64,
            )),
            TAG_LONG => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::LongList,
                read_list::<i64>(reader)? as u64,
            )),
            TAG_FLOAT => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::FloatList,
                read_list::<f32>(reader)? as u64,
            )),
            TAG_DOUBLE => tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::DoubleList,
                read_list::<f64>(reader)? as u64,
            )),
            TAG_BYTE_ARRAY => {
                let len: usize = reader.read_i32()?.max(0) as usize;
                let index: usize = tape.len();

                tape.push_unchecked(TapeElement::new_with_len_and_offset(
                    TapeElementKind::ByteArrayList,
                    len,
                    index + len.unchecked_add(1),
                ));

                tape.reserve(len.unchecked_add(1));

                for _ in 0..len {
                    tape.push_unchecked(TapeElement::new(read_list::<u8>(reader)? as u64));
                }

                tape.push_unchecked(TapeElement::new_with_kind(
                    TapeElementKind::End,
                    index as u64,
                ));
            }
            TAG_STRING => {
                let len: usize = reader.read_i32()?.max(0) as usize;
                let index: usize = tape.len();

                tape.push_unchecked(TapeElement::new_with_len_and_offset(
                    TapeElementKind::StringList,
                    len,
                    index + len.unchecked_add(1),
                ));

                tape.reserve(len.unchecked_add(1));

                for _ in 0..len {
                    tape.push_unchecked(TapeElement::new(read_string(reader)? as u64));
                }

                tape.push_unchecked(TapeElement::new_with_kind(
                    TapeElementKind::End,
                    index as u64,
                ));
            }
            TAG_LIST => {
                let len: usize = reader.read_i32()?.max(0) as usize;
                stack.push_unchecked(StackElement::new_list_list(len, tape.len()));
                tape.push_unchecked(TapeElement::new_with_len_and_offset(
                    TapeElementKind::ListList,
                    len,
                    0,
                ));
            }
            TAG_COMPOUND => {
                let len: usize = reader.read_i32()?.max(0) as usize;
                stack.push_unchecked(StackElement::new_compound_list(len, tape.len()));
                tape.push_unchecked(TapeElement::new_with_len_and_offset(
                    TapeElementKind::CompoundList,
                    len,
                    0,
                ));
            }
            TAG_INT_ARRAY => {
                let len: usize = reader.read_i32()?.max(0) as usize;
                let index: usize = tape.len();

                tape.push_unchecked(TapeElement::new_with_len_and_offset(
                    TapeElementKind::IntArrayList,
                    len,
                    index + len.unchecked_add(1),
                ));

                tape.reserve(len.unchecked_add(1));

                for _ in 0..len {
                    tape.push_unchecked(TapeElement::new(read_list::<i32>(reader)? as u64));
                }

                tape.push_unchecked(TapeElement::new_with_kind(
                    TapeElementKind::End,
                    index as u64,
                ));
            }
            TAG_LONG_ARRAY => {
                let len: usize = reader.read_i32()?.max(0) as usize;
                let index: usize = tape.len();

                tape.push_unchecked(TapeElement::new_with_len_and_offset(
                    TapeElementKind::LongArrayList,
                    len,
                    index + len.unchecked_add(1),
                ));

                tape.reserve(len.unchecked_add(1));

                for _ in 0..len {
                    tape.push_unchecked(TapeElement::new(read_list::<i64>(reader)? as u64));
                }

                tape.push_unchecked(TapeElement::new_with_kind(
                    TapeElementKind::End,
                    index as u64,
                ));
            }
            _ if reader.read_i32()? <= 0 => {
                tape.push_unchecked(TapeElement::new_with_kind(TapeElementKind::EmptyList, 0))
            }
            _ => return Err(NbtDecodeError::InvalidTag(tag_id)),
        }

        Ok(())
    }

    #[inline]
    pub(crate) unsafe fn read_in_list<'a>(
        reader: &mut impl Reader<'a>,
        tape: &mut Tape<'a>,
        stack: &mut Stack,
    ) -> Result<(), NbtDecodeError> {
        let StackElement::ListList { len, index } = stack.peek_unchecked_mut() else {
            unreachable_unchecked()
        };

        tape.reserve(1);

        if *len == 0 {
            let offset: usize = tape.len();
            tape.get_unchecked_mut(*index).set_offset(offset);
            tape.push_unchecked(TapeElement::new_with_kind(
                TapeElementKind::End,
                *index as u64,
            ));
            stack.pop_unchecked();
        } else {
            *len -= 1;
            List::read(reader, tape, stack)?;
        }

        Ok(())
    }
}
