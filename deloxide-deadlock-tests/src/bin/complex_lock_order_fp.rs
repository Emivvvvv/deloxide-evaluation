// complex_lock_order_fp.rs - Complex false positive for lock order graph detectors
//
// This test creates multiple lock order inversions across several threads,
// forming a complex cycle in the lock order graph, but uses barriers to ensure
// no actual circular wait occurs at runtime.
//
// Lock order graph will show: A→B→C (thread 1), B→C→A (thread 2), C→A→B (thread 3) = CYCLE
// But wait-for graph will never show a cycle because threads execute in phases
// using barriers to ensure no overlapping lock holdings.

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let deadlock_detected = init_detectors();
    let start = std::time::Instant::now();
    
    let lock_a = Arc::new(Mutex::new(()));
    let lock_b = Arc::new(Mutex::new(()));
    let lock_c = Arc::new(Mutex::new(()));
    
    // Phase counter to synchronize threads
    let phase = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    // Thread 1: A → B → C (phase 0)
    {
        let (a, b, c) = (lock_a.clone(), lock_b.clone(), lock_c.clone());
        let phase_counter = phase.clone();
        handles.push(thread::spawn(move || {
            // Wait for phase 0
            while phase_counter.load(Ordering::SeqCst) != 0 {
                thread::sleep(Duration::from_millis(1));
            }
            
            let _a = lock!(a);
            thread::sleep(Duration::from_millis(5));
            let _b = lock!(b);
            thread::sleep(Duration::from_millis(5));
            let _c = lock!(c);
            thread::sleep(Duration::from_millis(5));
            
            drop(_c);
            drop(_b);
            drop(_a);
            
            // Move to phase 1
            phase_counter.store(1, Ordering::SeqCst);
        }));
    }

    // Thread 2: B → C → A (phase 1)
    {
        let (a, b, c) = (lock_a.clone(), lock_b.clone(), lock_c.clone());
        let phase_counter = phase.clone();
        handles.push(thread::spawn(move || {
            // Wait for phase 1
            while phase_counter.load(Ordering::SeqCst) != 1 {
                thread::sleep(Duration::from_millis(1));
            }
            
            let _b = lock!(b);
            thread::sleep(Duration::from_millis(5));
            let _c = lock!(c);
            thread::sleep(Duration::from_millis(5));
            let _a = lock!(a);
            thread::sleep(Duration::from_millis(5));
            
            drop(_a);
            drop(_c);
            drop(_b);
            
            // Move to phase 2
            phase_counter.store(2, Ordering::SeqCst);
        }));
    }

    // Thread 3: C → A → B (phase 2)
    {
        let (a, b, c) = (lock_a.clone(), lock_b.clone(), lock_c.clone());
        let phase_counter = phase.clone();
        handles.push(thread::spawn(move || {
            // Wait for phase 2
            while phase_counter.load(Ordering::SeqCst) != 2 {
                thread::sleep(Duration::from_millis(1));
            }
            
            let _c = lock!(c);
            thread::sleep(Duration::from_millis(5));
            let _a = lock!(a);
            thread::sleep(Duration::from_millis(5));
            let _b = lock!(b);
            thread::sleep(Duration::from_millis(5));
            
            // All done
        }));
    }

    for handle in handles {
        let _ = handle.join(); // Ignore panics from detector
    }

    let elapsed = start.elapsed().as_secs_f64();
    let tool_flagged = deadlock_detected.load(Ordering::SeqCst);
    
    let filename = format!("fp_tests/complex_lock_order_fp_{}.csv", feature_name());
    append_log_no_seed(&filename, tool_flagged, false, elapsed);
}
