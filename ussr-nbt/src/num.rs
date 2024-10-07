//! A trait for swapping the endianness of a number.

mod private {
    pub trait Sealed {}

    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// A trait for swapping the endianness of a number.
pub trait Num: private::Sealed + bytemuck::Pod {
    #[must_use]
    fn swap_bytes(self) -> Self;
}

impl Num for i16 {
    #[inline]
    fn swap_bytes(self) -> Self {
        self.swap_bytes()
    }
}
impl Num for i32 {
    #[inline]
    fn swap_bytes(self) -> Self {
        self.swap_bytes()
    }
}
impl Num for i64 {
    #[inline]
    fn swap_bytes(self) -> Self {
        self.swap_bytes()
    }
}
impl Num for f32 {
    #[inline]
    fn swap_bytes(self) -> Self {
        f32::from_bits(self.to_bits().swap_bytes())
    }
}
impl Num for f64 {
    #[inline]
    fn swap_bytes(self) -> Self {
        f64::from_bits(self.to_bits().swap_bytes())
    }
}
