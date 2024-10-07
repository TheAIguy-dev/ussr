use std::io::{Cursor, Read};

use flate2::bufread::GzDecoder;

#[test]
fn borrow_eq() {
    let buf: Vec<u8> = gzip_decode(&std::fs::read("tests/TheAIguy_.nbt").unwrap());

    let mut reader: Cursor<&[u8]> = Cursor::new(&buf[..]);
    let read_nbt = ussr_nbt::borrow::Nbt::read(&mut reader).unwrap();

    let mut new_buf: Vec<u8> = vec![];
    ussr_nbt::borrow::Nbt::write(&read_nbt, &mut new_buf);

    let mut reader: Cursor<&[u8]> = Cursor::new(&new_buf[..]);
    let written_nbt = ussr_nbt::borrow::Nbt::read(&mut reader).unwrap();

    assert_eq!(read_nbt, written_nbt);
}

#[test]
fn owned_eq() {
    let buf: Vec<u8> = gzip_decode(&std::fs::read("tests/TheAIguy_.nbt").unwrap());

    let mut reader: Cursor<&[u8]> = Cursor::new(&buf[..]);
    let read_nbt = ussr_nbt::owned::Nbt::read(&mut reader).unwrap();

    let mut new_buf: Vec<u8> = vec![];
    ussr_nbt::owned::Nbt::write(&read_nbt, &mut new_buf).unwrap();

    let mut reader: Cursor<&[u8]> = Cursor::new(&new_buf[..]);
    let written_nbt = ussr_nbt::owned::Nbt::read(&mut reader).unwrap();

    assert_eq!(read_nbt, written_nbt);
}

fn gzip_decode(encoded: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    if GzDecoder::new(encoded).read_to_end(&mut buf).is_err() {
        buf = encoded.to_vec();
    }
    buf
}
