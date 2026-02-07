// Micro-benchmark: Single lock/unlock with random stress testing
// Tests the overhead of random stress mode with different configurations.
// Sample size: 100 (fast micro-benchmark)

use criterion::{criterion_group, criterion_main, Criterion};
use deloxide::{Deloxide, Mutex, StressConfig};
use std::sync::Arc;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(100)
}

// Default StressConfig
fn bench_stress_aggressive(c: &mut Criterion) {
    Deloxide::new()
        .with_random_stress()
        .with_stress_config(StressConfig::aggressive())
        .start()
        .unwrap();
    let m = Arc::new(Mutex::new(0));
    c.bench_function("single_lock_stress_random_aggressive", |b| {
        b.iter(|| {
            let _lock = m.lock();
        });
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = 
        bench_stress_aggressive
);

criterion_main!(benches);
