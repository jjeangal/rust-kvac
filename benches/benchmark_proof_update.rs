use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark_proof_update(_c: &mut Criterion) {
    // c.bench_function("proof_update", |b| b.iter(|| proof_update()));
}

criterion_group!(benches, benchmark_proof_update);
criterion_main!(benches);
