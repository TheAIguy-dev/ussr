use crate::num::Num;

macro_rules! swap_endian_impl {
    ($($name:ident: $feature:literal),* $(,)?) => {
        paste::paste! {
            $(
                #[target_feature(enable = $feature)]
                unsafe fn [<swap_endian_ $name>]<T: Num>(s: &mut [T]) {
                    for i in s.iter_mut() {
                        *i = i.swap_bytes();
                    }
                }
            )*
        }
    };
}

macro_rules! swap_endian_use {
    ($check:ident, $slice:expr, $($name:ident: $feature:literal),* $(,)?) => {
        paste::paste! {
            $(
                if $check!($feature) {
                    return unsafe { [<swap_endian_ $name>]($slice) };
                }
            )*
        }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
swap_endian_impl! {
    avx2: "avx2",
    avx: "avx",
    sse4_2: "sse4.2",
    sse4_1: "sse4.1",
    ssse3: "ssse3",
    sse3: "sse3",
    sse2: "sse2",
    sse: "sse",
}

fn swap_endian_fallback<T: Num>(slice: &mut [T]) {
    for i in slice.iter_mut() {
        *i = i.swap_bytes();
    }
}

pub fn swap_endian<T: Num>(slice: &mut [T]) {
    #[cfg(feature = "rt_cpu_feat")]
    {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            swap_endian_use! {
                is_x86_feature_detected, slice,

                avx2: "avx2",
                avx: "avx",
                sse4_2: "sse4.2",
                sse4_1: "sse4.1",
                ssse3: "ssse3",
                sse3: "sse3",
                sse2: "sse2",
                sse: "sse",
            }
        }
    }

    // TODO: aarch64
    // TODO: riscv32/riscv64
    // TODO: wasm32/wasm64
    //? https://doc.rust-lang.org/reference/attributes/codegen.html#the-target_feature-attribute

    swap_endian_fallback(slice);
}
