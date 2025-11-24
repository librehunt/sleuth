//! Performance benchmarks for scanner

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_scanner(_c: &mut Criterion) {
    // TODO: Implement benchmarks
}

criterion_group!(benches, bench_scanner);
criterion_main!(benches);
