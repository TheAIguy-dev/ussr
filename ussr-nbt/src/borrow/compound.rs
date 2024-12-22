use std::{hint::unreachable_unchecked, marker::PhantomData};

use super::{
    reader::Reader,
    stack::{Stack, StackElement},
    tape::{Tape, TapeElement, TapeElementKind},
};
use crate::NbtDecodeError;

pub struct Compound<'a> {
    _marker: PhantomData<&'a ()>,
}

impl Compound<'_> {
    #[inline]
    pub(crate) unsafe fn read<'a>(
        _: &mut impl Reader<'a>,
        tape: &mut Tape<'a>,
        stack: &mut Stack,
    ) -> Result<(), NbtDecodeError> {
        stack.push_unchecked(StackElement::new_compound(tape.len()));
        tape.push_unchecked(TapeElement::new_with_len_and_offset(
            TapeElementKind::Compound,
            0,
            0,
        ));
        Ok(())
    }

    #[inline]
    pub(crate) unsafe fn read_in_list<'a>(
        reader: &mut impl Reader<'a>,
        tape: &mut Tape<'a>,
        stack: &mut Stack,
    ) -> Result<(), NbtDecodeError> {
        let StackElement::CompoundList { len, index } = stack.peek_unchecked_mut() else {
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
            Compound::read(reader, tape, stack)?;
        }

        Ok(())
    }
}
