use std::{io::Cursor, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ussr_buf::{VarReadable, VarWritable};

fn bench_read(c: &mut Criterion) {
    c.bench_function("read_var", |b| {
        let buf: Vec<u8> = vec![239, 155, 175, 205, 248, 172, 209, 145, 1];
        b.iter(|| black_box(u64::read_var_from(&mut Cursor::new(&buf)).unwrap()))
    });
}

fn bench_write(c: &mut Criterion) {
    c.bench_function("write_var", |b| {
        let mut buf: Vec<u8> = Vec::with_capacity(10);
        b.iter(|| {
            buf.clear();
            black_box(0x123456789ABCDEFu64.write_var_to(&mut buf).unwrap())
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).sample_size(10000);
    targets = bench_read, bench_write
}
criterion_main!(benches);
