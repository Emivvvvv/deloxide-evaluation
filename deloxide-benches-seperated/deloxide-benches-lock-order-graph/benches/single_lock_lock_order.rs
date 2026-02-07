// Micro-benchmark: Single lock/unlock with lock order checking
// Tests the overhead of lock order checking on basic lock operations.
// Sample size: 100 (fast micro-benchmark)

use criterion::{criterion_group, Criterion};
use std::sync::Arc;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(100)
}

fn bench_deloxide_mutex_lock_unlock_with_lock_order_checking(c: &mut Criterion) {
    use deloxide::Mutex;

    let m = Arc::new(Mutex::new(0));
    c.bench_function("deloxide_mutex_lock_unlock_with_lock_order_checking", |b| {
        b.iter(|| {
            let _lock = m.lock();
        });
    });
}

// Initialize deloxide detector once with lock order checking
fn init_deloxide() {
    use deloxide::Deloxide;
    use std::sync::Once;

    static INIT: Once = Once::new();
    INIT.call_once(|| {
        Deloxide::new().with_lock_order_checking().start().unwrap();
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = bench_deloxide_mutex_lock_unlock_with_lock_order_checking,
);

fn main() {
    init_deloxide();
    benches();
}
