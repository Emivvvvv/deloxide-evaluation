// Thread-Local Hierarchy - False Positive Test
// Different threads have different lock hierarchies (no conflict)

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let _deadlock_detected = init_detectors();
    let start = std::time::Instant::now();

    // Create two sets of locks
    // Thread group 1 only uses set1 (A, B, C)
    // Thread group 2 only uses set2 (X, Y, Z)
    let lock_a = Arc::new(Mutex::new(()));
    let lock_b = Arc::new(Mutex::new(()));
    let lock_c = Arc::new(Mutex::new(()));

    let lock_x = Arc::new(Mutex::new(()));
    let lock_y = Arc::new(Mutex::new(()));
    let lock_z = Arc::new(Mutex::new(()));

    let mut handles = vec![];

    // Thread group 1: Always acquires A → B → C
    for _ in 0..3 {
        let a = lock_a.clone();
        let b = lock_b.clone();
        let c = lock_c.clone();

        handles.push(thread::spawn(move || {
            for _ in 0..50 {
                let _la = lock!(a);
                thread::sleep(Duration::from_millis(50));
                let _lb = lock!(b);
                thread::sleep(Duration::from_millis(50));
                let _lc = lock!(c);
                thread::sleep(Duration::from_millis(50));
            }
        }));
    }

    // Thread group 2: Always acquires X → Y → Z (different locks)
    for _ in 0..3 {
        let x = lock_x.clone();
        let y = lock_y.clone();
        let z = lock_z.clone();

        handles.push(thread::spawn(move || {
            for _ in 0..50 {
                let _lx = lock!(x);
                thread::sleep(Duration::from_millis(50));
                let _ly = lock!(y);
                thread::sleep(Duration::from_millis(50));
                let _lz = lock!(z);
                thread::sleep(Duration::from_millis(50));
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let filename = format!("fp_tests/thread_local_hierarchy_fp_{}.csv", feature_name());
    append_log_no_seed(&filename, false, false, elapsed);
}
