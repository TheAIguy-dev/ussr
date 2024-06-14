use crate::{Size, VarSize};

pub const MAX_STRING_LENGTH: usize = 32767;

macro_rules! impl_size {
    ($($type:ty),*) => {
        $(
            impl Size for $type {
                const SIZE: usize = std::mem::size_of::<$type>();
            }
        )*
    };
}
impl_size!(bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl VarSize for u32 {
    const MIN_SIZE: usize = 1;
    const MAX_SIZE: usize = 5;
}
impl VarSize for i32 {
    const MIN_SIZE: usize = 1;
    const MAX_SIZE: usize = 5;
}

impl VarSize for u64 {
    const MIN_SIZE: usize = 1;
    const MAX_SIZE: usize = 10;
}
impl VarSize for i64 {
    const MIN_SIZE: usize = 1;
    const MAX_SIZE: usize = 10;
}

impl VarSize for usize {
    const MIN_SIZE: usize = 1;
    /// Limited to 3 bytes because lengths can only have that many.
    const MAX_SIZE: usize = 3;
}

impl VarSize for String {
    const MIN_SIZE: usize = usize::MIN_SIZE + 0;
    const MAX_SIZE: usize = usize::MAX_SIZE + MAX_STRING_LENGTH * 3;
}
