// Medium benchmark: RwLock scalability with random stress testing
// Tests the overhead of random stress mode with different configurations on concurrent read workload.
// Sample size: 10 (medium complexity)
// Thread counts: [4]
// Pattern: Readers run concurrently, then writers (deadlock-proof)

use criterion::{criterion_group, criterion_main, Criterion};
use deloxide::{Deloxide, RwLock, StressConfig};
use std::sync::Arc;
use std::thread;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn run_concurrent_reads_test(threads: usize) {
    let l = Arc::new(RwLock::new(0usize));
    // let threads = 4;
    
    let mut handles = Vec::new();
    for _ in 0..threads {
        let l = l.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let g = l.read();
                let _ = *g;
                drop(g);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    
    {
        let mut g = l.write();
        *g += 1;
    }
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
        c.bench_function(&format!("rwlock_contention_stress_random_gentle_{}t", threads), |b| {
            b.iter(|| run_concurrent_reads_test(threads));
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
