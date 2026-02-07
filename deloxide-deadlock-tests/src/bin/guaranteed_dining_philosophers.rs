// guaranteed_dining_philosophers.rs - GUARANTEED philosopher deadlock using barriers

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::{Arc, Barrier};

fn main() {
    // Initialize detectors (don't log in callback to avoid race condition)
    let deadlock_detected = init_detectors();

    const NUM_PHILOSOPHERS: usize = 5;

    let forks: Vec<_> = (0..NUM_PHILOSOPHERS)
        .map(|_| Arc::new(Mutex::new(())))
        .collect();

    let barrier1 = Arc::new(Barrier::new(NUM_PHILOSOPHERS));
    let barrier2 = Arc::new(Barrier::new(NUM_PHILOSOPHERS));

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    for i in 0..NUM_PHILOSOPHERS {
        let left_fork = forks[i].clone();
        let right_fork = forks[(i + 1) % NUM_PHILOSOPHERS].clone();
        let (bar1, bar2) = (barrier1.clone(), barrier2.clone());

        spawn_with_panic_catch(tx.clone(), move || {
            bar1.wait();
            let _left = lock!(left_fork);
            bar2.wait();
            let _right = lock!(right_fork); // DEADLOCK
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, NUM_PHILOSOPHERS);
    
    let filename = csv_filename_guaranteed("guaranteed_dining_philosophers");
    append_log_no_seed(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
