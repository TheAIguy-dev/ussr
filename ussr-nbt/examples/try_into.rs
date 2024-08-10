//! An example of using [`TryInto`] for convenience.
//!
//! [`TryInto`] is implemented for all tags and lists. There is only one possible error - a type mismatch.

#![allow(unused_variables)]

use ussr_nbt::{mutf8::MString, owned::*};

fn main() {
    let nbt: Nbt = Nbt {
        name: "Test NBT".into(),
        compound: vec![
            ("Test byte", 123u8.into()),
            ("Test string", "Hello, NBT!".into()),
            ("Test list", vec![1f32, 2f32, 3f32].into()),
        ]
        .into(),
    };

    let byte: u8 = nbt.compound.tags[1].1.clone().try_into().unwrap();
    let string: MString = nbt.compound.tags[1].1.clone().try_into().unwrap();
    let float_list: Vec<f32> = nbt.compound.tags[2].1.clone().try_into().unwrap();
}
