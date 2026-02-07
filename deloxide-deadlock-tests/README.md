# deloxide-deadlock-tests

This directory contains a comprehensive suite of deadlock detection tests for evaluating `deloxide`.

## Purpose

This suite is designed to rigorously test the correctness of the deadlock detector. It verifies that:
1.  **True Positives**: Actual deadlocks are correctly detected and reported.
2.  **False Positives**: Valid locking patterns that *look* suspicious (e.g., false cycles in the lock graph) are *not* flagged as deadlocks.
3.  **Guaranteed Deadlocks**: Deterministic scenarios that force a deadlock state to ensure the detector triggers reliably.

## Content

The tests are organized into three categories:

### 1. True Positives (Heisenbugs)
Tests that intentionally create deadlock cycles. Many of these are **Heisenbugs** (transient, timing-dependent bugs) that do not occur on every run.
*   **Simple Cycles**: `two_lock`, `three_lock_cycle`, `five_lock_cycle`.
*   **Complex Scenarios**: `dining_philosophers`, `rwlock_deadlock`.

**Note on Detection Rates:**
Since these tests rely on probabilistic thread interleavings, a "pass" means the deadlock was *eventually* caught. The analysis scripts measure the **Heisenbug Detection Rate**—i.e., the probability that a deadlock is detected within a certain number of iterations or time limit. This metric is crucial for evaluating the effectiveness of the stress-testing features (`deloxide_random_default`, etc.) in forcing these elusive bugs to manifest.

### 2. False Positives (Valid Patterns)
Tests with complex synchronization patterns that do NOT deadlock, ensuring the detector is precise.
*   **Gate Guarded**: `gate_guarded_fp` - A/B locking protected by timing/control flow.
*   **Lock Free Interval**: `lock_free_interval_fp`.
*   **Read Dominated**: `read_dominated_fp`.
*   **Thread Local Hierarchy**: `thread_local_hierarchy_fp`.

### 3. Guaranteed Deadlocks
Deterministic versions of deadlock tests using `Barriers` to force specific thread interleavings, removing timing dependence.
*   `guaranteed_two_lock`
*   `guaranteed_dining_philosophers`
*   `guaranteed_rwlock_deadlock`

## Usage

### Running Tests

You can use the provided helper script `run_tests.sh` to execute tests with different configurations and features.

```bash
# Run all detection tests
./run_tests.sh detection

# Run false positive tests
./run_tests.sh false-positive

# Run guaranteed deadlock tests
./run_tests.sh guaranteed

# Run all tests
./run_tests.sh all
```

The script supports running tests with specific features over multiple iterations to catch Heisenbugs.

### Analysis

For in-depth analysis of the test results, use the python analysis suite:

```bash
# Run all analysis scripts and generate reports
python3 analysis/run_all_analysis.py
```

This master script triggers:
*   `analyze_detection_rate.py`: Calculates how often deadlocks are caught.
*   `verify_false_positives.py`: Confirms that false positive tests do NOT report deadlocks.
*   `compare_detectors.py`: Compares `deloxide` against other tools.

### Manual Execution

Each test is also an individual binary. To run a specific test manually:

```bash
cargo run --bin two_lock
```
