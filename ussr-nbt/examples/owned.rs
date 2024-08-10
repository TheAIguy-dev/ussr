//! An example of using the [`ussr_nbt::owned`] module.

use ussr_nbt::owned::*;

fn main() {
    let mut buf: Vec<u8> = Vec::new();

    Nbt {
        name: "Test NBT".into(),
        compound: vec![
            ("Test byte", 123u8.into()),
            ("Test string", "Hello, NBT!".into()),
            ("Test list", vec![1f32, 2f32, 3f32].into()),
        ]
        .into(),
    }
    .write(&mut buf)
    .unwrap();

    let nbt: Nbt = Nbt::read(&mut &buf[..]).unwrap();
    println!("{:#?}", nbt);
}
