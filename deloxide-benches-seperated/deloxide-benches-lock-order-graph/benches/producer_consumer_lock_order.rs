// Medium benchmark: Producer-consumer with lock order checking
// Tests the overhead of lock order checking on realistic multi-threaded workload.
// Sample size: 50 (medium complexity)
// Thread counts: [4, 16, 64]

use criterion::{criterion_group, Criterion};
use std::sync::Arc;
use std::thread;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn bench_producer_consumer_deloxide_with_lock_order_checking(c: &mut Criterion) {
    use deloxide::Mutex;
    let thread_counts = [16, 64];
    for &threads in &thread_counts {
        c.bench_function(&format!("producer_consumer_deloxide_with_lock_order_checking_{}x{}", threads, threads), |b| {
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
                for p in producers { p.join().unwrap(); }
                for c in consumers { c.join().unwrap(); }
            });
        });
    }
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
    targets = bench_producer_consumer_deloxide_with_lock_order_checking,
);

fn main() {
    init_deloxide();
    benches();
}
