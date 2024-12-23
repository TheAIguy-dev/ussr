use std::{
    io::{Cursor, Read},
    time::Duration,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;

fn bench_write_file(filename: &str, c: &mut Criterion) {
    let contents: Vec<u8> = std::fs::read(format!("tests/{filename}")).unwrap();
    let mut src: &[u8] = &contents[..];

    let mut src_decoder: GzDecoder<&mut &[u8]> = GzDecoder::new(&mut src);
    let mut input: Vec<u8> = Vec::new();
    if src_decoder.read_to_end(&mut input).is_err() {
        input = contents;
    }

    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> =
        c.benchmark_group(format!("write/{filename}"));
    group.throughput(Throughput::Bytes(input.len() as u64));

    let nbt = ussr_nbt::borrow::Nbt::read(&mut Cursor::new(&input)).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    nbt.write(&mut buf).unwrap();
    group.bench_function("ussr_borrow", |b| {
        b.iter(|| {
            buf.clear();
            black_box(nbt.write(&mut buf))
        })
    });

    let nbt = ussr_nbt::owned::Nbt::read(&mut Cursor::new(&input)).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    nbt.write(&mut buf).unwrap();
    group.bench_function("ussr_owned", |b| {
        b.iter(|| {
            buf.clear();
            black_box(nbt.write(&mut buf))
        })
    });
}

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn bench(c: &mut Criterion) {
    bench_write_file("TheAIguy_.nbt", c);
}

criterion_group! {
    name = compare;
    config = Criterion::default().warm_up_time(Duration::from_secs(5)).measurement_time(Duration::from_secs(60)).sample_size(10_000);
    targets = bench
}
// criterion_group!(compare, bench);
criterion_main!(compare);
