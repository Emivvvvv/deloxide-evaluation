// Conditional Locking - False Positive Test
// Coordinator ensures only one locking pattern at a time

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
    let coordinator = Arc::new(Mutex::new(true)); // true = forward, false = reverse

    let mut handles = vec![];

    // Threads take turns using A→B or B→A based on coordinator
    for tid in 0..4 {
        let a = a.clone();
        let b = b.clone();
        let coord = coordinator.clone();

        handles.push(thread::spawn(move || {
            for _ in 0..20 {
                let mut use_forward = lock!(coord);

                if tid % 2 == 0 {
                    // Even threads: wait for forward pattern
                    while !*use_forward {
                        drop(use_forward);
                        thread::sleep(Duration::from_millis(50));
                        use_forward = lock!(coord);
                    }
                    // Execute forward: A → B
                    let _la = lock!(a);
                    thread::sleep(Duration::from_millis(100));
                    let _lb = lock!(b);
                    thread::sleep(Duration::from_millis(100));
                    *use_forward = false;
                } else {
                    // Odd threads: wait for reverse pattern
                    while *use_forward {
                        drop(use_forward);
                        thread::sleep(Duration::from_millis(50));
                        use_forward = lock!(coord);
                    }
                    // Execute reverse: B → A
                    let _lb = lock!(b);
                    thread::sleep(Duration::from_millis(100));
                    let _la = lock!(a);
                    thread::sleep(Duration::from_millis(100));
                    *use_forward = true;
                }
                thread::sleep(Duration::from_millis(100));
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let filename = format!("fp_tests/conditional_locking_fp_{}.csv", feature_name());
    append_log_no_seed(&filename, false, false, elapsed);
}
