// dining_philosophers.rs - Heisenbug version with randomized timing

use deadlock_detector_benchmark::test_harness::*;
use deadlock_detector_benchmark::lock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let deadlock_detected = init_detectors();
    let seed = get_heisenbug_seed();
    
    const NUM_PHILOSOPHERS: usize = 5;

    // Create forks (chopsticks)
    let forks: Vec<_> = (0..NUM_PHILOSOPHERS)
        .map(|_| Arc::new(Mutex::new(())))
        .collect();

    let (tx, rx) = mpsc::channel::<bool>();
    let runner = TestRunner::new(deadlock_detected.clone());

    // Spawn philosophers
    for i in 0..NUM_PHILOSOPHERS {
        let left_fork = forks[i].clone();
        let right_fork = forks[(i + 1) % NUM_PHILOSOPHERS].clone();
        let flag = deadlock_detected.clone();
        let philosopher_seed = seed + i as u64;

        spawn_with_panic_catch(tx.clone(), move || {
            let mut rng = StdRng::seed_from_u64(philosopher_seed);
            
            thread::sleep(Duration::from_micros(rng.random_range(10..100)));

            // Each philosopher tries to pick up left fork first, then right fork
            // This creates circular wait: P0→F0→F1, P1→F1→F2, ..., P4→F4→F0
            let _left = lock!(left_fork);
            thread::sleep(Duration::from_micros(rng.random_range(15..60)));
            
            if rng.random_bool(0.7) {
                thread::yield_now();
            }

            if flag.load(std::sync::atomic::Ordering::SeqCst) {
                panic!("Detected before acquiring right fork");
            }

            let _right = lock!(right_fork);

            // Eat (never reached in deadlock)
            thread::sleep(Duration::from_micros(rng.random_range(20..80)));
        });
    }

    drop(tx);
    let (tool_flagged, timed_out, elapsed) = runner.wait_for_completion(rx, NUM_PHILOSOPHERS);
    
    let filename = csv_filename("dining_philosophers");
    append_log(&filename, tool_flagged, timed_out, elapsed);

    if tool_flagged || timed_out {
        std::process::exit(0);
    }
}
