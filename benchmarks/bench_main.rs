use criterion::criterion_main;

mod benches;
criterion_main! {
    benches::insert_bench::benchmarks,
    benches::proof_bench::benchmarks,
}
