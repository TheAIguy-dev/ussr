use std::io::{self, Write};

use byteorder::{WriteBytesExt, BE};
use paste::paste;
use ussr_nbt::{owned::Nbt, EncodeOpts};
use uuid::Uuid;

pub trait Encode {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()>;
}

pub trait VarEncode {
    fn var_encode(&self, writer: &mut impl Write) -> io::Result<()>;
}

pub trait EncodeExt {
    fn encode<T: Encode>(&mut self, value: T) -> io::Result<()>;
    fn var_encode<T: VarEncode>(&mut self, value: T) -> io::Result<()>;
}

impl<W: Write> EncodeExt for W {
    fn encode<T: Encode>(&mut self, value: T) -> io::Result<()> {
        value.encode(self)
    }
    fn var_encode<T: VarEncode>(&mut self, value: T) -> io::Result<()> {
        value.var_encode(self)
    }
}

macro_rules! impl_encode {
    ($($type:ty),*) => {
        paste! {
            $(
                impl Encode for $type {
                    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
                        writer.write_all(&self.to_be_bytes())
                    }
                }
            )*
        }
    };
}
impl_encode!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl Encode for bool {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(*self as u8)
    }
}

impl VarEncode for u32 {
    fn var_encode(&self, writer: &mut impl Write) -> io::Result<()> {
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

impl VarEncode for i32 {
    fn var_encode(&self, writer: &mut impl Write) -> io::Result<()> {
        u32::var_encode(&(*self as u32), writer)
    }
}

impl VarEncode for u64 {
    fn var_encode(&self, writer: &mut impl Write) -> io::Result<()> {
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

impl VarEncode for i64 {
    fn var_encode(&self, writer: &mut impl Write) -> io::Result<()> {
        u64::var_encode(&(*self as u64), writer)
    }
}

impl VarEncode for usize {
    /// Convenience implementation. Uses `u32::var_encode` (VarInt).
    fn var_encode(&self, writer: &mut impl Write) -> io::Result<()> {
        u32::var_encode(&(*self as u32), writer)
    }
}

impl Encode for str {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.var_encode(self.len() as u32)?;
        writer.write_all(self.as_bytes())
    }
}

impl Encode for String {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        self.as_str().encode(writer)
    }
}

impl Encode for Nbt {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        self.write_with_opts(writer, EncodeOpts::nameless())
    }
}

impl Encode for Uuid {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u128::<BE>(self.as_u128())
    }
}

impl<T: Encode> Encode for &T {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        (*self).encode(writer)
    }
}

impl<T: VarEncode> VarEncode for &T {
    fn var_encode(&self, writer: &mut impl Write) -> io::Result<()> {
        (*self).var_encode(writer)
    }
}

impl<T: Encode> Encode for [T] {
    /// Will use `VarEncode` for the length and `Encode` for the elements.
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        self.len().var_encode(writer)?;
        self.iter().try_for_each(|item| item.encode(writer))
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        self.is_some().encode(writer)?;
        self.as_ref().map_or(Ok(()), |value| value.encode(writer))
    }
}
