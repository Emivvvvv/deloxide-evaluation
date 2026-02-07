// Micro-benchmark: RwLock read/write with random stress testing
// Tests the overhead of random stress mode with different configurations on RwLock operations.
// Sample size: 100 (fast micro-benchmark)

use criterion::{criterion_group, criterion_main, Criterion};
use deloxide::{Deloxide, RwLock, };
use std::sync::Arc;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(100)
}

fn start_random_stress_default() {
    Deloxide::new().with_random_stress().start().unwrap();
}

// Read benchmarks with different s

fn bench_read_stress_default(c: &mut Criterion) {
    start_random_stress_default();
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("rwlock_read_stress_random_default", |b| {
        b.iter(|| {
            let _g = l.read();
        });
    });
}

fn bench_write_stress_default(c: &mut Criterion) {
    start_random_stress_default();
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("rwlock_write_stress_random_default", |b| {
        b.iter(|| {
            let mut g = l.write();
            *g += 1;
        });
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = 
        bench_read_stress_default,
        bench_write_stress_default
);

criterion_main!(benches);
