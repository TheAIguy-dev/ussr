use std::{
    io::{Cursor, Read},
    time::Duration,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;

fn bench_file(filename: &str, c: &mut Criterion) {
    let contents: Vec<u8> = std::fs::read(format!("tests/{filename}")).unwrap();
    let mut src: &[u8] = &contents[..];

    let mut src_decoder: GzDecoder<&mut &[u8]> = GzDecoder::new(&mut src);
    let mut input: Vec<u8> = Vec::new();
    if src_decoder.read_to_end(&mut input).is_err() {
        input = contents;
    }

    let mut input_stream: Cursor<&[u8]> = Cursor::new(&input[..]);

    let mut group = c.benchmark_group(format!("compare/{filename}/read"));
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("ussr_borrow", |b| {
        b.iter(|| {
            black_box(ussr_nbt::borrow::Nbt::read(&mut input_stream).unwrap());
            input_stream.set_position(0);
        })
    });
    group.bench_function("ussr_owned", |b| {
        b.iter(|| {
            black_box(ussr_nbt::owned::Nbt::read(&mut input_stream).unwrap());
            input_stream.set_position(0);
        })
    });
    // group.bench_function("simdnbt_borrow", |b| {
    //     b.iter(|| {
    //         black_box(simdnbt::borrow::read(&mut input_stream).unwrap());
    //         input_stream.set_position(0);
    //     })
    // });
    // group.bench_function("simdnbt_owned", |b| {
    //     b.iter(|| {
    //         black_box(simdnbt::owned::read(&mut input_stream).unwrap());
    //         input_stream.set_position(0);
    //     })
    // });
    // group.bench_function("ferrumc", |b| {
    //     b.iter(|| {
    //         let mut tape = ferrumc_nbt::NbtTape::new(&mut &input[..]);
    //         tape.parse();
    //         black_box(tape)
    //     })
    // });
    // group.bench_function("shen", |b| {
    //     let mut input = input.to_vec();
    //     b.iter(|| {
    //         black_box(
    //             shen_nbt5::NbtValue::from_binary::<shen_nbt5::nbt_version::Java>(&mut input)
    //                 .unwrap(),
    //         );
    //     })
    // });
    // group.bench_function("azalea", |b| {
    //     b.iter(|| {
    //         black_box(azalea_nbt::Nbt::read(&mut input_stream).unwrap());
    //         input_stream.set_position(0);
    //     })
    // });
    // group.bench_function("graphite", |b| {
    //     b.iter(|| {
    //         black_box(graphite_binary::nbt::decode::read(&mut &input[..]).unwrap());
    //     })
    // });
    // group.bench_function("valence", |b| {
    //     b.iter(|| {
    //         black_box(valence_nbt::from_binary::<String>(&mut &input[..]).unwrap());
    //     })
    // });
    // group.bench_function("fastnbt", |b| {
    //     b.iter(|| {
    //         black_box(fastnbt::from_bytes::<fastnbt::Value>(&input).unwrap());
    //     })
    // });
    // group.bench_function("hematite", |b| {
    //     b.iter(|| {
    //         black_box(nbt::Blob::from_reader(&mut input_stream).unwrap());
    //         input_stream.set_position(0);
    //     })
    // });
    // group.bench_function("crab", |b| {
    //     b.iter(|| {
    //         black_box(crab_nbt::Nbt::read(&mut input_stream).unwrap());
    //         input_stream.set_position(0);
    //     })
    // });
    // group.bench_function("quartz", |b| {
    //     b.iter(|| {
    //         black_box(
    //             quartz_nbt::io::read_nbt(&mut input_stream, quartz_nbt::io::Flavor::Uncompressed)
    //                 .unwrap(),
    //         );
    //         input_stream.set_position(0);
    //     })
    // });

    // Removed because it's not working correctly ¯\_(ツ)_/¯
    // group.bench_function("golden_apple", |b| {
    //     b.iter(|| {
    //         black_box(golden_apple::nbt::from_reader(&mut input_stream).unwrap());
    //         input_stream.set_position(0);
    //     })
    // });

    group.finish();

    let mut group = c.benchmark_group(format!("compare/{filename}/write"));
    group.throughput(Throughput::Elements(1));
    // group.throughput(Throughput::Bytes(input.len() as u64));

    let nbt = ussr_nbt::borrow::Nbt::read(&mut Cursor::new(&input)).unwrap();
    group.bench_function("ussr_borrow", |b| {
        b.iter(|| {
            let mut out: Vec<u8> = Vec::new();
            nbt.write(&mut out).unwrap();
            black_box(out);
        })
    });

    let nbt = ussr_nbt::owned::Nbt::read(&mut Cursor::new(&input)).unwrap();
    group.bench_function("ussr_owned", |b| {
        b.iter(|| {
            let mut out: Vec<u8> = Vec::new();
            nbt.write(&mut out).unwrap();
            black_box(out);
        })
    });

    // let nbt = simdnbt::borrow::read(&mut Cursor::new(&input))
    //     .unwrap()
    //     .unwrap();
    // group.bench_function("simdnbt_borrow", |b| {
    //     b.iter(|| {
    //         let mut out: Vec<u8> = Vec::new();
    //         nbt.write(&mut out);
    //         black_box(out);
    //     })
    // });

    // let nbt = simdnbt::owned::read(&mut Cursor::new(&input))
    //     .unwrap()
    //     .unwrap();
    // group.bench_function("simdnbt_owned", |b| {
    //     b.iter(|| {
    //         let mut out: Vec<u8> = Vec::new();
    //         nbt.write(&mut out);
    //         black_box(out);
    //     })
    // });

    let nbt = shen_nbt5::NbtValue::from_binary::<shen_nbt5::nbt_version::Java>(&mut input).unwrap();
    group.bench_function("shen", |b| {
        b.iter(|| {
            black_box(nbt.to_binary::<shen_nbt5::nbt_version::Java>().unwrap());
        })
    });

    // let nbt = azalea_nbt::Nbt::read(&mut Cursor::new(&input)).unwrap();
    // group.bench_function("azalea", |b| {
    //     b.iter(|| {
    //         let mut out: Vec<u8> = Vec::new();
    //         nbt.write(&mut out);
    //         black_box(out);
    //     })
    // });

    let nbt = graphite_binary::nbt::decode::read(&mut &input[..]).unwrap();
    group.bench_function("graphite", |b| {
        b.iter(|| {
            let out: Vec<u8> = graphite_binary::nbt::encode::write(&nbt);
            black_box(out);
        })
    });

    // Removed because too slow and messing up my plot
    // let nbt = valence_nbt::from_binary::<String>(&mut &input[..]).unwrap();
    // group.bench_function("valence", |b| {
    //     b.iter(|| {
    //         let mut out: Vec<u8> = Vec::new();
    //         valence_nbt::to_binary(&nbt.0, &mut out, &nbt.1).unwrap();
    //         black_box(out);
    //     })
    // });

    // Removed because too slow and messing up my plot
    // let nbt = fastnbt::from_bytes::<fastnbt::Value>(&input).unwrap();
    // group.bench_function("fastnbt", |b| {
    //     b.iter(|| {
    //         let mut out: Vec<u8> = Vec::new();
    //         fastnbt::to_writer(&mut out, &nbt).unwrap();
    //         black_box(out);
    //     })
    // });

    // Removed because too slow and messing up my plot
    // let nbt = nbt::Blob::from_reader(&mut Cursor::new(&input)).unwrap();
    // group.bench_function("hematite", |b| {
    //     b.iter(|| {
    //         let mut out: Vec<u8> = Vec::new();
    //         nbt.to_writer(&mut out).unwrap();
    //         black_box(out);
    //     })
    // });

    // Removed because too slow and messing up my plot
    // let nbt = crab_nbt::Nbt::read(&mut Cursor::new(&input)).unwrap();
    // group.bench_function("crab", |b| {
    //     b.iter(|| {
    //         black_box(nbt.write());
    //     })
    // });

    // Removed because too slow and messing up my plot
    // let nbt = quartz_nbt::io::read_nbt(
    //     &mut Cursor::new(&input),
    //     quartz_nbt::io::Flavor::Uncompressed,
    // )
    // .unwrap();
    // group.bench_function("quartz", |b| {
    //     b.iter(|| {
    //         let mut out: Vec<u8> = Vec::new();
    //         quartz_nbt::io::write_nbt(
    //             &mut out,
    //             Some(&nbt.1),
    //             &nbt.0,
    //             quartz_nbt::io::Flavor::Uncompressed,
    //         )
    //         .unwrap();
    //         black_box(out);
    //     })
    // });

    // Removed because it's not working correctly ¯\_(ツ)_/¯
    // let nbt = golden_apple::nbt::from_reader(&mut Cursor::new(&input)).unwrap();
    // group.bench_function("golden_apple", |b| {
    //     b.iter(|| {
    //         black_box(golden_apple::nbt::to_bytes(nbt.clone()).unwrap());
    //     })
    // });

    group.finish();
}

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn bench(c: &mut Criterion) {
    bench_file("TheAIguy_.nbt", c);
}

criterion_group! {
    name = compare;
    config = Criterion::default()
                .warm_up_time(Duration::from_secs(5))
                // .measurement_time(Duration::from_secs(60))
                .sample_size(10_000);
    targets = bench
}
// criterion_group!(compare, bench);
criterion_main!(compare);
