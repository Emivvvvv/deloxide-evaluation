// Micro-benchmark: RwLock read/write with random stress testing
// Tests the overhead of random stress mode with different configurations on RwLock operations.
// Sample size: 100 (fast micro-benchmark)

use criterion::{criterion_group, criterion_main, Criterion};
use deloxide::{Deloxide, RwLock, StressConfig};
use std::sync::Arc;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(100)
}



fn start_random_stress_with(config: StressConfig) {
    Deloxide::new()
        .with_random_stress()
        .with_stress_config(config)
        .start()
        .unwrap();
}

// Read benchmarks with different StressConfigs

fn bench_read_stress_gentle(c: &mut Criterion) {
    start_random_stress_with(StressConfig::gentle());
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("rwlock_read_stress_random_gentle", |b| {
        b.iter(|| {
            let _g = l.read();
        });
    });
}

fn bench_write_stress_gentle(c: &mut Criterion) {
    start_random_stress_with(StressConfig::gentle());
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("rwlock_write_stress_random_gentle", |b| {
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
        bench_read_stress_gentle,
        bench_write_stress_gentle
);

criterion_main!(benches);
