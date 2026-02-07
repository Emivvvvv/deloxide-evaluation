# False Positive Verification

## Overview

This analysis verifies two types of false positive tests:

1. **Traditional FP Tests**: Should NEVER trigger any detector
2. **Lock Order FP Tests**: Should trigger lock order detectors but NOT wait-for graph detectors

## Test Results

| Test                                                      | Type           | Detector   |   Runs |   Flagged |   Expected |   False Positives |   Avg Time (s) | Status                  |
|:----------------------------------------------------------|:---------------|:-----------|-------:|----------:|-----------:|------------------:|---------------:|:------------------------|
| complex_lock_order_fp_deloxide                            | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.058  | ✅ PASS                  |
| complex_lock_order_fp_deloxide_aggressive                 | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0832 | ✅ PASS                  |
| complex_lock_order_fp_deloxide_component_based_delays     | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.068  | ✅ PASS                  |
| complex_lock_order_fp_deloxide_gentle                     | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0563 | ✅ PASS                  |
| complex_lock_order_fp_deloxide_lock_order                 | Lock Order FP  | Lock Order |      1 |         1 |          0 |                 0 |         0.0573 | ❌ KNOWN FP (Lock Order) |
| complex_lock_order_fp_deloxide_random_default             | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0601 | ✅ PASS                  |
| complex_lock_order_fp_no_deadlocks                        | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.056  | ✅ PASS                  |
| complex_lock_order_fp_parking_lot                         | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0542 | ✅ PASS                  |
| conditional_locking_fp_deloxide                           | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        24.8585 | ✅ PASS                  |
| conditional_locking_fp_deloxide_aggressive                | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        25.5446 | ✅ PASS                  |
| conditional_locking_fp_deloxide_component_based_delays    | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        25.193  | ✅ PASS                  |
| conditional_locking_fp_deloxide_gentle                    | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        24.877  | ✅ PASS                  |
| conditional_locking_fp_deloxide_lock_order                | Traditional FP | Lock Order |      1 |         0 |          0 |                 0 |        24.8747 | ✅ PASS                  |
| conditional_locking_fp_deloxide_random_default            | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        24.9991 | ✅ PASS                  |
| conditional_locking_fp_no_deadlocks                       | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        78.6235 | ✅ PASS                  |
| conditional_locking_fp_parking_lot                        | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        24.4613 | ✅ PASS                  |
| four_hier_fp_deloxide                                     | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.6103 | ✅ PASS                  |
| four_hier_fp_deloxide_aggressive                          | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.6309 | ✅ PASS                  |
| four_hier_fp_deloxide_component_based_delays              | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.621  | ✅ PASS                  |
| four_hier_fp_deloxide_gentle                              | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.6218 | ✅ PASS                  |
| four_hier_fp_deloxide_lock_order                          | Traditional FP | Lock Order |      1 |         0 |          0 |                 0 |         0.617  | ✅ PASS                  |
| four_hier_fp_deloxide_random_default                      | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.6206 | ✅ PASS                  |
| four_hier_fp_no_deadlocks                                 | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.5021 | ✅ PASS                  |
| four_hier_fp_parking_lot                                  | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.6157 | ✅ PASS                  |
| gate_guarded_fp_deloxide                                  | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.4542 | ✅ PASS                  |
| gate_guarded_fp_deloxide_aggressive                       | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.4637 | ✅ PASS                  |
| gate_guarded_fp_deloxide_component_based_delays           | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.46   | ✅ PASS                  |
| gate_guarded_fp_deloxide_gentle                           | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.4545 | ✅ PASS                  |
| gate_guarded_fp_deloxide_lock_order                       | Traditional FP | Lock Order |      1 |         0 |          0 |                 0 |         0.4508 | ✅ PASS                  |
| gate_guarded_fp_deloxide_random_default                   | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.4683 | ✅ PASS                  |
| gate_guarded_fp_no_deadlocks                              | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.4641 | ✅ PASS                  |
| gate_guarded_fp_parking_lot                               | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.4486 | ✅ PASS                  |
| lock_free_interval_fp_deloxide                            | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.0066 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_aggressive                 | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.0221 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_component_based_delays     | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.0206 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_gentle                     | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.0321 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_lock_order                 | Traditional FP | Lock Order |      1 |         0 |          0 |                 0 |         1.0111 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_random_default             | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.0098 | ✅ PASS                  |
| lock_free_interval_fp_no_deadlocks                        | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.0182 | ✅ PASS                  |
| lock_free_interval_fp_parking_lot                         | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.9692 | ✅ PASS                  |
| lock_order_inversion_fp_deloxide                          | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0453 | ✅ PASS                  |
| lock_order_inversion_fp_deloxide_aggressive               | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0501 | ✅ PASS                  |
| lock_order_inversion_fp_deloxide_component_based_delays   | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0533 | ✅ PASS                  |
| lock_order_inversion_fp_deloxide_gentle                   | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0495 | ✅ PASS                  |
| lock_order_inversion_fp_deloxide_lock_order               | Lock Order FP  | Lock Order |      1 |         1 |          0 |                 0 |         0.0494 | ❌ KNOWN FP (Lock Order) |
| lock_order_inversion_fp_deloxide_random_default           | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0547 | ✅ PASS                  |
| lock_order_inversion_fp_no_deadlocks                      | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.049  | ✅ PASS                  |
| lock_order_inversion_fp_parking_lot                       | Lock Order FP  | Wait-For   |      1 |         0 |          0 |                 0 |         0.0445 | ✅ PASS                  |
| producer_consumer_fp_deloxide                             | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.6159 | ✅ PASS                  |
| producer_consumer_fp_deloxide_aggressive                  | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.7035 | ✅ PASS                  |
| producer_consumer_fp_deloxide_component_based_delays      | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.6621 | ✅ PASS                  |
| producer_consumer_fp_deloxide_gentle                      | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.6159 | ✅ PASS                  |
| producer_consumer_fp_deloxide_lock_order                  | Traditional FP | Lock Order |      1 |         0 |          0 |                 0 |         0.6088 | ✅ PASS                  |
| producer_consumer_fp_deloxide_random_default              | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.7628 | ✅ PASS                  |
| producer_consumer_fp_no_deadlocks                         | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         2.8539 | ✅ PASS                  |
| producer_consumer_fp_parking_lot                          | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         0.5674 | ✅ PASS                  |
| read_dominated_fp_deloxide                                | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.8202 | ✅ PASS                  |
| read_dominated_fp_deloxide_aggressive                     | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.8054 | ✅ PASS                  |
| read_dominated_fp_deloxide_component_based_delays         | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.8285 | ✅ PASS                  |
| read_dominated_fp_deloxide_gentle                         | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.8242 | ✅ PASS                  |
| read_dominated_fp_deloxide_lock_order                     | Traditional FP | Lock Order |      1 |         0 |          0 |                 0 |         1.8336 | ✅ PASS                  |
| read_dominated_fp_deloxide_random_default                 | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.831  | ✅ PASS                  |
| read_dominated_fp_no_deadlocks                            | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         3.5114 | ✅ PASS                  |
| read_dominated_fp_parking_lot                             | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |         1.689  | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide                        | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        24.1465 | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_aggressive             | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        25.1566 | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_component_based_delays | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        24.6767 | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_gentle                 | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        24.227  | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_lock_order             | Traditional FP | Lock Order |      1 |         0 |          0 |                 0 |        24.1916 | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_random_default         | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        24.5569 | ✅ PASS                  |
| thread_local_hierarchy_fp_no_deadlocks                    | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        45.0234 | ✅ PASS                  |
| thread_local_hierarchy_fp_parking_lot                     | Traditional FP | Wait-For   |      1 |         0 |          0 |                 0 |        23.3946 | ✅ PASS                  |

## Interpretation

- **Type**: Traditional FP or Lock Order FP
- **Detector**: Wait-For (runtime) or Lock Order (static analysis)
- **Flagged**: Number of times the detector flagged a deadlock
- **Expected**: Expected number of flags (should always be 0 for FP tests)
- **False Positives**: Number of incorrect detections
- **Status**:
  - ✅ PASS: No false positives detected
  - ❌ KNOWN FP (Lock Order): Known limitation of lock order graph detection
  - ❌ FALSE POS: Incorrect detection (false positive)

## Summary

- **Total test configurations**: 72
- **Passed (no false positives)**: 70/72
- **Traditional FP tests**: 56
  - False positives: 0
- **Lock Order FP tests**: 16
  - Total false positives: 0
  - Known FP (lock order detectors): 2
  - Unexpected FP (wait-for detectors): 0
- **Total false positives across all tests**: 2

## Analysis

### False Positive Breakdown:
- **Traditional FP tests**: 0 false positives
- **Lock Order FP tests (known limitation)**: 2 configurations
- **Lock Order FP tests (unexpected)**: 0 false positives

✅ **GOOD**: No unexpected false positives!

- Traditional FP tests: All passed correctly
- Lock order FP tests: Only flagged by lock order detectors (expected limitation)

⚠️  **Note**: 2 known false positives from lock order graph detection
   These demonstrate the limitation of static lock order analysis.
