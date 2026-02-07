// Micro-benchmark: Single lock/unlock operation overhead
// Tests the baseline cost of acquiring and releasing a mutex with no contention.
// Sample size: 100 (fast micro-benchmark)

use criterion::{criterion_group, Criterion};
use std::sync::Arc;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(100)
}

fn bench_std_mutex_lock_unlock(c: &mut Criterion) {
    use std::sync::Mutex;

    let m = Arc::new(Mutex::new(0));
    c.bench_function("std_mutex_lock_unlock", |b| {
        b.iter(|| {
            let _lock = m.lock().unwrap();
        });
    });
}

fn bench_parking_lot_mutex_lock_unlock(c: &mut Criterion) {
    use parking_lot::Mutex;
    let m = Arc::new(Mutex::new(0));
    c.bench_function("parking_lot_mutex_lock_unlock", |b| {
        b.iter(|| {
            let _lock = m.lock();
        });
    });
}

fn bench_no_deadlocks_mutex_lock_unlock(c: &mut Criterion) {
    use no_deadlocks::prelude::Mutex;
    let m = Arc::new(Mutex::new(0));
    c.bench_function("no_deadlocks_mutex_lock_unlock", |b| {
        b.iter(|| {
            let _lock = m.lock();
        });
    });
}

fn bench_deloxide_mutex_lock_unlock(c: &mut Criterion) {
    use deloxide::Mutex;

    let m = Arc::new(Mutex::new(0));
    c.bench_function("deloxide_mutex_lock_unlock", |b| {
        b.iter(|| {
            let _lock = m.lock();
        });
    });
}

fn init_deloxide() {
    use deloxide::Deloxide;
    use std::sync::Once;

    static INIT: Once = Once::new();
    INIT.call_once(|| {
        Deloxide::new().start().unwrap();
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets =
        bench_std_mutex_lock_unlock,
        bench_parking_lot_mutex_lock_unlock,
        bench_no_deadlocks_mutex_lock_unlock,
        bench_deloxide_mutex_lock_unlock,
);

fn main() {
    init_deloxide();
    benches();
}
