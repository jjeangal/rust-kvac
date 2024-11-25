use criterion::{criterion_group, criterion_main, Criterion};
use rust_kvac::proof_update::proof_update;

pub fn benchmark_proof_update(c: &mut Criterion) {
    // c.bench_function("proof_update", |b| b.iter(|| proof_update()));
}

criterion_group!(benches, benchmark_proof_update);
criterion_main!(benches);
