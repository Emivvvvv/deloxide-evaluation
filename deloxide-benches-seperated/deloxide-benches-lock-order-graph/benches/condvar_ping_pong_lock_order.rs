// Slow benchmark: Condvar ping-pong with lock order checking
// Tests the overhead of lock order checking on condition variable coordination.
// Sample size: 10 (slow due to thread coordination overhead)

use criterion::{criterion_group, Criterion};
use std::sync::Arc;
use std::thread;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn bench_condvar_ping_pong_deloxide_with_lock_order_checking(c: &mut Criterion) {
    use deloxide::{Condvar, Mutex};
    c.bench_function("condvar_ping_pong_deloxide_with_lock_order_checking", |b| {
        b.iter(|| {
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            let pair2 = pair.clone();
            let t = thread::spawn(move || {
                let (lock, cv) = &*pair2;
                let mut ready = lock.lock();
                *ready = true;
                cv.notify_one();
            });
            let (lock, cv) = &*pair;
            let mut ready = lock.lock();
            while !*ready {
                cv.wait(&mut ready);
            }
            t.join().unwrap();
        });
    });
}

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
    targets = bench_condvar_ping_pong_deloxide_with_lock_order_checking,
);

fn main() {
    init_deloxide();
    benches();
}
