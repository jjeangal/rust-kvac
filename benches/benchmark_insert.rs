use criterion::{criterion_group, criterion_main, Criterion};
use rust_kvac::insert::insert;

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("verify", |b| b.iter(|| insert()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
