use core::fmt;
use std::io::{self, Write};

use paste::paste;
#[cfg(feature = "uuid")]
use uuid::Uuid;

use crate::{VarWritable, Writable, WriteExt};

macro_rules! impl_writable {
    ($($type:ty),*) => {
        paste! {
            $(
                impl Writable for $type {
                    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {
                        writer.[<write_ $type>](*self)
                    }
                }
            )*
        }
    };
}
impl_writable!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

/// Write an array to the writer, prefixed with its length as a fixed-sized type `L`.
pub fn write_array<L, T>(writer: &mut impl Write, array: &[T]) -> io::Result<()>
where
    L: Writable + TryFrom<usize>,
    <L as TryFrom<usize>>::Error: fmt::Debug,
    T: Writable,
{
    L::try_from(array.len())
        .expect("Could not convert from usize")
        .write_to(writer)?;
    for item in array.iter() {
        item.write_to(writer)?;
    }
    Ok(())
}

/// Write an array to the writer, prefixed with its length as a variable-sized type `L`.
pub fn write_var_array<L, T>(writer: &mut impl Write, array: &[T]) -> io::Result<()>
where
    L: VarWritable + TryFrom<usize>,
    <L as TryFrom<usize>>::Error: fmt::Debug,
    T: VarWritable,
{
    L::try_from(array.len())
        .expect("Could not convert to {} from usize")
        .write_var_to(writer)?;
    for item in array.iter() {
        item.write_var_to(writer)?;
    }
    Ok(())
}

impl Writable for bool {
    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(*self as u8)
    }
}

impl VarWritable for u32 {
    fn write_var_to(&self, writer: &mut impl Write) -> io::Result<()> {
        let mut value: u32 = *self;
        loop {
            let byte: u8 = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                writer.write_u8(byte | 0x80)?;
            } else {
                writer.write_u8(byte)?;
                return Ok(());
            }
        }
    }
}

impl VarWritable for i32 {
    fn write_var_to(&self, writer: &mut impl Write) -> io::Result<()> {
        u32::write_var_to(&(*self as u32), writer)
    }
}

impl VarWritable for u64 {
    fn write_var_to(&self, writer: &mut impl Write) -> io::Result<()> {
        let mut value: u64 = *self;
        loop {
            let byte: u8 = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                writer.write_u8(byte | 0x80)?;
            } else {
                writer.write_u8(byte)?;
                return Ok(());
            }
        }
    }
}

impl VarWritable for i64 {
    fn write_var_to(&self, writer: &mut impl Write) -> io::Result<()> {
        u64::write_var_to(&(*self as u64), writer)
    }
}

impl VarWritable for usize {
    fn write_var_to(&self, writer: &mut impl Write) -> io::Result<()> {
        let mut value: usize = *self;
        loop {
            let byte: u8 = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                writer.write_u8(byte | 0x80)?;
            } else {
                writer.write_u8(byte)?;
                return Ok(());
            }
        }
    }
}

impl Writable for String {
    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {
        u32::write_var_to(&(self.len() as u32), writer)?;
        writer.write_all(self.as_bytes())
    }
}

impl<T: Writable> Writable for &T {
    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {
        (*self).write_to(writer)
    }
}

impl<T: Writable> Writable for &[T] {
    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {
        self.len().write_var_to(writer)?;
        for item in self.iter() {
            item.write_to(writer)?;
        }
        Ok(())
    }
}

impl<T: Writable> Writable for Vec<T> {
    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {
        self.as_slice().write_to(writer)
    }
}

#[cfg(feature = "uuid")]
impl Writable for Uuid {
    fn write_to(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u128(self.as_u128())
    }
}
