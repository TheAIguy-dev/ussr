//! An example of using the [`ussr_nbt::borrow`] module.

use std::io::Cursor;

use ussr_nbt::{borrow::*, mutf8::mstr};

fn main() {
    let buf: Vec<u8> = vec![
        10, 0, 8, 84, 101, 115, 116, 32, 78, 66, 84, 1, 0, 9, 84, 101, 115, 116, 32, 98, 121, 116,
        101, 123, 8, 0, 11, 84, 101, 115, 116, 32, 115, 116, 114, 105, 110, 103, 0, 11, 72, 101,
        108, 108, 111, 44, 32, 78, 66, 84, 33, 9, 0, 9, 84, 101, 115, 116, 32, 108, 105, 115, 116,
        5, 0, 0, 0, 3, 63, 128, 0, 0, 64, 0, 0, 0, 64, 64, 0, 0, 0,
    ];

    // Reading from a [`Cursor`] is ever so slightly faster than from a byte slice.
    let nbt: Nbt = Nbt::read(&mut Cursor::new(&buf)).unwrap();
    assert_eq!(
        nbt.compound
            .tags
            .iter()
            .filter(|(k, _)| *k == "Test string".as_bytes())
            .next()
            .unwrap()
            .1,
        Tag::String(&mstr::from_string("Hello, NBT!"))
    );
}
