// Medium benchmark: Bounded MPMC queue with lock order checking
// Two producers and two consumers move bursts of messages through a bounded
// queue guarded by a Mutex + Condvar pair. Exercises coordination and lock
// ordering without large thread counts.
// Sample size: 20 (moderate cost)

use criterion::{criterion_group, Criterion};
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;

use deloxide::{Condvar, Mutex};

const BURST_SIZES: [usize; 2] = [4_096, 16_384];
const QUEUE_CAPACITY: usize = 128;
const PRODUCERS: usize = 2;
const CONSUMERS: usize = 2;

struct BoundedQueue {
    state: Mutex<QueueState>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
}

struct QueueState {
    buffer: VecDeque<u32>,
}

impl BoundedQueue {
    fn new(capacity: usize) -> Self {
        Self {
            state: Mutex::new(QueueState {
                buffer: VecDeque::with_capacity(capacity),
            }),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            capacity,
        }
    }

    fn push(&self, value: u32) {
        let mut state = self.state.lock();
        while state.buffer.len() == self.capacity {
            self.not_full.wait(&mut state);
        }
        state.buffer.push_back(value);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> u32 {
        let mut state = self.state.lock();
        loop {
            if let Some(value) = state.buffer.pop_front() {
                self.not_full.notify_one();
                return value;
            }
            self.not_empty.wait(&mut state);
        }
    }
}

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(20)
}

fn bench_bounded_mpmc_deloxide_with_lock_order_checking(c: &mut Criterion) {
    for &burst in &BURST_SIZES {
        c.bench_function(
            &format!(
                "bounded_mpmc_deloxide_with_lock_order_checking_{}msgs",
                burst
            ),
            |b| {
                b.iter(|| {
                    let queue = Arc::new(BoundedQueue::new(QUEUE_CAPACITY));
                    let mut handles = Vec::new();

                    let msgs_per_producer = burst / PRODUCERS;
                    for producer in 0..PRODUCERS {
                        let q = queue.clone();
                        handles.push(thread::spawn(move || {
                            let start = producer * msgs_per_producer;
                            for offset in 0..msgs_per_producer {
                                q.push((start + offset) as u32);
                            }
                        }));
                    }

                    let msgs_per_consumer = burst / CONSUMERS;
                    for _ in 0..CONSUMERS {
                        let q = queue.clone();
                        handles.push(thread::spawn(move || {
                            for _ in 0..msgs_per_consumer {
                                let _ = q.pop();
                            }
                        }));
                    }

                    for handle in handles {
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
        Deloxide::new()
            .with_lock_order_checking()
            .start()
            .unwrap();
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = bench_bounded_mpmc_deloxide_with_lock_order_checking,
);

fn main() {
    init_deloxide();
    benches();
}

