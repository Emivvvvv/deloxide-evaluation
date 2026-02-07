// guaranteed_two_lock.rs - GUARANTEED deadlock using barriers

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::{Arc, Barrier};

fn main() {
    // Initialize detectors (don't log in callback to avoid race condition)
    let deadlock_detected = init_detectors();

    let a = Arc::new(Mutex::new(()));
    let b = Arc::new(Mutex::new(()));

    let barrier1 = Arc::new(Barrier::new(2));
    let barrier2 = Arc::new(Barrier::new(2));

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    // Thread 1: A → B
    {
        let (a1, b1) = (a.clone(), b.clone());
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());
        let flag1 = deadlock_detected.clone();
        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            let _lock_a = lock!(a1);
            bar2.wait();
            if flag1.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before locking B");
            }
            let _lock_b = lock!(b1); // DEADLOCK
        });
    }

    // Thread 2: B → A
    {
        let (a2, b2) = (a.clone(), b.clone());
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());
        let flag2 = deadlock_detected.clone();
        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            if flag2.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before locking B");
            }
            let _lock_b = lock!(b2);
            bar2.wait();
            if flag2.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before locking A");
            }
            let _lock_a = lock!(a2); // DEADLOCK
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, 2);
    
    let filename = csv_filename_guaranteed("guaranteed_two_lock");
    append_log_no_seed(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
