//! An example of converting between NBT tags and Rust types.

#![allow(unused_variables)]

use ussr_nbt::{mutf8::mstr, owned::*};

fn main() {
    let nbt: Nbt = Nbt {
        name: "Test NBT".into(),
        compound: vec![
            ("Test byte".into(), 123u8.into()),
            ("Test string".into(), "Hello, NBT!".into()),
            ("Test list".into(), vec![1f32, 2f32, 3f32].into()),
        ]
        .into(),
    };

    let byte: u8 = nbt.compound.tags[0].1.byte().unwrap();
    let string: &mstr = nbt.compound.tags[1].1.string().unwrap();
    let floats: Vec<f32> = nbt.compound.tags[2].1.list().unwrap().float().unwrap().to_vec();
}
