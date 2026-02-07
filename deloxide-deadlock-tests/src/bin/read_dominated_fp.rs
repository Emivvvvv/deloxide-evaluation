// Read-Dominated - False Positive Test
// Many readers with RwLock should not trigger false positives

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::read_lock;
use deadlock_detector_benchmark::write_lock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let _deadlock_detected = init_detectors();
    let start = std::time::Instant::now();

    let rw_a = Arc::new(RwLock::new(0i32));
    let rw_b = Arc::new(RwLock::new(0i32));
    let rw_c = Arc::new(RwLock::new(0i32));

    let mut handles = vec![];

    // Spawn many reader threads with various read patterns
    for tid in 0..8 {
        let a = rw_a.clone();
        let b = rw_b.clone();
        let c = rw_c.clone();

        handles.push(thread::spawn(move || {
            for _ in 0..50 {
                match tid % 4 {
                    0 => {
                        // A → B → C
                        let _ra = read_lock!(a);
                        let _rb = read_lock!(b);
                        let _rc = read_lock!(c);
                        thread::sleep(Duration::from_millis(10));
                    }
                    1 => {
                        // C → B → A
                        let _rc = read_lock!(c);
                        let _rb = read_lock!(b);
                        let _ra = read_lock!(a);
                        thread::sleep(Duration::from_millis(10));
                    }
                    2 => {
                        // B → A → C
                        let _rb = read_lock!(b);
                        let _ra = read_lock!(a);
                        let _rc = read_lock!(c);
                        thread::sleep(Duration::from_millis(10));
                    }
                    _ => {
                        // A → C (skip B)
                        let _ra = read_lock!(a);
                        let _rc = read_lock!(c);
                        thread::sleep(Duration::from_millis(10));
                    }
                }
                thread::sleep(Duration::from_millis(20));
            }
        }));
    }

    // Add one occasional writer
    {
        let a = rw_a.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                thread::sleep(Duration::from_millis(100));
                let mut _wa = write_lock!(a);
                *_wa += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let filename = format!("fp_tests/read_dominated_fp_{}.csv", feature_name());
    append_log_no_seed(&filename, false, false, elapsed);
}
