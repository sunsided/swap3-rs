use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use swap3::slice;

pub fn criterion_benchmark(c: &mut Criterion) {
    let indexes = get_indexes(42);

    c.bench_function("bca_safe", |bencher| {
        let mut values = black_box(get_values());
        bencher.iter(|| {
            for (a, b, c) in indexes.iter().cloned() {
                slice::bca_safe(&mut values, a, b, c)
            }
        })
    });

    #[cfg(feature = "unsafe")]
    c.bench_function("bca_unsafe", |bencher| {
        let mut values = black_box(get_values());
        bencher.iter(|| {
            for (a, b, c) in indexes.iter().cloned() {
                slice::bca_unsafe(&mut values, a, b, c)
            }
        })
    });

    c.bench_function("cab_safe", |bencher| {
        let mut values = black_box(get_values());
        bencher.iter(|| {
            for (a, b, c) in indexes.iter().cloned() {
                slice::cab_safe(&mut values, a, b, c)
            }
        })
    });

    #[cfg(feature = "unsafe")]
    c.bench_function("cab_unsafe", |bencher| {
        let mut values = black_box(get_values());
        bencher.iter(|| {
            for (a, b, c) in indexes.iter().cloned() {
                slice::cab_unsafe(&mut values, a, b, c)
            }
        })
    });
}

fn get_values() -> Vec<u64> {
    (0..100).map(|v| v + 1000).collect()
}

fn get_indexes(seed: u64) -> Vec<(usize, usize, usize)> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..100)
        .map(|_| {
            let a = rng.gen_range(0..100);
            let mut b = rng.gen_range(0..100);
            while b == a {
                b = rng.gen_range(0..100);
            }
            let mut c = rng.gen_range(0..100);
            while c == a || c == b {
                c = rng.gen_range(0..100);
            }
            (a, b, c)
        })
        .collect()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
