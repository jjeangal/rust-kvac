use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(_c: &mut Criterion) {
    //c.bench_function("verify", |b| b.iter(|| insert()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
