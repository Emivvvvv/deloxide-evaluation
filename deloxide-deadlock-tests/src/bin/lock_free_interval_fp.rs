// Lock-Free Interval - False Positive Test
// Locks in conflicting orders but never held simultaneously

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let _deadlock_detected = init_detectors();
    let start = std::time::Instant::now();

    let a = Arc::new(Mutex::new(()));
    let b = Arc::new(Mutex::new(()));

    let a1 = a.clone();
    let b1 = b.clone();
    let t1 = thread::spawn(move || {
        // Thread 1: Lock A, unlock, delay, lock B
        for _ in 0..10 {
            let _lock_a = lock!(a1);
            thread::sleep(Duration::from_millis(10));
            drop(_lock_a);
            
            thread::sleep(Duration::from_millis(50)); // Lock-free interval
            
            let _lock_b = lock!(b1);
            thread::sleep(Duration::from_millis(10));
            drop(_lock_b);
            
            thread::sleep(Duration::from_millis(20));
        }
    });

    let a2 = a.clone();
    let b2 = b.clone();
    let t2 = thread::spawn(move || {
        // Thread 2: Lock B, unlock, delay, lock A
        for _ in 0..10 {
            let _lock_b = lock!(b2);
            thread::sleep(Duration::from_millis(10));
            drop(_lock_b);
            
            thread::sleep(Duration::from_millis(50)); // Lock-free interval
            
            let _lock_a = lock!(a2);
            thread::sleep(Duration::from_millis(10));
            drop(_lock_a);
            
            thread::sleep(Duration::from_millis(20));
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let elapsed = start.elapsed().as_secs_f64();
    let filename = format!("fp_tests/lock_free_interval_fp_{}.csv", feature_name());
    append_log_no_seed(&filename, false, false, elapsed);
}
