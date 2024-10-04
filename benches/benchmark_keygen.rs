use criterion::{criterion_group, criterion_main, Criterion};
use rust_kvac::keygen::keygen;

pub fn benchmark_keygen(c: &mut Criterion) {
    c.bench_function("keygen", |b| b.iter(|| keygen()));
}

criterion_group!(benches, benchmark_keygen);
criterion_main!(benches);
