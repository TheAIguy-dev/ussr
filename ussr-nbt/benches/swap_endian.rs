// #![feature(portable_simd)]

mod num {
    include!("../src/num.rs");
}
mod swap_endian {
    include!("../src/swap_endian.rs");
}
include!("../src/endian.rs");

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};

use swap_endian::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

macro_rules! swap_endian_bench {
    ($group:expr, $input:expr => $($name:ident),*) => {
        paste::paste! {
            $(
                $group.bench_function(stringify!($name), |b| {
                    b.iter_batched_ref(
                        || $input.clone(),
                        #[allow(unused_unsafe)]
                        |input| unsafe { [<swap_endian_ $name>](black_box(input)) },
                        BatchSize::SmallInput,
                    )
                });
            )*
        }
    };
}
use swap_endian_bench;

// #[inline]
// fn swap_endianness_32bit(bytes: &mut [u8], num: usize) {
//     use std::simd::prelude::*;
//
//     for i in 0..num / 16 {
//         let simd: u8x64 = Simd::from_slice(bytes[i * 16 * 4..(i + 1) * 16 * 4].as_ref());
//         #[rustfmt::skip]
//         let simd = simd_swizzle!(simd, [
//             3, 2, 1, 0,
//             7, 6, 5, 4,
//             11, 10, 9, 8,
//             15, 14, 13, 12,
//             19, 18, 17, 16,
//             23, 22, 21, 20,
//             27, 26, 25, 24,
//             31, 30, 29, 28,
//             35, 34, 33, 32,
//             39, 38, 37, 36,
//             43, 42, 41, 40,
//             47, 46, 45, 44,
//             51, 50, 49, 48,
//             55, 54, 53, 52,
//             59, 58, 57, 56,
//             63, 62, 61, 60,
//         ]);
//         bytes[i * 16 * 4..(i + 1) * 16 * 4].copy_from_slice(simd.as_array());
//     }
//
//     let mut i = num / 16 * 16;
//     if i + 8 <= num {
//         let simd: u8x32 = Simd::from_slice(bytes[i * 4..i * 4 + 32].as_ref());
//         #[rustfmt::skip]
//         let simd = simd_swizzle!(simd, [
//             3, 2, 1, 0,
//             7, 6, 5, 4,
//             11, 10, 9, 8,
//             15, 14, 13, 12,
//             19, 18, 17, 16,
//             23, 22, 21, 20,
//             27, 26, 25, 24,
//             31, 30, 29, 28,
//         ]);
//         bytes[i * 4..i * 4 + 32].copy_from_slice(simd.as_array());
//         i += 8;
//     }
//     if i + 4 <= num {
//         let simd: u8x16 = Simd::from_slice(bytes[i * 4..i * 4 + 16].as_ref());
//         #[rustfmt::skip]
//         let simd = simd_swizzle!(simd, [
//             3, 2, 1, 0,
//             7, 6, 5, 4,
//             11, 10, 9, 8,
//             15, 14, 13, 12,
//         ]);
//         bytes[i * 4..i * 4 + 16].copy_from_slice(simd.as_array());
//         i += 4;
//     }
//     if i + 2 <= num {
//         let simd: u8x8 = Simd::from_slice(bytes[i * 4..i * 4 + 8].as_ref());
//         #[rustfmt::skip]
//         let simd = simd_swizzle!(simd, [
//             3, 2, 1, 0,
//             7, 6, 5, 4,
//         ]);
//         bytes[i * 4..i * 4 + 8].copy_from_slice(simd.as_array());
//         i += 2;
//     }
//     if i < num {
//         let simd: u8x4 = Simd::from_slice(bytes[i * 4..i * 4 + 4].as_ref());
//         #[rustfmt::skip]
//         let simd = simd_swizzle!(simd, [
//             3, 2, 1, 0,
//         ]);
//         bytes[i * 4..i * 4 + 4].copy_from_slice(simd.as_array());
//     }
// }

fn bench(c: &mut Criterion) {
    let input: Vec<i32> = (0..2i32.pow(22)).collect();

    let mut group = c.benchmark_group("swap_endian");
    group.throughput(Throughput::Elements(input.len() as u64));

    // My cpu has all the features so it's safe to use them
    swap_endian_bench!(group, input => avx2, fallback);

    // group.bench_function("simdnbt", |b| {
    //     b.iter_batched_ref(
    //         || (input.clone(), input.len()),
    //         |(input, len)| unsafe {
    //             swap_endianness_32bit(black_box(bytemuck::cast_slice_mut(input)), black_box(*len))
    //         },
    //         BatchSize::SmallInput,
    //     )
    // });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10_000).measurement_time(Duration::from_secs(30));
    targets = bench
}
criterion_main!(benches);
