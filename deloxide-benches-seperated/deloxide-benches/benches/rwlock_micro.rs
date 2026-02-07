// Micro-benchmark: RwLock read/write operation overhead
// Tests the baseline cost of read and write lock operations with no contention.
// Sample size: 100 (fast micro-benchmark)

use criterion::{criterion_group, Criterion};
use std::sync::Arc;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(100)
}

fn bench_std_rwlock_read_lock_unlock(c: &mut Criterion) {
    use std::sync::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("std_rwlock_read_lock_unlock", |b| {
        b.iter(|| {
            let _g = l.read().unwrap();
        });
    });
}

fn bench_std_rwlock_write_lock_unlock(c: &mut Criterion) {
    use std::sync::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("std_rwlock_write_lock_unlock", |b| {
        b.iter(|| {
            let mut g = l.write().unwrap();
            *g += 1;
        });
    });
}

fn bench_parking_lot_rwlock_read_lock_unlock(c: &mut Criterion) {
    use parking_lot::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("parking_lot_rwlock_read_lock_unlock", |b| {
        b.iter(|| {
            let _g = l.read();
        });
    });
}

fn bench_parking_lot_rwlock_write_lock_unlock(c: &mut Criterion) {
    use parking_lot::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("parking_lot_rwlock_write_lock_unlock", |b| {
        b.iter(|| {
            let mut g = l.write();
            *g += 1;
        });
    });
}

fn bench_no_deadlocks_rwlock_read_lock_unlock(c: &mut Criterion) {
    use no_deadlocks::prelude::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("no_deadlocks_rwlock_read_lock_unlock", |b| {
        b.iter(|| {
            let _g = l.read().unwrap();
        });
    });
}

fn bench_no_deadlocks_rwlock_write_lock_unlock(c: &mut Criterion) {
    use no_deadlocks::prelude::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("no_deadlocks_rwlock_write_lock_unlock", |b| {
        b.iter(|| {
            let mut g = l.write().unwrap();
            *g += 1;
        });
    });
}

fn bench_deloxide_rwlock_read_lock_unlock(c: &mut Criterion) {
    use deloxide::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("deloxide_rwlock_read_lock_unlock", |b| {
        b.iter(|| {
            let _g = l.read();
        });
    });
}

fn bench_deloxide_rwlock_write_lock_unlock(c: &mut Criterion) {
    use deloxide::RwLock;
    let l = Arc::new(RwLock::new(0usize));
    c.bench_function("deloxide_rwlock_write_lock_unlock", |b| {
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
        Deloxide::new().start().unwrap();
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets =
        bench_std_rwlock_read_lock_unlock,
        bench_std_rwlock_write_lock_unlock,
        bench_parking_lot_rwlock_read_lock_unlock,
        bench_parking_lot_rwlock_write_lock_unlock,
        bench_no_deadlocks_rwlock_read_lock_unlock,
        bench_no_deadlocks_rwlock_write_lock_unlock,
        bench_deloxide_rwlock_read_lock_unlock,
        bench_deloxide_rwlock_write_lock_unlock,
);

fn main() {
    init_deloxide();
    benches();
}
