// lock_order_inversion_fp.rs - False positive for lock order graph detectors
// 
// This test creates a lock order inversion that forms a cycle in the lock order graph,
// but uses synchronization to ensure no actual circular wait occurs at runtime.
//
// Lock order graph will show: A→B (thread 1) and B→A (thread 2) = CYCLE
// But wait-for graph will never show a cycle because threads are synchronized
// to never hold locks simultaneously.

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let deadlock_detected = init_detectors();
    let start = std::time::Instant::now();
    
    let lock_a = Arc::new(Mutex::new(()));
    let lock_b = Arc::new(Mutex::new(()));
    
    // Synchronization flag to prevent actual deadlock
    let thread1_done = Arc::new(AtomicBool::new(false));

    let mut handles = vec![];

    // Thread 1: A → B (then signals thread 2 can start)
    {
        let a1 = lock_a.clone();
        let b1 = lock_b.clone();
        let done = thread1_done.clone();
        handles.push(thread::spawn(move || {
            // Acquire A then B
            let _lock_a = lock!(a1);
            thread::sleep(Duration::from_millis(10));
            
            let _lock_b = lock!(b1);
            thread::sleep(Duration::from_millis(10));
            
            // Release both locks (drop happens here)
            drop(_lock_b);
            drop(_lock_a);
            
            // Signal that thread 1 is done
            done.store(true, Ordering::SeqCst);
        }));
    }

    // Thread 2: B → A (waits for thread 1 to finish first)
    {
        let a2 = lock_a.clone();
        let b2 = lock_b.clone();
        let done = thread1_done.clone();
        handles.push(thread::spawn(move || {
            // Wait for thread 1 to complete
            while !done.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(1));
            }
            
            // Now acquire B then A (opposite order)
            let _lock_b = lock!(b2);
            thread::sleep(Duration::from_millis(10));
            
            let _lock_a = lock!(a2);
            thread::sleep(Duration::from_millis(10));
            
            // No deadlock possible because thread 1 released everything
        }));
    }

    for handle in handles {
        let _ = handle.join(); // Ignore panics from detector
    }

    let elapsed = start.elapsed().as_secs_f64();
    let tool_flagged = deadlock_detected.load(Ordering::SeqCst);
    
    let filename = format!("fp_tests/lock_order_inversion_fp_{}.csv", feature_name());
    append_log_no_seed(&filename, tool_flagged, false, elapsed);
}
