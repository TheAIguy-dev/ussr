use std::io::{self, Cursor};

use crate::{num::Num, NbtDecodeError};

pub struct UnexpectedEof;
impl From<UnexpectedEof> for NbtDecodeError {
    fn from(_: UnexpectedEof) -> Self {
        NbtDecodeError::Io(io::ErrorKind::UnexpectedEof.into())
    }
}

pub unsafe trait Reader<'a> {
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], UnexpectedEof>;

    fn skip(&mut self, len: usize) -> Result<(), UnexpectedEof>;

    unsafe fn ptr(&mut self) -> *const u8;

    read_ty!(u8, u16, u32, u64, i32, f32, f64);
}

unsafe impl<'a> Reader<'a> for &'a [u8] {
    #[inline]
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], UnexpectedEof> {
        let (slice, remaining) = self.split_at_checked(len).ok_or(UnexpectedEof)?;
        *self = remaining;
        Ok(slice)
    }

    #[inline]
    fn skip(&mut self, len: usize) -> Result<(), UnexpectedEof> {
        if len > self.len() {
            return Err(UnexpectedEof);
        }
        *self = unsafe {
            std::slice::from_raw_parts(self.as_ptr().add(len), self.len().unchecked_sub(len))
        };
        Ok(())
    }

    #[inline]
    unsafe fn ptr(&mut self) -> *const u8 {
        self.as_ptr()
    }
}

unsafe impl<'a> Reader<'a> for Cursor<&'a [u8]> {
    #[inline]
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], UnexpectedEof> {
        let pos: usize = self.position() as usize;
        let slice: &[u8] = self.get_ref().get(pos..pos + len).ok_or(UnexpectedEof)?;
        self.set_position((pos + len) as u64);
        Ok(slice)
    }

    #[inline]
    fn skip(&mut self, len: usize) -> Result<(), UnexpectedEof> {
        let pos: usize = self.position() as usize;
        if len > self.get_ref().len() - pos {
            return Err(UnexpectedEof);
        }
        self.set_position((pos + len) as u64);
        Ok(())
    }

    #[inline]
    unsafe fn ptr(&mut self) -> *const u8 {
        self.get_ref().as_ptr().add(self.position() as usize)
    }
}

unsafe impl<'a> Reader<'a> for Cursor<&'a Vec<u8>> {
    #[inline]
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], UnexpectedEof> {
        let pos: usize = self.position() as usize;
        let slice: &[u8] = self.get_ref().get(pos..pos + len).ok_or(UnexpectedEof)?;
        self.set_position((pos + len) as u64);
        Ok(slice)
    }

    #[inline]
    fn skip(&mut self, len: usize) -> Result<(), UnexpectedEof> {
        let pos: usize = self.position() as usize;
        if len > self.get_ref().len() - pos {
            return Err(UnexpectedEof);
        }
        self.set_position((pos + len) as u64);
        Ok(())
    }

    #[inline]
    unsafe fn ptr(&mut self) -> *const u8 {
        self.get_ref().as_ptr().add(self.position() as usize)
    }
}

macro_rules! read_ty {
    ($($type:ty),*) => {
        paste::paste! {
            $(
                #[inline]
                fn [<read_ $type>](&mut self) -> Result<$type, UnexpectedEof> {
                    let ptr: *const $type = unsafe { self.ptr() } as *const $type;
                    self.skip(size_of::<$type>())?;
                    Ok(unsafe { ptr.read_unaligned() }.to_be())
                }
            )*
        }
    };
}
use read_ty;
