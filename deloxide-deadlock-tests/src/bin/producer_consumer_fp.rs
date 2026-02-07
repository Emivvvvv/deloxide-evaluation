// Producer-Consumer Pattern - False Positive Test
// Classic producer-consumer with queue (one-way data flow)

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let _deadlock_detected = init_detectors();
    let start = std::time::Instant::now();

    let queue = Arc::new(Mutex::new(Vec::<i32>::new()));
    let mut handles = vec![];

    // Spawn 4 producers
    for prod_id in 0..4 {
        let q = queue.clone();
        handles.push(thread::spawn(move || {
            for i in 0..100 {
                let mut guard = lock!(q);
                guard.push(prod_id * 1000 + i);
                drop(guard);
                thread::sleep(Duration::from_millis(5));
            }
        }));
    }

    // Spawn 4 consumers
    for _ in 0..4 {
        let q = queue.clone();
        handles.push(thread::spawn(move || {
            let mut consumed = 0;
            while consumed < 100 {
                let mut guard = lock!(q);
                if !guard.is_empty() {
                    guard.pop();
                    consumed += 1;
                }
                drop(guard);
                thread::sleep(Duration::from_millis(5));
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let filename = format!("fp_tests/producer_consumer_fp_{}.csv", feature_name());
    append_log_no_seed(&filename, false, false, elapsed);
}
