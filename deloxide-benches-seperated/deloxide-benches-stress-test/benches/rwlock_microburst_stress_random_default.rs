// Medium benchmark: RwLock microburst with random stress testing
// Tests the overhead of random stress mode with different configurations on mixed read/write workloads.
// Sample size: 10 (lightweight but includes coordination)

use criterion::{criterion_group, criterion_main, Criterion};
use deloxide::{Deloxide, RwLock};
use std::sync::Arc;
use std::thread;

// Configurations: (readers, writers, reader_iters, writer_iters)
const CONFIGS: &[(usize, usize, usize, usize)] = &[
    (4, 1, 2_000, 200),
    (6, 2, 3_000, 300),
];

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn run_microburst(readers: usize, writers: usize, reader_iters: usize, writer_iters: usize) {
    let lock = Arc::new(RwLock::new(0usize));
    let mut reader_handles = Vec::with_capacity(readers);
    let mut writer_handles = Vec::with_capacity(writers);

    // Readers
    for _ in 0..readers {
        let l = lock.clone();
        reader_handles.push(thread::spawn(move || {
            let mut checksum = 0usize;
            for _ in 0..reader_iters {
                let guard = l.read();
                checksum ^= *guard;
            }
            checksum
        }));
    }

    // Writers
    for _ in 0..writers {
        let l = lock.clone();
        writer_handles.push(thread::spawn(move || {
            for _ in 0..writer_iters {
                let mut guard = l.write();
                *guard = guard.wrapping_add(1);
                drop(guard);
                thread::yield_now();
            }
        }));
    }

    for handle in reader_handles {
        handle.join().unwrap();
    }
    for handle in writer_handles {
        handle.join().unwrap();
    }
}

// Default 
fn bench_stress_default(c: &mut Criterion) {
    Deloxide::new().with_random_stress().start().unwrap();
    for &(readers, writers, reader_iters, writer_iters) in CONFIGS {
        c.bench_function(
            &format!(
                "rwlock_microburst_stress_random_default_{}r{}w",
                readers, writers
            ),
            |b| {
                b.iter(|| run_microburst(readers, writers, reader_iters, writer_iters));
            },
        );
    }
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = 
        bench_stress_default
);

criterion_main!(benches);
