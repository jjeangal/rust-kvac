use criterion::{criterion_group, criterion_main, Criterion};
use rust_kvac::verify::verify;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("verify", |b| b.iter(|| verify()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
