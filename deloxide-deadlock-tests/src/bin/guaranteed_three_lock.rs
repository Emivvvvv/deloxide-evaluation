// guaranteed_three_lock.rs - GUARANTEED 3-way deadlock using barriers

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::{Arc, Barrier};

fn main() {
    // Initialize detectors (don't log in callback to avoid race condition)
    let deadlock_detected = init_detectors();

    let a = Arc::new(Mutex::new(()));
    let b = Arc::new(Mutex::new(()));
    let c = Arc::new(Mutex::new(()));

    let barrier1 = Arc::new(Barrier::new(3));
    let barrier2 = Arc::new(Barrier::new(3));

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    // Thread 1: A → B
    {
        let (a1, b1) = (a.clone(), b.clone());
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            let _lock_a = lock!(a1);
            bar2.wait();
            let _lock_b = lock!(b1); // DEADLOCK
        });
    }

    // Thread 2: B → C
    {
        let (b2, c2) = (b.clone(), c.clone());
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            let _lock_b = lock!(b2);
            bar2.wait();
            let _lock_c = lock!(c2); // DEADLOCK
        });
    }

    // Thread 3: C → A
    {
        let (c3, a3) = (c.clone(), a.clone());
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            let _lock_c = lock!(c3);
            bar2.wait();
            let _lock_a = lock!(a3); // DEADLOCK
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, 3);
    
    let filename = csv_filename_guaranteed("guaranteed_three_lock");
    append_log_no_seed(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
