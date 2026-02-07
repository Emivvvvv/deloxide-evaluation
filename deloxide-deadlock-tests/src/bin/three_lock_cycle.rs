// three_lock_cycle.rs - Heisenbug version with randomized timing

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
    let c = Arc::new(Mutex::new(()));

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    // Thread 1: A → B → C
    {
        let (a1, b1, c1) = (a.clone(), b.clone(), c.clone());
        let flag1 = deadlock_detected.clone();
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed);
            
            thread::sleep(Duration::from_micros(rng.random_range(0..150)));
            let _la = lock!(a1);
            thread::sleep(Duration::from_micros(rng.random_range(20..100)));

            if rng.random_bool(0.7) {
                thread::yield_now();
            }

            if flag1.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before acquiring B");
            }
            let _lb = lock!(b1);
            thread::sleep(Duration::from_micros(rng.random_range(20..100)));

            if rng.random_bool(0.6) {
                thread::yield_now();
            }

            if flag1.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before acquiring C");
            }
            let _lc = lock!(c1);
        });
    }

    // Thread 2: B → C → A
    {
        let (a2, b2, c2) = (a.clone(), b.clone(), c.clone());
        let flag2 = deadlock_detected.clone();
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed + 1);
            
            thread::sleep(Duration::from_micros(rng.random_range(50..250)));

            if flag2.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before acquiring B");
            }
            let _lb = lock!(b2);
            thread::sleep(Duration::from_micros(rng.random_range(20..100)));
            
            if rng.random_bool(0.8) {
                thread::yield_now();
            }

            if flag2.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before acquiring C");
            }
            let _lc = lock!(c2);
            thread::sleep(Duration::from_micros(rng.random_range(20..100)));

            if rng.random_bool(0.7) {
                thread::yield_now();
            }

            if flag2.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before acquiring A");
            }
            let _la = lock!(a2);
        });
    }

    // Thread 3: C → A → B
    {
        let (a3, b3, c3) = (a.clone(), b.clone(), c.clone());
        let flag3 = deadlock_detected.clone();
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed + 2);
            
            thread::sleep(Duration::from_micros(rng.random_range(100..350)));

            if flag3.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before acquiring C");
            }
            let _lc = lock!(c3);
            thread::sleep(Duration::from_micros(rng.random_range(20..100)));
            
            if rng.random_bool(0.8) {
                thread::yield_now();
            }

            if flag3.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before acquiring A");
            }
            let _la = lock!(a3);
            thread::sleep(Duration::from_micros(rng.random_range(20..100)));

            if rng.random_bool(0.7) {
                thread::yield_now();
            }

            if flag3.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before acquiring B");
            }
            let _lb = lock!(b3);
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, 3);
    
    let filename = csv_filename("three_lock_cycle");
    append_log(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
