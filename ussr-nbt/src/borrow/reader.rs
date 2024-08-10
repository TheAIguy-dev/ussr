use std::io::{self, Cursor};

use paste::paste;

pub trait Reader<'a> {
    fn read_slice(&mut self, len: usize) -> io::Result<&'a [u8]>;

    read_ty!(u8, u16, u32, i16, i32, i64, f32, f64);
}

impl<'a> Reader<'a> for &'a [u8] {
    #[inline]
    fn read_slice(&mut self, len: usize) -> io::Result<&'a [u8]> {
        if len > self.len() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        let (slice, remaining) = self.split_at(len);
        *self = remaining;
        Ok(slice)
    }
}

impl<'a> Reader<'a> for Cursor<&'a [u8]> {
    #[inline]
    fn read_slice(&mut self, len: usize) -> io::Result<&'a [u8]> {
        let pos: usize = self.position() as usize;
        if len > self.get_ref().as_ref().len() - pos {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        let slice: &[u8] = &self.get_ref()[pos..pos + len];
        self.set_position(self.position() + len as u64);
        Ok(slice)
    }
}

impl<'a> Reader<'a> for Cursor<&'a Vec<u8>> {
    #[inline]
    fn read_slice(&mut self, len: usize) -> io::Result<&'a [u8]> {
        let pos: usize = self.position() as usize;
        if len > self.get_ref().len() - pos {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        let slice: &[u8] = &self.get_ref()[pos..pos + len];
        self.set_position(self.position() + len as u64);
        Ok(slice)
    }
}

macro_rules! read_ty {
    ($($type:ty),*) => {
        paste! {
            $(
                #[inline]
                fn [<read_ $type>](&mut self) -> io::Result<$type> {
                    Ok(<$type>::from_be_bytes(self.read_slice(size_of::<$type>())?.try_into().unwrap()))
                }
            )*
        }
    };
}
use read_ty;
