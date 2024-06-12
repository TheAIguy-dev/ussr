use std::{
    io::{self, Read, Write},
    mem::size_of,
};

use paste::paste;

macro_rules! impl_read_ext {
    ($($type:ty),*) => {
        paste! {
            $(
                fn [<read_ $type>](&mut self) -> io::Result<$type> {
                    let mut buf = [0; size_of::<$type>()];
                    self.read_exact(&mut buf)?;
                    Ok(<$type>::from_be_bytes(buf))
                }
            )*
        }
    };
}

macro_rules! impl_write_ext {
    ($($type:ty),*) => {
        paste! {
            $(
                fn [<write_ $type>](&mut self, value: $type) -> io::Result<()> {
                    self.write_all(&value.to_be_bytes())
                }
            )*
        }
    };
}

/// Extends [`Read`] with read methods for primitive types.
pub trait ReadExt: Read {
    impl_read_ext!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
}
impl<R: Read> ReadExt for R {}

/// Extends [`Write`] with write methods for primitive types.
pub trait WriteExt: Write {
    impl_write_ext!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
}
impl<W: Write> WriteExt for W {}
