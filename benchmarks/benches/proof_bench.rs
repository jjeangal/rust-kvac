use crate::benches::utils::{initial_commitment, KEYS_10K, SAME_KEY_VALUES_10K};
use criterion::{black_box, criterion_group, BatchSize, BenchmarkId, Criterion};
use rust_kvac::insert::insert;
use rust_kvac::proof_update::proof_update;

pub fn benchmark_proof_same_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("proof_same_key");

    let initial_kv = &SAME_KEY_VALUES_10K[0];
    let initial_commitment = initial_commitment();
    let (_, proof, _) = insert(&initial_commitment, initial_kv);

    // Single update
    group.bench_function("single_update", |b| {
        b.iter(|| {
            black_box(proof_update(
                &SAME_KEY_VALUES_10K[1].key(),
                &proof,
                &SAME_KEY_VALUES_10K[1],
            ))
        })
    });

    // Sequential updates (same key)
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, &updates| {
            b.iter_batched(
                || (proof.clone()),
                |mut prf| {
                    for kv in SAME_KEY_VALUES_10K.iter().take(updates).skip(1) {
                        prf = black_box(
                            proof_update(&kv.key(), &prf, &kv).expect("Proof update failed"),
                        );
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

pub fn benchmark_proof_different_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("proof_different_key");

    let initial_kv = &KEYS_10K[0];
    let initial_commitment = initial_commitment();
    let (_, proof, _) = insert(&initial_commitment, initial_kv);

    // Single update with different key
    group.bench_function("single_update", |b| {
        b.iter(|| black_box(proof_update(&KEYS_10K[0].key(), &proof, &KEYS_10K[1])))
    });

    // Sequential updates with different keys
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, &updates| {
            b.iter_batched(
                || proof.clone(),
                |mut prf| {
                    for kv in KEYS_10K.iter().take(updates).skip(1) {
                        prf = black_box(
                            proof_update(&KEYS_10K[0].key(), &prf, kv)
                                .expect("Proof update failed"),
                        );
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
    targets = benchmark_proof_same_key, benchmark_proof_different_key
);
