use std::io::{self, Cursor};

use paste::paste;

use crate::NbtReadError;

pub trait Reader<'a> {
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], NbtReadError>;

    read_ty!(u8, u16, u32, i16, i32, i64, f32, f64);
}

impl<'a> Reader<'a> for &'a [u8] {
    fn read_slice(&mut self, amount: usize) -> Result<&'a [u8], NbtReadError> {
        if amount > self.len() {
            return Err(NbtReadError::Io(io::ErrorKind::UnexpectedEof.into()));
        }
        let slice: &[u8] = &self[..amount];
        *self = &self[amount..];
        Ok(slice)
    }
}

impl<'a> Reader<'a> for Cursor<&'a [u8]> {
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], NbtReadError> {
        if len > self.get_ref().len() {
            return Err(NbtReadError::Io(io::ErrorKind::UnexpectedEof.into()));
        }
        let slice: &[u8] =
            &self.get_ref()[self.position() as usize..self.position() as usize + len];
        self.set_position(self.position() + len as u64);
        Ok(slice)
    }
}

macro_rules! read_ty {
    ($($type:ty),*) => {
        paste! {
            $(
                fn [<read_ $type>](&mut self) -> Result<$type, NbtReadError> {
                    Ok(<$type>::from_be_bytes(self.read_slice(size_of::<$type>())?.try_into().unwrap()))
                }
            )*
        }
    };
}
use read_ty;
