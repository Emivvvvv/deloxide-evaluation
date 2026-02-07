# Deadlock Detection Benchmark Suite

Comprehensive benchmark suite for testing deadlock detection libraries in Rust.

## Quick Start

```bash
# Run a single test with all detectors
./run_tests.sh test-all-features two_lock

# Run all detection tests
./run_tests.sh detection

# Run with specific features only (e.g., Deloxide variants + parking_lot, no no_deadlocks)
./run_tests.sh detection -f deloxide,deloxide_random_default,deloxide_aggressive,deloxide_component_based_delays,parking_lot_deadlock

# Run detection tests with every Deloxide variant
./run_tests.sh detection --deloxide-all

# Run everything 10 times
./run_tests.sh all -n 10

# Get help
./run_tests.sh help
./run_tests.sh list-features
```

## Features

✅ **Unified Test Harness** - Single module handles all feature-specific complexity
✅ **Comprehensive Test Runner** - One script for all testing scenarios
✅ **Multiple Test Categories** - Detection, false positive, and guaranteed tests
✅ **6 Detector Support** - deloxide (4 variants), parking_lot, no_deadlocks
✅ **Clean Architecture** - 78% code reduction through centralization

## Test Categories

### Deadlock Detection Tests (6 tests)
Tests that contain actual deadlocks:
- `two_lock` - Simple ABBA deadlock
- `three_lock_cycle` - Three-way circular dependency
- `dining_philosophers` - Classic 5-philosopher problem
- `rwlock_deadlock` - RwLock write-write deadlock
- `five_lock_cycle` - Complex 5-lock circular wait

### False Positive Tests (7 tests)
Tests that should NOT deadlock:
- `four_hier` - Hierarchical lock ordering
- `gate_guarded` - Conditional locking
- `producer_consumer_fp` - Producer-consumer pattern
- `lock_free_interval_fp` - Lock-free intervals
- `conditional_locking_fp` - Conditional locks
- `read_dominated_fp` - Read-heavy workload
- `thread_local_hierarchy_fp` - Thread-local hierarchies

### Guaranteed Deadlock Tests (5 tests)
Tests with barriers ensuring 100% deadlock:
- `guaranteed_two_lock` - Guaranteed ABBA
- `guaranteed_three_lock` - Guaranteed 3-way cycle
- `guaranteed_dining_philosophers` - Guaranteed philosophers
- `guaranteed_rwlock_deadlock` - Guaranteed RwLock
- `guaranteed_condvar_deadlock` - Condvar deadlock

## Supported Detectors

- **deloxide** - Basic Deloxide detector
- **deloxide_random_default** - Deloxide with random stress
- **deloxide_aggressive** - Deloxide with aggressive stress
- **deloxide_component_based_delays** - Deloxide with component delays
- **parking_lot_deadlock** - parking_lot's deadlock detector
- **no_deadlocks** - no_deadlocks library

## Usage Examples

### Run Test Groups
```bash
# All detection tests
./run_tests.sh detection

# All false positive tests
./run_tests.sh false-positive

# All guaranteed tests
./run_tests.sh guaranteed

# Everything
./run_tests.sh all
```

### Run Individual Tests
```bash
# Single test with one detector
./run_tests.sh single two_lock deloxide

# Single test with all detectors
./run_tests.sh test-all-features two_lock
```

### Run with Iterations
```bash
# Run 10 times
./run_tests.sh detection -n 10

# Run 100 times quietly
./run_tests.sh all -n 100 --quiet
```

### Run with Specific Features
```bash
# Run with only specific detectors
./run_tests.sh detection -f deloxide,parking_lot_deadlock

# All deloxide variants + parking_lot (exclude no_deadlocks)
./run_tests.sh all -f deloxide,deloxide_random_default,deloxide_aggressive,deloxide_component_based_delays,parking_lot_deadlock

# Shortcut for every Deloxide feature
./run_tests.sh detection --deloxide-all

# Combine with iterations
./run_tests.sh detection -n 10 -f deloxide,parking_lot_deadlock

# List available features
./run_tests.sh list-features
```

### Compare Detectors
```bash
# Test same scenario with different detectors
./run_tests.sh single two_lock deloxide
./run_tests.sh single two_lock parking_lot_deadlock
./run_tests.sh single two_lock no_deadlocks

# Check results
cat deadlock_tests/two_lock_*.csv
```

### Direct Cargo Usage
```bash
# Run specific test with specific feature
cargo run --release --bin two_lock --features deloxide
cargo run --release --bin dining_philosophers --features parking_lot_deadlock
```

## Output

Results are saved to CSV files:

```
deadlock_tests/
├── two_lock_deloxide.csv
├── two_lock_parking_lot.csv
└── ...

guaranteed_deadlock_tests/
├── guaranteed_two_lock_deloxide.csv
└── ...
```

CSV format: `detected, timed_out, elapsed_time`

## Documentation

- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Quick command reference
- **[FEATURE_FILTERING_GUIDE.md](FEATURE_FILTERING_GUIDE.md)** - How to select specific detectors
- **[TEST_RUNNER_GUIDE.md](TEST_RUNNER_GUIDE.md)** - Complete usage guide
- **[REFACTORING_GUIDE.md](REFACTORING_GUIDE.md)** - How to add new tests
- **[MIGRATION_COMPLETE.md](MIGRATION_COMPLETE.md)** - What was done
- **[OLD_SCRIPTS_MIGRATION.md](OLD_SCRIPTS_MIGRATION.md)** - Migrating from old scripts

## Architecture

### Test Harness (`src/test_harness.rs`)
Central module that provides:
- Automatic detector initialization
- Feature-agnostic lock macros
- CSV logging helpers
- Thread management utilities
- Timeout handling

### Benefits
- **78% code reduction** per test file
- **Single source of truth** for feature handling
- **Easy to maintain** and extend
- **Consistent behavior** across all tests

## Building

```bash
# Build all tests
cargo build --release --bins

# Build specific test
cargo build --release --bin two_lock --features deloxide
```

## Requirements

- Rust (edition 2024)
- Dependencies:
  - `parking_lot` (optional, with deadlock_detection)
  - `deloxide` (optional, local path)
  - `no_deadlocks` (optional)
  - `anyhow`, `serde`, `csv`, `rand`

## Project Structure

```
.
├── src/
│   ├── lib.rs                    # Library entry point
│   ├── main.rs                   # Main binary (unused)
│   ├── test_harness.rs           # Central test harness
│   └── bin/                      # Test binaries
│       ├── two_lock.rs
│       ├── three_lock_cycle.rs
│       ├── dining_philosophers.rs
│       ├── rwlock_deadlock.rs
│       ├── five_lock_cycle.rs
│       ├── four_hier.rs
│       ├── gate_guarded.rs
│       ├── producer_consumer_fp.rs
│       ├── lock_free_interval_fp.rs
│       ├── conditional_locking_fp.rs
│       ├── read_dominated_fp.rs
│       ├── thread_local_hierarchy_fp.rs
│       ├── guaranteed_two_lock.rs
│       ├── guaranteed_three_lock.rs
│       ├── guaranteed_dining_philosophers.rs
│       ├── guaranteed_rwlock_deadlock.rs
│       └── guaranteed_condvar_deadlock.rs
├── run_tests.sh                  # Comprehensive test runner
├── Cargo.toml                    # Project configuration
└── docs/                         # Documentation
```
