use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};

mod matrix_bench;
use matrix_bench::*;


fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("sum");
	group.sample_size(10);
	group.measurement_time(Duration::from_secs(15));
    group.bench_function("big random sum", |b| b.iter(|| sum_bench()));
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);