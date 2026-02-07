// Medium benchmark: RwLock scalability with concurrent readers
// Tests RwLock's key feature: multiple concurrent readers.
// Sample size: 50 (medium complexity)
// Thread counts: [4, 16, 64] for all implementations
// Pattern: Readers run concurrently, then writers run sequentially (deadlock-proof)

use criterion::{criterion_group, Criterion};
use std::sync::Arc;
use std::thread;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

// Deadlock-proof pattern: All readers run concurrently, then all writers
fn bench_rwlock_concurrent_reads_std(c: &mut Criterion) {
    use std::sync::RwLock;
    let thread_counts = [4, 16, 64];
    for &threads in &thread_counts {
        c.bench_function(&format!("rwlock_concurrent_reads_std_{}t", threads), |b| {
            b.iter(|| {
                let l = Arc::new(RwLock::new(0usize));
                
                // Phase 1: All threads read concurrently (tests concurrent read performance)
                let mut handles = Vec::new();
                for _ in 0..threads {
                    let l = l.clone();
                    handles.push(thread::spawn(move || {
                        for _ in 0..1000 {
                            let g = l.read().unwrap();
                            let _ = *g;
                            drop(g);
                        }
                    }));
                }
                for h in handles {
                    h.join().unwrap();
                }
                
                // Phase 2: Single writer updates (no deadlock possible)
                {
                    let mut g = l.write().unwrap();
                    *g += 1;
                }
            });
        });
    }
}

fn bench_rwlock_concurrent_reads_parking_lot(c: &mut Criterion) {
    use parking_lot::RwLock;
    let thread_counts = [4, 16, 64];
    for &threads in &thread_counts {
        c.bench_function(&format!("rwlock_concurrent_reads_parking_lot_{}t", threads), |b| {
            b.iter(|| {
                let l = Arc::new(RwLock::new(0usize));
                
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
            });
        });
    }
}
fn bench_rwlock_concurrent_reads_no_deadlocks(c: &mut Criterion) {
    use no_deadlocks::prelude::RwLock;
    let thread_counts = [4];
    for &threads in &thread_counts {
        c.bench_function(
            &format!(
                "rwlock_concurrent_reads_no_deadlocks_{}t",
                threads
            ),
            |b| {
                b.iter(|| {
                    let l = Arc::new(RwLock::new(0usize));

                    let mut handles = Vec::new();
                    for _ in 0..threads {
                        let l = l.clone();
                        handles.push(thread::spawn(move || {
                            for _ in 0..1000 {
                                let g = l.read().unwrap();
                                let _ = *g;
                                drop(g);
                            }
                        }));
                    }
                    for h in handles {
                        h.join().unwrap();
                    }

                    {
                        let mut g = l.write().unwrap();
                        *g += 1;
                    }
                });
            },
        );
    }
}

fn bench_rwlock_concurrent_reads_deloxide(c: &mut Criterion) {
    use deloxide::RwLock;
    let thread_counts = [4, 16, 64];
    for &threads in &thread_counts {
        c.bench_function(&format!("rwlock_concurrent_reads_deloxide_{}t", threads), |b| {
            b.iter(|| {
                let l = Arc::new(RwLock::new(0usize));
                
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
            });
        });
    }
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
        bench_rwlock_concurrent_reads_std,
        bench_rwlock_concurrent_reads_parking_lot,
        bench_rwlock_concurrent_reads_no_deadlocks,
        bench_rwlock_concurrent_reads_deloxide,
);

fn main() {
    init_deloxide();
    benches();
}
