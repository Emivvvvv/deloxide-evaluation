// five_lock_cycle.rs - Heisenbug version with randomized timing

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let deadlock_detected = init_detectors();
    let seed = get_heisenbug_seed();
    
    // 5 locks, 5 threads, circular dependency
    // Thread 0: A→B→C→D→E
    // Thread 1: B→C→D→E→A
    // Thread 2: C→D→E→A→B
    // Thread 3: D→E→A→B→C
    // Thread 4: E→A→B→C→D

    let a = Arc::new(Mutex::new(()));
    let b = Arc::new(Mutex::new(()));
    let c = Arc::new(Mutex::new(()));
    let d = Arc::new(Mutex::new(()));
    let e = Arc::new(Mutex::new(()));

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    // Thread 0: A→B→C→D→E
    {
        let (l0, l1, l2, l3, l4) = (a.clone(), b.clone(), c.clone(), d.clone(), e.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed);
            thread::sleep(Duration::from_micros(rng.random_range(0..200)));
            let _g0 = lock!(l0); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.6) { thread::yield_now(); }
            let _g1 = lock!(l1); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.6) { thread::yield_now(); }
            let _g2 = lock!(l2); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.6) { thread::yield_now(); }
            let _g3 = lock!(l3); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.6) { thread::yield_now(); }
            let _g4 = lock!(l4);
        });
    }

    // Thread 1: B→C→D→E→A
    {
        let (l0, l1, l2, l3, l4) = (b.clone(), c.clone(), d.clone(), e.clone(), a.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed + 1);
            thread::sleep(Duration::from_micros(rng.random_range(50..300)));
            let _g0 = lock!(l0); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.7) { thread::yield_now(); }
            let _g1 = lock!(l1); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.7) { thread::yield_now(); }
            let _g2 = lock!(l2); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.7) { thread::yield_now(); }
            let _g3 = lock!(l3); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.7) { thread::yield_now(); }
            let _g4 = lock!(l4);
        });
    }

    // Thread 2: C→D→E→A→B
    {
        let (l0, l1, l2, l3, l4) = (c.clone(), d.clone(), e.clone(), a.clone(), b.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed + 2);
            thread::sleep(Duration::from_micros(rng.random_range(100..400)));
            let _g0 = lock!(l0); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.8) { thread::yield_now(); }
            let _g1 = lock!(l1); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.8) { thread::yield_now(); }
            let _g2 = lock!(l2); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.8) { thread::yield_now(); }
            let _g3 = lock!(l3); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.8) { thread::yield_now(); }
            let _g4 = lock!(l4);
        });
    }

    // Thread 3: D→E→A→B→C
    {
        let (l0, l1, l2, l3, l4) = (d.clone(), e.clone(), a.clone(), b.clone(), c.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed + 3);
            thread::sleep(Duration::from_micros(rng.random_range(150..500)));
            let _g0 = lock!(l0); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.7) { thread::yield_now(); }
            let _g1 = lock!(l1); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.7) { thread::yield_now(); }
            let _g2 = lock!(l2); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.7) { thread::yield_now(); }
            let _g3 = lock!(l3); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.7) { thread::yield_now(); }
            let _g4 = lock!(l4);
        });
    }

    // Thread 4: E→A→B→C→D
    {
        let (l0, l1, l2, l3, l4) = (e.clone(), a.clone(), b.clone(), c.clone(), d.clone());
        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(seed + 4);
            thread::sleep(Duration::from_micros(rng.random_range(200..600)));
            let _g0 = lock!(l0); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.6) { thread::yield_now(); }
            let _g1 = lock!(l1); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.6) { thread::yield_now(); }
            let _g2 = lock!(l2); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.6) { thread::yield_now(); }
            let _g3 = lock!(l3); thread::sleep(Duration::from_micros(rng.random_range(15..80)));
            if rng.random_bool(0.6) { thread::yield_now(); }
            let _g4 = lock!(l4);
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, 5);
    
    let filename = csv_filename("five_lock_cycle");
    append_log(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
