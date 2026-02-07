// Medium benchmark: RwLock microburst with multiple implementations
// Mixes bursts of readers with a small number of writers to validate fairness
// and ordering while keeping iteration counts modest.
// Sample size: 30 (lightweight critical sections)

use criterion::{criterion_group, Criterion};
use std::sync::Arc;
use std::thread;

const CONFIGS: &[(usize, usize, usize, usize)] = &[
    // (readers, writers, reader_iterations, writer_iterations)
    (4, 1, 2_000, 200),
    (6, 2, 3_000, 300),
];

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(30)
}

// ============================================================================
// Std implementation
// ============================================================================

fn bench_rwlock_microburst_std(c: &mut Criterion) {
    for &(readers, writers, reader_iters, writer_iters) in CONFIGS {
        c.bench_function(
            &format!("rwlock_microburst_std_{}r{}w", readers, writers),
            |b| {
                b.iter(|| {
                    let lock = Arc::new(std::sync::RwLock::new(0usize));
                    let mut reader_handles = Vec::with_capacity(readers);
                    let mut writer_handles = Vec::with_capacity(writers);

                    for _ in 0..readers {
                        let l = lock.clone();
                        reader_handles.push(thread::spawn(move || {
                            let mut checksum = 0usize;
                            for _ in 0..reader_iters {
                                let guard = l.read().unwrap();
                                checksum ^= *guard;
                            }
                            checksum
                        }));
                    }

                    for _ in 0..writers {
                        let l = lock.clone();
                        writer_handles.push(thread::spawn(move || {
                            for _ in 0..writer_iters {
                                let mut guard = l.write().unwrap();
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
                });
            },
        );
    }
}

// ============================================================================
// Parking Lot implementation
// ============================================================================

fn bench_rwlock_microburst_parking_lot(c: &mut Criterion) {
    for &(readers, writers, reader_iters, writer_iters) in CONFIGS {
        c.bench_function(
            &format!("rwlock_microburst_parking_lot_{}r{}w", readers, writers),
            |b| {
                b.iter(|| {
                    let lock = Arc::new(parking_lot::RwLock::new(0usize));
                    let mut reader_handles = Vec::with_capacity(readers);
                    let mut writer_handles = Vec::with_capacity(writers);

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
                });
            },
        );
    }
}

// ============================================================================
// no_deadlocks implementation
// ============================================================================

fn bench_rwlock_microburst_no_deadlocks(c: &mut Criterion) {
    for &(readers, writers, reader_iters, writer_iters) in CONFIGS {
        c.bench_function(
            &format!("rwlock_microburst_no_deadlocks_{}r{}w", readers, writers),
            |b| {
                b.iter(|| {
                    let lock = Arc::new(no_deadlocks::prelude::RwLock::new(0usize));
                    let mut reader_handles = Vec::with_capacity(readers);
                    let mut writer_handles = Vec::with_capacity(writers);

                    for _ in 0..readers {
                        let l = lock.clone();
                        reader_handles.push(thread::spawn(move || {
                            let mut checksum = 0usize;
                            for _ in 0..reader_iters {
                                let guard = l.read().unwrap();
                                checksum ^= *guard;
                            }
                            checksum
                        }));
                    }

                    for _ in 0..writers {
                        let l = lock.clone();
                        writer_handles.push(thread::spawn(move || {
                            for _ in 0..writer_iters {
                                let mut guard = l.write().unwrap();
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
                });
            },
        );
    }
}

// ============================================================================
// Deloxide implementation
// ============================================================================

fn bench_rwlock_microburst_deloxide(c: &mut Criterion) {
    for &(readers, writers, reader_iters, writer_iters) in CONFIGS {
        c.bench_function(
            &format!("rwlock_microburst_deloxide_{}r{}w", readers, writers),
            |b| {
                b.iter(|| {
                    let lock = Arc::new(deloxide::RwLock::new(0usize));
                    let mut reader_handles = Vec::with_capacity(readers);
                    let mut writer_handles = Vec::with_capacity(writers);

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
                });
            },
        );
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
        // bench_rwlock_microburst_std,
        // bench_rwlock_microburst_parking_lot,
        // bench_rwlock_microburst_no_deadlocks,
        bench_rwlock_microburst_deloxide,
);

fn main() {
    init_deloxide();
    benches();
}
