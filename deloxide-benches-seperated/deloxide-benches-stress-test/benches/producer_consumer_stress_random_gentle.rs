// Medium benchmark: Producer-consumer with random stress testing
// Tests the overhead of random stress mode with different configurations.
// Sample size: 10 (medium complexity)
// Thread counts: [4]

use criterion::{criterion_group, criterion_main, Criterion};
use deloxide::{Deloxide, Mutex, StressConfig};
use std::sync::Arc;
use std::thread;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn run_producer_consumer_test(threads: usize) {
    let data = Arc::new(Mutex::new(Vec::new()));
    let mut producers = Vec::new();
    let mut consumers = Vec::new();
    // let threads = 4;
    
    for _ in 0..threads {
        let prod = data.clone();
        producers.push(thread::spawn(move || {
            for i in 0..1000 {
                prod.lock().push(i);
            }
        }));
    }
    
    for _ in 0..threads {
        let cons = data.clone();
        consumers.push(thread::spawn(move || {
            let mut received = 0;
            while received < 1000 {
                let mut guard = cons.lock();
                if !guard.is_empty() {
                    guard.pop();
                    received += 1;
                }
                drop(guard);
            }
        }));
    }
    
    for p in producers { p.join().unwrap(); }
    for c in consumers { c.join().unwrap(); }
}

// Default StressConfig
fn bench_stress_gentle(c: &mut Criterion) {
    Deloxide::new()
        .with_random_stress()
        .with_stress_config(StressConfig::gentle())
        .start()
        .unwrap();
    let thread_counts = [4, 16, 64];
    for &threads in &thread_counts {
        c.bench_function(&format!("producer_consumer_stress_random_gentle_{}x{}", threads, threads), |b| {
            b.iter(|| run_producer_consumer_test(threads));
        });
    }
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = 
        bench_stress_gentle
);

criterion_main!(benches);
