// Slow benchmark: Condvar ping-pong with random stress testing
// Tests the overhead of random stress mode with different configurations on condition variable coordination.
// Sample size: 10 (slow due to thread coordination overhead)

use criterion::{criterion_group, criterion_main, Criterion};
use deloxide::{Condvar, Deloxide, Mutex, StressConfig};
use std::sync::Arc;
use std::thread;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn run_ping_pong_test() {
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
}

// Default StressConfig
fn bench_stress_gentle(c: &mut Criterion) {
    Deloxide::new()
        .with_random_stress()
        .with_stress_config(StressConfig::gentle())
        .start()
        .unwrap();
    c.bench_function("condvar_ping_pong_stress_random_gentle", |b| {
        b.iter(|| run_ping_pong_test());
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = 
        bench_stress_gentle
);

criterion_main!(benches);
