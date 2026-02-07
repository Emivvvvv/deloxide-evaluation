// Medium benchmark: Bounded MPMC queue with random stress testing
// Tests the overhead of random stress mode with different configurations on condvar coordination.
// Sample size: 10 (moderate cost due to coordination)

use criterion::{criterion_group, criterion_main, Criterion};
use deloxide::{Condvar, Deloxide, Mutex, };
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;

const BURST_SIZES: [usize; 2] = [4_096, 16_384];
const QUEUE_CAPACITY: usize = 128;
const PRODUCERS: usize = 2;
const CONSUMERS: usize = 2;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

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

fn run_bounded_mpmc_test(burst: usize) {
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
}

// Default 
fn bench_stress_default(c: &mut Criterion) {
    Deloxide::new().with_random_stress().start().unwrap();
    for &burst in &BURST_SIZES {
        c.bench_function(&format!("bounded_mpmc_stress_random_default_{}msgs", burst), |b| {
            b.iter(|| run_bounded_mpmc_test(burst));
        });
    }
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = 
        bench_stress_default
);

criterion_main!(benches);
