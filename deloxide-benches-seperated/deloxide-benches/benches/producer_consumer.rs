// Medium benchmark: Producer-consumer pattern with multiple threads
// Tests lock contention in a realistic multi-threaded scenario.
// Sample size: 50 (medium complexity)
// Thread counts: [4, 16, 64] for all implementations

use criterion::{criterion_group, Criterion};
use std::sync::Arc;
use std::thread;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn bench_producer_consumer_std(c: &mut Criterion) {
    use std::sync::Mutex;
    let thread_counts = [4, 16, 64];
    for &threads in &thread_counts {
        c.bench_function(&format!("producer_consumer_std_{}x{}", threads, threads), |b| {
            b.iter(|| {
                let data = Arc::new(Mutex::new(Vec::new()));
                let mut producers = Vec::new();
                let mut consumers = Vec::new();
                for _ in 0..threads {
                    let prod = data.clone();
                    producers.push(thread::spawn(move || {
                        for i in 0..1000 {
                            prod.lock().unwrap().push(i);
                        }
                    }));
                }
                for _ in 0..threads {
                    let cons = data.clone();
                    consumers.push(thread::spawn(move || {
                        let mut received = 0;
                        while received < 1000 {
                            let mut guard = cons.lock().unwrap();
                            if !guard.is_empty() {
                                guard.pop();
                                received += 1;
                            }
                            drop(guard);
                        }
                    }));
                }
                for p in producers {
                    p.join().unwrap();
                }
                for c in consumers {
                    c.join().unwrap();
                }
            });
        });
    }
}

fn bench_producer_consumer_parking_lot(c: &mut Criterion) {
    use parking_lot::Mutex;
    let thread_counts = [4, 16, 64];
    for &threads in &thread_counts {
        c.bench_function(&format!("producer_consumer_parking_lot_{}x{}", threads, threads), |b| {
            b.iter(|| {
                let data = Arc::new(Mutex::new(Vec::new()));
                let mut producers = Vec::new();
                let mut consumers = Vec::new();
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
                for p in producers {
                    p.join().unwrap();
                }
                for c in consumers {
                    c.join().unwrap();
                }
            });
        });
    }
}
fn bench_producer_consumer_no_deadlocks(c: &mut Criterion) {
    use no_deadlocks::prelude::Mutex;
    let thread_counts = [4];
    for &threads in &thread_counts {
        c.bench_function(
            &format!(
                "producer_consumer_no_deadlocks_{}x{}",
                threads, threads
            ),
            |b| {
                b.iter(|| {
                    let data = Arc::new(Mutex::new(Vec::new()));
                    let mut producers = Vec::new();
                    let mut consumers = Vec::new();
                    for _ in 0..threads {
                        let prod = data.clone();
                        producers.push(thread::spawn(move || {
                            for i in 0..1000 {
                                prod.lock().unwrap().push(i);
                            }
                        }));
                    }
                    for _ in 0..threads {
                        let cons = data.clone();
                        consumers.push(thread::spawn(move || {
                            let mut received = 0;
                            while received < 1000 {
                                let mut guard = cons.lock().unwrap();
                                if !guard.is_empty() {
                                    guard.pop();
                                    received += 1;
                                }
                                drop(guard);
                            }
                        }));
                    }
                    for p in producers {
                        p.join().unwrap();
                    }
                    for c in consumers {
                        c.join().unwrap();
                    }
                });
            },
        );
    }
}

fn bench_producer_consumer_deloxide(c: &mut Criterion) {
    use deloxide::Mutex;
    let thread_counts = [4, 16, 64];
    for &threads in &thread_counts {
        c.bench_function(&format!("producer_consumer_deloxide_{}x{}", threads, threads), |b| {
            b.iter(|| {
                let data = Arc::new(Mutex::new(Vec::new()));
                let mut producers = Vec::new();
                let mut consumers = Vec::new();
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
                for p in producers {
                    p.join().unwrap();
                }
                for c in consumers {
                    c.join().unwrap();
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
        // bench_producer_consumer_std,
        bench_producer_consumer_parking_lot,
        // bench_producer_consumer_no_deadlocks,
        bench_producer_consumer_deloxide,
);

fn main() {
    init_deloxide();
    benches();
}
