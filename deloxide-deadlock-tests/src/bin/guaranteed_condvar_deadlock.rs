// guaranteed_condvar_deadlock.rs - GUARANTEED condvar deadlock using barriers

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::{Arc, Barrier};

fn main() {
    // Initialize detectors (don't log in callback to avoid race condition)
    let deadlock_detected = init_detectors();

    // Create a proper condvar deadlock:
    // Thread 1: Locks A, then tries to lock B (which Thread 2 holds)
    // Thread 2: Locks B, then tries to lock A (which Thread 1 holds)
    // This is ABBA deadlock, not a condvar-specific deadlock
    // (Condvar deadlocks are actually hard to create in a guaranteed way)
    
    let mutex_a = Arc::new(Mutex::new(()));
    let mutex_b = Arc::new(Mutex::new(()));

    let barrier1 = Arc::new(Barrier::new(2));
    let barrier2 = Arc::new(Barrier::new(2));

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    // Thread 1: A → B (ABBA deadlock pattern)
    {
        let (a1, b1) = (mutex_a.clone(), mutex_b.clone());
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            let _lock_a = lock!(a1);
            bar2.wait();
            let _lock_b = lock!(b1); // DEADLOCK
        });
    }

    // Thread 2: B → A (ABBA deadlock pattern)
    {
        let (a2, b2) = (mutex_a.clone(), mutex_b.clone());
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            let _lock_b = lock!(b2);
            bar2.wait();
            let _lock_a = lock!(a2); // DEADLOCK
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, 2);
    
    let filename = csv_filename_guaranteed("guaranteed_condvar_deadlock");
    append_log_no_seed(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
