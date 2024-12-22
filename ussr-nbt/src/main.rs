use std::io::{Cursor, Read};

use flate2::bufread::GzDecoder;

fn gzip_decode(encoded: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    if GzDecoder::new(encoded).read_to_end(&mut buf).is_err() {
        buf = encoded.to_vec();
    }
    buf
}

fn main() {
    let input: Vec<u8> = gzip_decode(&std::fs::read("tests/TheAIguy_.nbt").unwrap());
    println!("{input:?}");

    let nbt: ussr_nbt::owned::Nbt = ussr_nbt::owned::Nbt::read(&mut Cursor::new(&input)).unwrap();
    println!("{nbt:#?}");

    let nbt: ussr_nbt::borrow::Nbt<'_> =
        ussr_nbt::borrow::Nbt::read(&mut Cursor::new(&input)).unwrap();
    println!("{nbt:#?}");
}
