// Slow benchmark: Condition variable ping-pong coordination
// Tests condvar overhead in simple two-thread coordination.
// Sample size: 10 (slow due to thread coordination overhead)

use criterion::{criterion_group, Criterion};
use std::sync::Arc;
use std::thread;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn bench_condvar_ping_pong_std(c: &mut Criterion) {
    use std::sync::{Condvar, Mutex};
    c.bench_function("condvar_ping_pong_std", |b| {
        b.iter(|| {
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            let pair2 = pair.clone();
            let t = thread::spawn(move || {
                let (lock, cv) = &*pair2;
                let mut ready = lock.lock().unwrap();
                *ready = true;
                cv.notify_one();
            });
            let (lock, cv) = &*pair;
            let mut ready = lock.lock().unwrap();
            while !*ready {
                ready = cv.wait(ready).unwrap();
            }
            t.join().unwrap();
        });
    });
}

fn bench_condvar_ping_pong_parking_lot(c: &mut Criterion) {
    use parking_lot::{Condvar, Mutex};
    c.bench_function("condvar_ping_pong_parking_lot", |b| {
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
fn bench_condvar_ping_pong_no_deadlocks(c: &mut Criterion) {
    use no_deadlocks::prelude::{Condvar, Mutex};
    c.bench_function("condvar_ping_pong_no_deadlocks", |b| {
        b.iter(|| {
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            let pair2 = pair.clone();
            let t = thread::spawn(move || {
                let (lock, cv) = &*pair2;
                let mut ready = lock.lock().unwrap();
                *ready = true;
                cv.notify_one();
            });
            let (lock, cv) = &*pair;
            let mut ready = lock.lock().unwrap();
            while !*ready {
                ready = cv.wait(ready).unwrap();
            }
            t.join().unwrap();
        });
    });
}

fn bench_condvar_ping_pong_deloxide(c: &mut Criterion) {
    use deloxide::{Condvar, Mutex};
    c.bench_function("condvar_ping_pong_deloxide", |b| {
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
        Deloxide::new().start().unwrap();
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets =
        // bench_condvar_ping_pong_std,
        // bench_condvar_ping_pong_parking_lot,
        // bench_condvar_ping_pong_no_deadlocks,
        bench_condvar_ping_pong_deloxide,
);

fn main() {
    init_deloxide();
    benches();
}
