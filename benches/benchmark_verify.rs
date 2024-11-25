use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark_verify(c: &mut Criterion) {
    //c.bench_function("verify", |b| b.iter(|| verify()));
}

criterion_group!(benches, benchmark_verify);
criterion_main!(benches);
