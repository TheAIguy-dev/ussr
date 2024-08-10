use std::io::Cursor;

use paste::paste;

pub trait Writer {
    fn write_slice(&mut self, slice: &[u8]);

    write_ty!(u8, u16, u32, i16, i32, i64, f32, f64);
}

impl Writer for Vec<u8> {
    #[inline]
    fn write_slice(&mut self, slice: &[u8]) {
        self.extend_from_slice(slice);
    }
}

impl Writer for Cursor<Vec<u8>> {
    #[inline]
    fn write_slice(&mut self, slice: &[u8]) {
        self.get_mut().extend_from_slice(slice);
    }
}

macro_rules! write_ty {
    ($($type:ty),*) => {
        paste! {
            $(
                #[inline]
                fn [<write_ $type>](&mut self, value: $type) {
                    self.write_slice(&value.to_be_bytes());
                }
            )*
        }
    };
}
use write_ty;
