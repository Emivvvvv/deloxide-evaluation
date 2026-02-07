// Medium benchmark: RwLock scalability with lock order checking
// Tests the overhead of lock order checking on concurrent read workload.
// Sample size: 50 (medium complexity)
// Thread counts: [4, 16, 64]
// Pattern: Readers run concurrently, then writers (deadlock-proof)

use criterion::{criterion_group, Criterion};
use std::sync::Arc;
use std::thread;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn bench_rwlock_concurrent_reads_deloxide_with_lock_order_checking(c: &mut Criterion) {
    use deloxide::RwLock;
    let thread_counts = [4, 16, 64];
    for &threads in &thread_counts {
        c.bench_function(&format!("rwlock_concurrent_reads_deloxide_with_lock_order_checking_{}t", threads), |b| {
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
        Deloxide::new().with_lock_order_checking().start().unwrap();
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = bench_rwlock_concurrent_reads_deloxide_with_lock_order_checking,
);

fn main() {
    init_deloxide();
    benches();
}
