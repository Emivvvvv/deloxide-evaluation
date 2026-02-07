// rwlock_deadlock.rs - Heisenbug version with randomized timing

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::write_lock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let deadlock_detected = init_detectors();
    let seed = get_heisenbug_seed();
    
    // Write-Write circular wait deadlock
    // Thread1: write(A) → write(B)
    // Thread2: write(B) → write(A)
    
    let rw_a = Arc::new(RwLock::new(0i32));
    let rw_b = Arc::new(RwLock::new(0i32));

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    // Thread 1: write(A) -> write(B)
    {
        let a1 = rw_a.clone();
        let b1 = rw_b.clone();
        let flag1 = deadlock_detected.clone();
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed);
            
            let mut _wa = write_lock!(a1);
            thread::sleep(Duration::from_micros(rng.random_range(5..40)));

            if rng.random_bool(0.6) {
                thread::yield_now();
            }

            if flag1.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before writing B");
            }

            let mut _wb = write_lock!(b1);
            *_wa += 1;
            *_wb += 1;
        });
    }

    // Thread 2: write(B) -> write(A)
    {
        let a2 = rw_a.clone();
        let b2 = rw_b.clone();
        let flag2 = deadlock_detected.clone();
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed + 1);
            
            thread::sleep(Duration::from_micros(rng.random_range(10..50)));

            if flag2.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before writing B");
            }

            let mut _wb = write_lock!(b2);
            
            if rng.random_bool(0.7) {
                thread::yield_now();
            }

            if flag2.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before writing A");
            }

            let mut _wa = write_lock!(a2);
            *_wa += 1;
            *_wb += 1;
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, 2);
    
    let filename = csv_filename("rwlock_deadlock");
    append_log(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
