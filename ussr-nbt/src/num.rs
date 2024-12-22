mod private {
    pub trait Sealed {}

    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

pub trait Num: private::Sealed + bytemuck::Pod {
    #[must_use]
    fn to_be(self) -> Self;
}

impl Num for i16 {
    #[inline]
    fn to_be(self) -> Self {
        self.to_be()
    }
}
impl Num for i32 {
    #[inline]
    fn to_be(self) -> Self {
        self.to_be()
    }
}
impl Num for i64 {
    #[inline]
    fn to_be(self) -> Self {
        self.to_be()
    }
}
impl Num for f32 {
    #[inline]
    fn to_be(self) -> Self {
        f32::from_bits(self.to_bits().to_be())
    }
}
impl Num for f64 {
    #[inline]
    fn to_be(self) -> Self {
        f64::from_bits(self.to_bits().to_be())
    }
}
