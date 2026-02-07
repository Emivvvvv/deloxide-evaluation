// two_lock.rs - Heisenbug version with randomized timing

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let deadlock_detected = init_detectors();
    let seed = get_heisenbug_seed();
    
    let a = Arc::new(Mutex::new(()));
    let b = Arc::new(Mutex::new(()));

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    // Thread 1: A → B
    {
        let a1 = a.clone();
        let b1 = b.clone();
        let flag1 = deadlock_detected.clone();
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed);
            
            let _lock_a = lock!(a1);
            thread::sleep(Duration::from_micros(rng.random_range(10..100)));
            
            // Probabilistic yields
            if rng.random_bool(0.7) {
                thread::yield_now();
            }
            if rng.random_bool(0.5) {
                thread::yield_now();
            }

            if flag1.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before locking B");
            }
            let _lock_b = lock!(b1);
        });
    }

    // Thread 2: B → A
    {
        let a2 = a.clone();
        let b2 = b.clone();
        let flag2 = deadlock_detected.clone();
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed + 1);
            
            thread::sleep(Duration::from_micros(rng.random_range(50..150)));
            
            if rng.random_bool(0.6) {
                thread::yield_now();
            }

            if flag2.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before locking B");
            }
            let _lock_b = lock!(b2);
            
            if rng.random_bool(0.7) {
                thread::yield_now();
            }

            if flag2.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before locking A");
            }
            let _lock_a = lock!(a2);
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, 2);
    
    let filename = csv_filename("two_lock");
    append_log(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
