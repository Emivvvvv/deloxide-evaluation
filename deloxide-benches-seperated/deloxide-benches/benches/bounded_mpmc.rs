// Medium benchmark: Bounded MPMC queue with multiple implementations
// Two producers and two consumers move bursts of messages through a bounded
// queue guarded by a Mutex + Condvar pair. Exercises coordination overhead.
// Sample size: 20 (moderate cost)

use criterion::{criterion_group, Criterion};
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;

const BURST_SIZES: [usize; 2] = [4_096, 16_384];
const QUEUE_CAPACITY: usize = 128;
const PRODUCERS: usize = 2;
const CONSUMERS: usize = 2;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(20)
}

// ============================================================================
// Std implementation
// ============================================================================

struct BoundedQueueStd {
    state: std::sync::Mutex<QueueState>,
    not_empty: std::sync::Condvar,
    not_full: std::sync::Condvar,
    capacity: usize,
}

struct QueueState {
    buffer: VecDeque<u32>,
}

impl BoundedQueueStd {
    fn new(capacity: usize) -> Self {
        Self {
            state: std::sync::Mutex::new(QueueState {
                buffer: VecDeque::with_capacity(capacity),
            }),
            not_empty: std::sync::Condvar::new(),
            not_full: std::sync::Condvar::new(),
            capacity,
        }
    }

    fn push(&self, value: u32) {
        let mut state = self.state.lock().unwrap();
        while state.buffer.len() == self.capacity {
            state = self.not_full.wait(state).unwrap();
        }
        state.buffer.push_back(value);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> u32 {
        let mut state = self.state.lock().unwrap();
        loop {
            if let Some(value) = state.buffer.pop_front() {
                self.not_full.notify_one();
                return value;
            }
            state = self.not_empty.wait(state).unwrap();
        }
    }
}

fn bench_bounded_mpmc_std(c: &mut Criterion) {
    for &burst in &BURST_SIZES {
        c.bench_function(
            &format!("bounded_mpmc_std_{}msgs", burst),
            |b| {
                b.iter(|| {
                    let queue = Arc::new(BoundedQueueStd::new(QUEUE_CAPACITY));
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

// ============================================================================
// Parking Lot implementation
// ============================================================================

struct BoundedQueueParkingLot {
    state: parking_lot::Mutex<QueueState>,
    not_empty: parking_lot::Condvar,
    not_full: parking_lot::Condvar,
    capacity: usize,
}

impl BoundedQueueParkingLot {
    fn new(capacity: usize) -> Self {
        Self {
            state: parking_lot::Mutex::new(QueueState {
                buffer: VecDeque::with_capacity(capacity),
            }),
            not_empty: parking_lot::Condvar::new(),
            not_full: parking_lot::Condvar::new(),
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

fn bench_bounded_mpmc_parking_lot(c: &mut Criterion) {
    for &burst in &BURST_SIZES {
        c.bench_function(
            &format!("bounded_mpmc_parking_lot_{}msgs", burst),
            |b| {
                b.iter(|| {
                    let queue = Arc::new(BoundedQueueParkingLot::new(QUEUE_CAPACITY));
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

// ============================================================================
// Deloxide implementation
// ============================================================================

struct BoundedQueueDeloxide {
    state: deloxide::Mutex<QueueState>,
    not_empty: deloxide::Condvar,
    not_full: deloxide::Condvar,
    capacity: usize,
}

impl BoundedQueueDeloxide {
    fn new(capacity: usize) -> Self {
        Self {
            state: deloxide::Mutex::new(QueueState {
                buffer: VecDeque::with_capacity(capacity),
            }),
            not_empty: deloxide::Condvar::new(),
            not_full: deloxide::Condvar::new(),
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

fn bench_bounded_mpmc_deloxide(c: &mut Criterion) {
    for &burst in &BURST_SIZES {
        c.bench_function(
            &format!("bounded_mpmc_deloxide_{}msgs", burst),
            |b| {
                b.iter(|| {
                    let queue = Arc::new(BoundedQueueDeloxide::new(QUEUE_CAPACITY));
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
        Deloxide::new().start().unwrap();
    });
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets =
        // bench_bounded_mpmc_std,
        // bench_bounded_mpmc_parking_lot,
        bench_bounded_mpmc_deloxide,
);

fn main() {
    init_deloxide();
    benches();
}
