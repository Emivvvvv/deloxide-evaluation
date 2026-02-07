// guaranteed_rwlock_deadlock.rs - GUARANTEED RwLock deadlock using barriers

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::write_lock;
use std::sync::{Arc, Barrier};

fn main() {
    // Initialize detectors (don't log in callback to avoid race condition)
    let deadlock_detected = init_detectors();

    let rw_a = Arc::new(RwLock::new(0i32));
    let rw_b = Arc::new(RwLock::new(0i32));

    let barrier1 = Arc::new(Barrier::new(2));
    let barrier2 = Arc::new(Barrier::new(2));

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    // Thread 1: write(A) → write(B)
    {
        let (a1, b1) = (rw_a.clone(), rw_b.clone());
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            let mut _wa = write_lock!(a1);
            bar2.wait();
            let mut _wb = write_lock!(b1); // DEADLOCK
            *_wa += 1;
            *_wb += 1;
        });
    }

    // Thread 2: write(B) → write(A)
    {
        let (a2, b2) = (rw_a.clone(), rw_b.clone());
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            let mut _wb = write_lock!(b2);
            bar2.wait();
            let mut _wa = write_lock!(a2); // DEADLOCK
            *_wa += 1;
            *_wb += 1;
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, 2);
    
    let filename = csv_filename_guaranteed("guaranteed_rwlock_deadlock");
    append_log_no_seed(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
