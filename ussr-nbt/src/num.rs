use bytemuck::Pod;

mod private {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// A trait for swapping the endianness of a number.
pub trait Num: private::Sealed + Pod {
    fn swap_bytes(self) -> Self;
}

macro_rules! impl_num {
    ($($type:ty),*) => {
        $(
            impl Num for $type {
                fn swap_bytes(self) -> Self {
                    self.swap_bytes()
                }
            }
        )*
    };
}
use impl_num;

impl_num!(i16, i32, i64);
impl Num for f32 {
    fn swap_bytes(self) -> Self {
        f32::from_bits(self.to_bits().swap_bytes())
    }
}
impl Num for f64 {
    fn swap_bytes(self) -> Self {
        f64::from_bits(self.to_bits().swap_bytes())
    }
}
