use std::{future::Future, io};

use futures_lite::AsyncWriteExt;
use paste::paste;
use ussr_nbt::owned::Nbt;
use uuid::Uuid;

pub trait Encode {
    fn encode(
        &self,
        writer: &mut (impl AsyncWriteExt + Unpin + Send),
    ) -> impl Future<Output = io::Result<()>> + Send;
}

pub trait VarEncode {
    fn var_encode(
        &self,
        writer: &mut (impl AsyncWriteExt + Unpin + Send),
    ) -> impl Future<Output = io::Result<()>> + Send;
}

pub trait EncodeExt {
    fn encode<T: Encode + Sync + Send>(
        &mut self,
        value: T,
    ) -> impl Future<Output = io::Result<()>> + Send;
    fn var_encode<T: VarEncode + Sync + Send>(
        &mut self,
        value: T,
    ) -> impl Future<Output = io::Result<()>> + Send;
}

impl<W: AsyncWriteExt + Unpin + Send> EncodeExt for W {
    async fn encode<T: Encode + Sync + Send>(&mut self, value: T) -> io::Result<()> {
        value.encode(self).await
    }
    async fn var_encode<T: VarEncode + Sync + Send>(&mut self, value: T) -> io::Result<()> {
        value.var_encode(self).await
    }
}

#[macro_export]
macro_rules! async_encode_array {
    ($array:expr, $writer:expr, $f:expr, $g:expr) => {};
}

macro_rules! impl_encode {
    ($($type:ty),*) => {
        paste! {
            $(
                impl Encode for $type {
                    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
                        writer.write_all(&self.to_be_bytes()).await
                    }
                }
            )*
        }
    };
}
impl_encode!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl Encode for bool {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.encode(*self as u8).await
    }
}

impl VarEncode for u32 {
    async fn var_encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        let mut value: u32 = *self;

        loop {
            let byte: u8 = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                writer.encode(byte | 0x80).await?;
            } else {
                writer.encode(byte).await?;
                return Ok(());
            }
        }
    }
}

impl VarEncode for i32 {
    async fn var_encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.var_encode(*self as u32).await
    }
}

impl VarEncode for u64 {
    async fn var_encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        let mut value: u64 = *self;

        loop {
            let byte: u8 = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                writer.encode(byte | 0x80).await?;
            } else {
                writer.encode(byte).await?;
                return Ok(());
            }
        }
    }
}

impl VarEncode for i64 {
    async fn var_encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.var_encode(*self as u64).await
    }
}

impl VarEncode for usize {
    /// Convenience implementation. Uses `u32::var_encode` (VarInt).
    async fn var_encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.var_encode(*self as u32).await
    }
}

impl Encode for str {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.var_encode(self.len() as u32).await?;
        writer.write_all(self.as_bytes()).await
    }
}

impl Encode for String {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        self.as_str().encode(writer).await
    }
}

impl Encode for Nbt {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        todo!()
    }
}

impl Encode for Uuid {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.encode(self.as_u128()).await
    }
}

impl<T: Encode + Sync> Encode for &T {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        (*self).encode(writer).await
    }
}

impl<T: VarEncode + Sync> VarEncode for &T {
    async fn var_encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        (*self).var_encode(writer).await
    }
}

impl<T: Encode + Sync> Encode for [T] {
    /// Will use `VarEncode` for the length and `Encode` for the elements.
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.var_encode(self.len()).await?;

        for value in self.iter() {
            writer.encode(value).await?;
        }

        Ok(())
    }
}

impl<T: Encode + Sync> Encode for Option<T> {
    async fn encode(&self, writer: &mut (impl AsyncWriteExt + Unpin + Send)) -> io::Result<()> {
        writer.encode(self.is_some()).await?;

        if let Some(value) = self {
            writer.encode(value).await?;
        }

        Ok(())
    }
}
