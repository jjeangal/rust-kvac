use crate::benches::utils::{initial_commitment, KEYS_10K};
use criterion::{black_box, criterion_group, BatchSize, BenchmarkId, Criterion};
use rust_kvac::insert::insert;

fn benchmark_insert_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_scaling");

    for size in [100, 500, 1000].iter() {
        let mut initial_commitment = initial_commitment();
        for kv in KEYS_10K.iter().take(*size) {
            let (new_commitment, _, _) = insert(&initial_commitment, kv);
            initial_commitment = new_commitment;
        }

        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter_batched(
                || initial_commitment.clone(),
                |commitment| {
                    let mut current = commitment;
                    for kv in KEYS_10K.iter().skip(size).take(100) {
                        let (new_commitment, _, _) = black_box(insert(&current, kv));
                        current = new_commitment;
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(
    name = benchmarks;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = benchmark_insert_scaling
);
