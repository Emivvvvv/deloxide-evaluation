// test_harness.rs - Central system for deadlock testing
// Handles feature-specific initialization, mutex types, and logging

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

// Re-export types that tests will need
pub use rand::rngs::StdRng;
pub use rand::{Rng, SeedableRng};
pub use std::panic;
pub use std::sync::mpsc;
pub use std::sync::Arc;
pub use std::time::Instant;

// Feature-specific mutex type
#[cfg(feature = "parking_lot_deadlock")]
pub type Mutex<T> = parking_lot::Mutex<T>;

#[cfg(feature = "use_std")]
pub type Mutex<T> = std::sync::Mutex<T>;

#[cfg(any(
    feature = "deloxide",
    feature = "stress_random",
    feature = "stress_component",
    feature = "stress_aggressive",
    feature = "stress_gentle",
    feature = "deloxide_lock_order"
))]
pub type Mutex<T> = deloxide::Mutex<T>;

#[cfg(feature = "no_deadlocks")]
pub type Mutex<T> = no_deadlocks::Mutex<T>;

// Feature-specific RwLock type
#[cfg(feature = "parking_lot_deadlock")]
pub type RwLock<T> = parking_lot::RwLock<T>;

#[cfg(feature = "use_std")]
pub type RwLock<T> = std::sync::RwLock<T>;

#[cfg(any(
    feature = "deloxide",
    feature = "stress_random",
    feature = "stress_component",
    feature = "stress_aggressive",
    feature = "stress_gentle",
    feature = "deloxide_lock_order"
))]
pub type RwLock<T> = deloxide::RwLock<T>;

#[cfg(feature = "no_deadlocks")]
pub type RwLock<T> = no_deadlocks::RwLock<T>;

// Feature-specific Condvar type
#[cfg(feature = "parking_lot_deadlock")]
pub type Condvar = parking_lot::Condvar;

#[cfg(feature = "use_std")]
pub type Condvar = std::sync::Condvar;

#[cfg(any(
    feature = "deloxide",
    feature = "stress_random",
    feature = "stress_component",
    feature = "stress_aggressive",
    feature = "stress_gentle",
    feature = "deloxide_lock_order"
))]
pub type Condvar = deloxide::Condvar;

#[cfg(feature = "no_deadlocks")]
pub type Condvar = no_deadlocks::Condvar;

// Macro to simplify lock acquisition across features
#[macro_export]
macro_rules! lock {
    ($mutex:expr) => {{
        #[cfg(any(
            feature = "parking_lot_deadlock",
            feature = "deloxide",
            feature = "stress_random",
            feature = "stress_component",
            feature = "stress_aggressive",
            feature = "stress_gentle",
            feature = "deloxide_lock_order"
        ))]
        {
            $mutex.lock()
        }
        #[cfg(any(feature = "no_deadlocks", feature = "use_std"))]
        {
            $mutex.lock().unwrap()
        }
    }};
}

#[macro_export]
macro_rules! read_lock {
    ($rwlock:expr) => {{
        #[cfg(any(
            feature = "parking_lot_deadlock",
            feature = "deloxide",
            feature = "stress_random",
            feature = "stress_component",
            feature = "stress_aggressive",
            feature = "stress_gentle",
            feature = "deloxide_lock_order"
        ))]
        {
            $rwlock.read()
        }
        #[cfg(any(feature = "no_deadlocks", feature = "use_std"))]
        {
            $rwlock.read().unwrap()
        }
    }};
}

#[macro_export]
macro_rules! write_lock {
    ($rwlock:expr) => {{
        #[cfg(any(
            feature = "parking_lot_deadlock",
            feature = "deloxide",
            feature = "stress_random",
            feature = "stress_component",
            feature = "stress_aggressive",
            feature = "stress_gentle",
            feature = "deloxide_lock_order"
        ))]
        {
            $rwlock.write()
        }
        #[cfg(any(feature = "no_deadlocks", feature = "use_std"))]
        {
            $rwlock.write().unwrap()
        }
    }};
}

// Get feature name for CSV logging
pub fn feature_name() -> &'static str {
    #[cfg(feature = "stress_random")]
    return "stress_random";

    #[cfg(feature = "stress_component")]
    return "stress_component";
    #[cfg(feature = "stress_aggressive")]
    return "stress_aggressive";
    #[cfg(feature = "stress_gentle")]
    return "stress_gentle";
    #[cfg(feature = "deloxide_lock_order")]
    return "deloxide_lock_order";
    #[cfg(all(
        feature = "deloxide",
        not(feature = "stress_random"),
        not(feature = "stress_component"),
        not(feature = "stress_aggressive"),
        not(feature = "stress_gentle"),
        not(feature = "deloxide_lock_order")
    ))]
    return "deloxide";
    #[cfg(feature = "parking_lot_deadlock")]
    return "parking_lot";
    #[cfg(feature = "no_deadlocks")]
    return "no_deadlocks";
    #[cfg(feature = "use_std")]
    return "std";
    #[cfg(all(
        not(feature = "deloxide"),
        not(feature = "stress_random"),
        not(feature = "stress_component"),
        not(feature = "stress_aggressive"),
        not(feature = "stress_gentle"),
        not(feature = "deloxide_lock_order"),
        not(feature = "parking_lot_deadlock"),
        not(feature = "no_deadlocks"),
        not(feature = "use_std")
    ))]
    compile_error!("Enable exactly one feature");
}

// Get timeout duration based on feature
pub fn timeout_duration() -> Duration {
    Duration::from_secs(10)
}

// CSV logging
pub fn csv_filename(test_name: &str) -> String {
    format!("deadlock_tests/{}_{}.csv", test_name, feature_name())
}

pub fn csv_filename_guaranteed(test_name: &str) -> String {
    format!(
        "guaranteed_deadlock_tests/{}_{}.csv",
        test_name,
        feature_name()
    )
}

// Append log for Heisenbug tests (with seed)
pub fn append_log(filename: &str, tool_flagged: bool, timed_out: bool, elapsed: f64) {
    let seed = get_heisenbug_seed();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Failed to open CSV file");
    writeln!(file, "{tool_flagged}, {timed_out}, {elapsed:.6}, {seed}")
        .expect("Failed to write to CSV");
}

// Append log for guaranteed tests (without seed)
pub fn append_log_no_seed(filename: &str, tool_flagged: bool, timed_out: bool, elapsed: f64) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Failed to open CSV file");
    writeln!(file, "{tool_flagged}, {timed_out}, {elapsed:.6}").expect("Failed to write to CSV");
}

// Initialize deadlock detectors
pub fn init_detectors() -> Arc<AtomicBool> {
    let deadlock_detected = Arc::new(AtomicBool::new(false));

    #[cfg(feature = "parking_lot_deadlock")]
    {
        let flag = deadlock_detected.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_micros(10));
            let cycles = parking_lot::deadlock::check_deadlock();
            if !cycles.is_empty() {
                flag.store(true, Ordering::SeqCst);
                panic!("parking_lot detected a deadlock");
            }
        });
    }

    #[cfg(all(
        feature = "deloxide",
        not(feature = "stress_random"),
        not(feature = "stress_component"),
        not(feature = "stress_aggressive"),
        not(feature = "stress_gentle"),
        not(feature = "deloxide_lock_order")
    ))]
    {
        use deloxide::Deloxide;
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                panic!("Deloxide detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide");
    }

    #[cfg(feature = "stress_random")]
    {
        use deloxide::Deloxide;
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_random_stress()
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                panic!("Deloxide (random preemption) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with random preemption");
    }

    #[cfg(feature = "stress_component")]
    {
        use deloxide::Deloxide;
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_component_stress()
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                panic!("Deloxide (component delays) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with component-based delays");
    }

    #[cfg(feature = "stress_aggressive")]
    {
        use deloxide::{Deloxide, StressConfig};
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_random_stress()
            .with_stress_config(StressConfig::aggressive())
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                panic!("Deloxide (aggressive) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with aggressive config");
    }

    #[cfg(feature = "stress_gentle")]
    {
        use deloxide::{Deloxide, StressConfig};
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_random_stress()
            .with_stress_config(StressConfig::gentle())
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                panic!("Deloxide (gentle) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with gentle config");
    }

    #[cfg(feature = "deloxide_lock_order")]
    {
        use deloxide::Deloxide;
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_lock_order_checking()
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                panic!("Deloxide (lock order) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with lock order checking");
    }

    deadlock_detected
}

// Thread spawning helper with panic catching
pub fn spawn_with_panic_catch<F>(tx: mpsc::Sender<bool>, func: F) -> thread::JoinHandle<()>
where
    F: FnOnce() + Send + 'static,
{
    thread::spawn(move || {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(func));
        let _ = tx.send(result.is_ok());
    })
}

// Wait for threads with timeout and deadlock detection
pub struct TestRunner {
    pub deadlock_detected: Arc<AtomicBool>,
    pub timeout: Duration,
    pub start_time: Instant,
}

impl TestRunner {
    pub fn new(deadlock_detected: Arc<AtomicBool>) -> Self {
        Self {
            deadlock_detected,
            timeout: timeout_duration(),
            start_time: Instant::now(),
        }
    }

    pub fn wait_for_completion(
        &self,
        rx: mpsc::Receiver<bool>,
        expected_threads: usize,
    ) -> (bool, bool, f64) {
        let mut completed = 0;
        let mut tool_flagged = false;
        let mut timed_out = false;

        let deadline = Instant::now() + self.timeout;
        while Instant::now() < deadline {
            if self.deadlock_detected.load(Ordering::SeqCst) {
                tool_flagged = true;
                break;
            }
            match rx.try_recv() {
                Ok(status) => {
                    if !status {
                        tool_flagged = true;
                        break;
                    }
                    completed += 1;
                    if completed == expected_threads {
                        break;
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    tool_flagged = true;
                    break;
                }
            }
            thread::sleep(Duration::from_millis(1));
        }

        if !tool_flagged && completed < expected_threads {
            timed_out = true;
        }

        let elapsed = self.start_time.elapsed().as_secs_f64();
        (tool_flagged, timed_out, elapsed)
    }
}

// Initialize detectors for guaranteed tests with custom callback
#[allow(unused_variables)]
pub fn init_detectors_with_callback<F>(callback: F) -> Arc<AtomicBool>
where
    F: Fn() + Send + Sync + 'static,
{
    let deadlock_detected = Arc::new(AtomicBool::new(false));

    #[cfg(feature = "parking_lot_deadlock")]
    {
        let flag = deadlock_detected.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(50));
            let cycles = parking_lot::deadlock::check_deadlock();
            if !cycles.is_empty() {
                flag.store(true, Ordering::SeqCst);
                callback();
                panic!("parking_lot detected a deadlock");
            }
        });
    }

    #[cfg(all(
        feature = "deloxide",
        not(feature = "stress_random"),
        not(feature = "stress_component"),
        not(feature = "stress_aggressive"),
        not(feature = "stress_gentle"),
        not(feature = "deloxide_lock_order")
    ))]
    {
        use deloxide::Deloxide;
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                callback();
                panic!("Deloxide detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide");
    }

    #[cfg(feature = "stress_random")]
    {
        use deloxide::Deloxide;
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_random_stress()
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                callback();
                panic!("Deloxide (random preemption) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with random preemption");
    }

    #[cfg(feature = "stress_component")]
    {
        use deloxide::Deloxide;
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_component_stress()
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                callback();
                panic!("Deloxide (component delays) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with component-based delays");
    }

    #[cfg(feature = "stress_aggressive")]
    {
        use deloxide::{Deloxide, StressConfig};
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_random_stress()
            .with_stress_config(StressConfig::aggressive())
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                callback();
                panic!("Deloxide (aggressive) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with aggressive config");
    }

    #[cfg(feature = "stress_gentle")]
    {
        use deloxide::{Deloxide, StressConfig};
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_random_stress()
            .with_stress_config(StressConfig::gentle())
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                callback();
                panic!("Deloxide (gentle) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with gentle config");
    }

    #[cfg(feature = "deloxide_lock_order")]
    {
        use deloxide::Deloxide;
        let flag = deadlock_detected.clone();
        Deloxide::new()
            .with_lock_order_checking()
            .callback(move |_| {
                flag.store(true, Ordering::SeqCst);
                callback();
                panic!("Deloxide (lock order) detected a deadlock");
            })
            .start()
            .expect("Failed to initialize Deloxide with lock order checking");
    }

    deadlock_detected
}

// Heisenbug seed management for reproducible randomness across detectors
pub fn get_heisenbug_seed() -> u64 {
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Check if HEISENBUG_SEED environment variable is set
    if let Ok(seed_str) = env::var("HEISENBUG_SEED") {
        if let Ok(seed) = seed_str.parse::<u64>() {
            return seed;
        }
    }

    // Generate a new seed based on current time
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

// Helper to create a seeded RNG for Heisenbug tests
pub fn create_heisenbug_rng() -> StdRng {
    let seed = get_heisenbug_seed();
    StdRng::seed_from_u64(seed)
}
