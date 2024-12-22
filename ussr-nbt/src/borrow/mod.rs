mod compound;
mod list;
mod read_utils;
mod reader;
mod stack;
mod tag;
mod tape;

use std::io::{self, Write};

use crate::{DecodeOpts, EncodeOpts, NbtDecodeError, TAG_COMPOUND};
use compound::Compound;
use list::List;
use read_utils::read_string;
use reader::Reader;
use stack::{Stack, StackElement};
use tag::Tag;
use tape::{ImmutableTape, Tape, TapeElement};

#[allow(non_camel_case_types)]
type mstr = str;

#[derive(Debug)]
pub struct Nbt<'a> {
    tape: ImmutableTape<'a>,
}

impl<'a> Nbt<'a> {
    pub fn read(reader: &mut impl Reader<'a>) -> Result<Nbt<'a>, NbtDecodeError> {
        Nbt::read_with_opts(reader, DecodeOpts::default())
    }

    pub fn read_with_opts(
        reader: &mut impl Reader<'a>,
        opts: DecodeOpts,
    ) -> Result<Nbt<'a>, NbtDecodeError> {
        let root_tag: u8 = reader.read_u8()?;
        if root_tag != TAG_COMPOUND {
            return Err(NbtDecodeError::InvalidRootTag(root_tag));
        }

        let mut tape: Tape<'a> = Tape::new();
        let mut stack: Stack = Stack::new(opts.depth_limit as usize + 1); // the + 1 is absolutely necessary oh my god

        unsafe {
            tape.push_unchecked(TapeElement::new(read_string(reader)? as u64));
            Compound::read(reader, &mut tape, &mut stack)?;

            while !stack.is_empty() {
                if stack.len() > opts.depth_limit as usize {
                    return Err(NbtDecodeError::DepthLimitExceeded);
                }

                match stack.peek_unchecked_mut() {
                    StackElement::Compound { .. } => Tag::read(reader, &mut tape, &mut stack)?,
                    StackElement::ListList { .. } => {
                        List::read_in_list(reader, &mut tape, &mut stack)?
                    }
                    StackElement::CompoundList { .. } => {
                        Compound::read_in_list(reader, &mut tape, &mut stack)?
                    }
                }
            }
        }

        Ok(Nbt { tape: tape.into() })
    }

    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        Nbt::write_with_opts(self, writer, EncodeOpts::default())
    }

    pub fn write_with_opts(&self, writer: &mut impl Write, opts: EncodeOpts) -> io::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tape::TapeElementKind;

    fn get_list<T>(ptr: *const u8) -> Vec<T> {
        let len: usize = unsafe { (ptr as *const i32).read_unaligned() }
            .to_be()
            .max(0) as usize;
        let mut vec: Vec<T> = Vec::with_capacity(len);
        unsafe {
            std::ptr::copy_nonoverlapping(
                ptr.add(size_of::<i32>()),
                vec.as_mut_ptr() as *mut u8,
                len * size_of::<T>(),
            );
            vec.set_len(len);
        }
        vec
    }

    fn get_string<'a>(ptr: *const u8) -> &'a str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                ptr.add(size_of::<u16>()),
                (ptr as *const u16).read_unaligned().to_be() as usize,
            ))
        }
    }

    #[test]
    fn test_read() {
        #[rustfmt::skip]
        let buf: &[u8] = &[
            0x0a, 0, 0,
                0x01,    0, 0,    42,

                0x02,    0, 0,    0, 123,

                0x03,    0, 0,    0, 0, 1, 200,

                0x04,    0, 0,    0, 0, 0, 183, 227, 35, 206, 93,

                0x05,    0, 0,    66, 246, 233, 223,

                0x06,    0, 0,    64, 94, 221, 60, 7, 251, 76, 153,

                0x07,    0, 0,    0, 0, 0, 4,
                    1,
                    2,
                    3,
                    4,

                0x08,    0, 0,    0, 13,    72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33,

                0x09,    0, 0,    0x02,    0, 0, 0, 4,
                    0, 5,
                    0, 6,
                    0, 7,
                    0, 8,

                0x0a,    0, 0,
                0x00,

                0x0b,    0, 0,    0, 0, 0, 4,
                    0, 0, 0, 9,
                    0, 0, 0, 10,
                    0, 0, 0, 11,
                    0, 0, 0, 12,

                0x0c,    0, 0,    0, 0, 0, 4,
                    0, 0, 0, 0, 0, 0, 0, 13,
                    0, 0, 0, 0, 0, 0, 0, 14,
                    0, 0, 0, 0, 0, 0, 0, 15,
                    0, 0, 0, 0, 0, 0, 0, 16,
            0x00,
        ];

        // Compound 12 28
        //     &""
        //     Byte 42
        //
        //     &""
        //     Short 123
        //
        //     &""
        //     Int 456
        //
        //     &""
        //     Long 789789789
        //     Long 789789789
        //
        //     &""
        //     Float 42.0
        //
        //     &""
        //     Double 123.45678901234567890123456
        //     Double 123.45678901234567890123456
        //
        //     &""
        //     ByteArray &[1, 2, 3, 4]
        //
        //     &""
        //     String &"Hello, world!"
        //
        //     &""
        //     ShortList &[5, 6, 7, 8]
        //
        //     &""
        //     Compound 0 17
        //     End 16
        //
        //     &""
        //     IntArray &[9, 10, 11, 12]
        //
        //     &""
        //     LongArray &[13, 14, 15, 16]
        // End 0

        let nbt: Nbt = Nbt::read(&mut &buf[..]).unwrap();
        let mut elements = nbt.tape.iter();

        assert_eq!(nbt.tape.len(), 30);

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");

        assert_eq!(
            elements.next().unwrap().get_len_and_offset(),
            (TapeElementKind::Compound, 12, 29)
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(elements.next().unwrap().get_data_without_kind() as u8, 42);

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(elements.next().unwrap().get_data_without_kind() as i16, 123);

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(elements.next().unwrap().get_data_without_kind() as i32, 456);

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            (elements.next().unwrap().get_data_without_kind() << 32
                | elements.next().unwrap().get_data_without_kind()) as i64,
            789789789789
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            f32::from_bits(elements.next().unwrap().get_data_without_kind() as u32),
            123.45678
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            f64::from_bits(
                elements.next().unwrap().get_data_without_kind() << 32
                    | elements.next().unwrap().get_data_without_kind()
            ),
            123.45678901234567890123456
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            {
                let (kind, data) = elements.next().unwrap().get_data();
                (kind, get_list::<u8>(data as *const u8))
            },
            (TapeElementKind::ByteArray, vec![1, 2, 3, 4])
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            get_string(elements.next().unwrap().get_ptr()),
            "Hello, world!"
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            {
                let (kind, data) = elements.next().unwrap().get_data();
                (
                    kind,
                    get_list::<i16>(data as *const u8)
                        .into_iter()
                        .map(i16::to_be)
                        .collect(),
                )
            },
            (TapeElementKind::ShortList, vec![5, 6, 7, 8])
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            elements.next().unwrap().get_len_and_offset(),
            (TapeElementKind::Compound, 0, 24)
        );
        assert_eq!(
            elements.next().unwrap().get_data(),
            (TapeElementKind::End, 23)
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            {
                let (kind, data) = elements.next().unwrap().get_data();
                (
                    kind,
                    get_list::<i32>(data as *const u8)
                        .into_iter()
                        .map(i32::to_be)
                        .collect(),
                )
            },
            (TapeElementKind::IntArray, vec![9, 10, 11, 12])
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            {
                let (kind, data) = elements.next().unwrap().get_data();
                (
                    kind,
                    get_list::<i64>(data as *const u8)
                        .into_iter()
                        .map(i64::to_be)
                        .collect(),
                )
            },
            (TapeElementKind::LongArray, vec![13, 14, 15, 16])
        );

        assert_eq!(
            elements.next().unwrap().get_data(),
            (TapeElementKind::End, 1)
        );
    }

    #[test]
    fn test_read_complex() {
        #[rustfmt::skip]
        let buf: &[u8] = &[
            0x0a, 0, 0,
                0x09,    0, 0,    0x09,    0, 0, 0, 2,
                    0x01,    0, 0, 0, 3,
                        1,
                        2,
                        3,

                    0x0a,    0, 0, 0, 1,
                        0x00,
            0x00,
        ];

        // Compound 1 9
        //     &""
        //     ListList 2 8
        //         ByteList &[1, 2, 3]
        //         CompoundList 1 7
        //             Compound 0 6
        //             End 5
        //         End 4
        //     End 2
        // End 0

        let nbt: Nbt = Nbt::read(&mut &buf[..]).unwrap();
        let mut elements = nbt.tape.iter();

        assert_eq!(nbt.tape.len(), 11);

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");

        assert_eq!(
            elements.next().unwrap().get_len_and_offset(),
            (TapeElementKind::Compound, 1, 10)
        );

        assert_eq!(get_string(elements.next().unwrap().get_ptr()), "");
        assert_eq!(
            elements.next().unwrap().get_len_and_offset(),
            (TapeElementKind::ListList, 2, 9)
        );

        assert_eq!(
            {
                let (kind, data) = elements.next().unwrap().get_data();
                (kind, get_list::<u8>(data as *const u8))
            },
            (TapeElementKind::ByteList, vec![1, 2, 3])
        );

        assert_eq!(
            elements.next().unwrap().get_len_and_offset(),
            (TapeElementKind::CompoundList, 1, 8)
        );

        assert_eq!(
            elements.next().unwrap().get_len_and_offset(),
            (TapeElementKind::Compound, 0, 7)
        );

        assert_eq!(
            elements.next().unwrap().get_data(),
            (TapeElementKind::End, 6)
        );

        assert_eq!(
            elements.next().unwrap().get_data(),
            (TapeElementKind::End, 5)
        );

        assert_eq!(
            elements.next().unwrap().get_data(),
            (TapeElementKind::End, 3)
        );

        assert_eq!(
            elements.next().unwrap().get_data(),
            (TapeElementKind::End, 1)
        );
    }

    #[test]
    fn test_read_depth_limit() {
        #[rustfmt::skip]
        let buf: &[u8] = &[
            0x0a, 0, 0,
                0x0a, 0, 0,
                0x00,
            0x00,
        ];

        let nbt = Nbt::read_with_opts(&mut &buf[..], DecodeOpts::new(1, true));

        assert!(matches!(nbt, Err(NbtDecodeError::DepthLimitExceeded)));
    }
}
