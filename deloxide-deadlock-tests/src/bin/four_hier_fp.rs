// Four Hierarchical Locks - False Positive Test
// Hierarchical locking pattern that should NOT cause deadlock

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let _deadlock_detected = init_detectors();
    let start = std::time::Instant::now();

    // Create four resources with hierarchical ordering
    let resource_a = Arc::new(Mutex::new("Resource A".to_string()));
    let resource_b = Arc::new(Mutex::new("Resource B".to_string()));
    let resource_c = Arc::new(Mutex::new("Resource C".to_string()));
    let resource_d = Arc::new(Mutex::new("Resource D".to_string()));

    let mut handles = vec![];

    // Thread 1: A -> B -> C -> D (ascending order)
    {
        let (r1, r2, r3, r4) = (resource_a.clone(), resource_b.clone(), resource_c.clone(), resource_d.clone());
        handles.push(thread::spawn(move || {
            let _lock_a = lock!(r1);
            thread::sleep(Duration::from_millis(50));
            let _lock_b = lock!(r2);
            thread::sleep(Duration::from_millis(50));
            let _lock_c = lock!(r3);
            thread::sleep(Duration::from_millis(50));
            let _lock_d = lock!(r4);
            thread::sleep(Duration::from_millis(100));
        }));
    }

    // Thread 2: A -> C -> D (skip B, still ascending)
    {
        let (r1, r3, r4) = (resource_a.clone(), resource_c.clone(), resource_d.clone());
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(25));
            let _lock_a = lock!(r1);
            thread::sleep(Duration::from_millis(50));
            let _lock_c = lock!(r3);
            thread::sleep(Duration::from_millis(50));
            let _lock_d = lock!(r4);
            thread::sleep(Duration::from_millis(100));
        }));
    }

    // Thread 3: B -> D (ascending order)
    {
        let (r2, r4) = (resource_b.clone(), resource_d.clone());
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(75));
            let _lock_b = lock!(r2);
            thread::sleep(Duration::from_millis(50));
            let _lock_d = lock!(r4);
            thread::sleep(Duration::from_millis(100));
        }));
    }

    // Thread 4: Single lock operations
    {
        let r2 = resource_b.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_millis(30));
                let _lock = lock!(r2);
                thread::sleep(Duration::from_millis(20));
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start.elapsed().as_secs_f64();
    
    // Log to fp_tests directory
    let filename = format!("fp_tests/four_hier_fp_{}.csv", feature_name());
    append_log_no_seed(&filename, false, false, elapsed);
}
