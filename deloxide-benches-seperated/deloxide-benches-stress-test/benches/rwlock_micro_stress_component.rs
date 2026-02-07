// Micro-benchmark: RwLock read/write with component stress testing
// Tests the overhead of component stress mode on RwLock operations.
// Sample size: 100 (fast micro-benchmark)

use criterion::{criterion_group, Criterion};
use std::sync::Arc;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(100)
}

fn bench_deloxide_rwlock_read_lock_unlock_with_component_stress(c: &mut Criterion) {
    use deloxide::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("deloxide_rwlock_read_lock_unlock_with_component_stress", |b| {
        b.iter(|| {
            let _g = l.read();
        });
    });
}

fn bench_deloxide_rwlock_write_lock_unlock_with_component_stress(c: &mut Criterion) {
    use deloxide::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("deloxide_rwlock_write_lock_unlock_with_component_stress", |b| {
        b.iter(|| {
            let mut g = l.write();
            *g += 1;
        });
    });
}

fn init_deloxide() {
    use deloxide::Deloxide;
    use std::sync::Once;

    static INIT: Once = Once::new();
    INIT.call_once(|| {
        Deloxide::new().with_component_stress().start().unwrap();
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets =
        bench_deloxide_rwlock_read_lock_unlock_with_component_stress,
        bench_deloxide_rwlock_write_lock_unlock_with_component_stress,
);

fn main() {
    init_deloxide();
    benches();
}
