# False Positive Verification

## Overview

This analysis verifies two types of false positive tests:

1. **Traditional FP Tests**: Should NEVER trigger any detector
2. **Lock Order FP Tests**: Should trigger lock order detectors but NOT wait-for graph detectors

## Test Results

| Test                                                      | Type           | Detector   |   Runs |   Flagged |   Expected |   False Positives |   Avg Time (s) | Status                  |
|:----------------------------------------------------------|:---------------|:-----------|-------:|----------:|-----------:|------------------:|---------------:|:------------------------|
| complex_lock_order_fp_deloxide                            | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0626 | ✅ PASS                  |
| complex_lock_order_fp_deloxide_aggressive                 | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.084  | ✅ PASS                  |
| complex_lock_order_fp_deloxide_component_based_delays     | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0744 | ✅ PASS                  |
| complex_lock_order_fp_deloxide_gentle                     | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0631 | ✅ PASS                  |
| complex_lock_order_fp_deloxide_lock_order                 | Lock Order FP  | Lock Order |     10 |        10 |          0 |                 0 |         0.0622 | ❌ KNOWN FP (Lock Order) |
| complex_lock_order_fp_deloxide_random_default             | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0685 | ✅ PASS                  |
| complex_lock_order_fp_no_deadlocks                        | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0641 | ✅ PASS                  |
| complex_lock_order_fp_parking_lot                         | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0583 | ✅ PASS                  |
| conditional_locking_fp_deloxide                           | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        25.5706 | ✅ PASS                  |
| conditional_locking_fp_deloxide_aggressive                | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        26.1583 | ✅ PASS                  |
| conditional_locking_fp_deloxide_component_based_delays    | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        25.907  | ✅ PASS                  |
| conditional_locking_fp_deloxide_gentle                    | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        25.6071 | ✅ PASS                  |
| conditional_locking_fp_deloxide_lock_order                | Traditional FP | Lock Order |     10 |         0 |          0 |                 0 |        25.6138 | ✅ PASS                  |
| conditional_locking_fp_deloxide_random_default            | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        25.7458 | ✅ PASS                  |
| conditional_locking_fp_no_deadlocks                       | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |       641.279  | ✅ PASS                  |
| conditional_locking_fp_parking_lot                        | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        24.1781 | ✅ PASS                  |
| four_hier_fp_deloxide                                     | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.6635 | ✅ PASS                  |
| four_hier_fp_deloxide_aggressive                          | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.6762 | ✅ PASS                  |
| four_hier_fp_deloxide_component_based_delays              | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.6666 | ✅ PASS                  |
| four_hier_fp_deloxide_gentle                              | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.6658 | ✅ PASS                  |
| four_hier_fp_deloxide_lock_order                          | Traditional FP | Lock Order |     10 |         0 |          0 |                 0 |         0.6613 | ✅ PASS                  |
| four_hier_fp_deloxide_random_default                      | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.6716 | ✅ PASS                  |
| four_hier_fp_no_deadlocks                                 | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         7.7394 | ✅ PASS                  |
| four_hier_fp_parking_lot                                  | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.6106 | ✅ PASS                  |
| gate_guarded_fp_deloxide                                  | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.5166 | ✅ PASS                  |
| gate_guarded_fp_deloxide_aggressive                       | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.5173 | ✅ PASS                  |
| gate_guarded_fp_deloxide_component_based_delays           | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.5236 | ✅ PASS                  |
| gate_guarded_fp_deloxide_gentle                           | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.5248 | ✅ PASS                  |
| gate_guarded_fp_deloxide_lock_order                       | Traditional FP | Lock Order |     10 |         0 |          0 |                 0 |         0.5245 | ✅ PASS                  |
| gate_guarded_fp_deloxide_random_default                   | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.5227 | ✅ PASS                  |
| gate_guarded_fp_no_deadlocks                              | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.5536 | ✅ PASS                  |
| gate_guarded_fp_parking_lot                               | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.4519 | ✅ PASS                  |
| lock_free_interval_fp_deloxide                            | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         1.1012 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_aggressive                 | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         1.1118 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_component_based_delays     | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         1.1125 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_gentle                     | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         1.0983 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_lock_order                 | Traditional FP | Lock Order |     10 |         0 |          0 |                 0 |         1.1064 | ✅ PASS                  |
| lock_free_interval_fp_deloxide_random_default             | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         1.1125 | ✅ PASS                  |
| lock_free_interval_fp_no_deadlocks                        | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         1.1022 | ✅ PASS                  |
| lock_free_interval_fp_parking_lot                         | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.9713 | ✅ PASS                  |
| lock_order_inversion_fp_deloxide                          | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.056  | ✅ PASS                  |
| lock_order_inversion_fp_deloxide_aggressive               | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0619 | ✅ PASS                  |
| lock_order_inversion_fp_deloxide_component_based_delays   | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0594 | ✅ PASS                  |
| lock_order_inversion_fp_deloxide_gentle                   | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0554 | ✅ PASS                  |
| lock_order_inversion_fp_deloxide_lock_order               | Lock Order FP  | Lock Order |     10 |        10 |          0 |                 0 |         0.0555 | ❌ KNOWN FP (Lock Order) |
| lock_order_inversion_fp_deloxide_random_default           | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0564 | ✅ PASS                  |
| lock_order_inversion_fp_no_deadlocks                      | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0546 | ✅ PASS                  |
| lock_order_inversion_fp_parking_lot                       | Lock Order FP  | Wait-For   |     10 |         0 |          0 |                 0 |         0.0502 | ✅ PASS                  |
| producer_consumer_fp_deloxide                             | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.7014 | ✅ PASS                  |
| producer_consumer_fp_deloxide_aggressive                  | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.7272 | ✅ PASS                  |
| producer_consumer_fp_deloxide_component_based_delays      | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.7194 | ✅ PASS                  |
| producer_consumer_fp_deloxide_gentle                      | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.7052 | ✅ PASS                  |
| producer_consumer_fp_deloxide_lock_order                  | Traditional FP | Lock Order |     10 |         0 |          0 |                 0 |         0.701  | ✅ PASS                  |
| producer_consumer_fp_deloxide_random_default              | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.7265 | ✅ PASS                  |
| producer_consumer_fp_no_deadlocks                         | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         4.7947 | ✅ PASS                  |
| producer_consumer_fp_parking_lot                          | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         0.5967 | ✅ PASS                  |
| read_dominated_fp_deloxide                                | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         2.0295 | ✅ PASS                  |
| read_dominated_fp_deloxide_aggressive                     | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         2.0622 | ✅ PASS                  |
| read_dominated_fp_deloxide_component_based_delays         | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         2.0685 | ✅ PASS                  |
| read_dominated_fp_deloxide_gentle                         | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         2.046  | ✅ PASS                  |
| read_dominated_fp_deloxide_lock_order                     | Traditional FP | Lock Order |     10 |         0 |          0 |                 0 |         2.0487 | ✅ PASS                  |
| read_dominated_fp_deloxide_random_default                 | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         2.0682 | ✅ PASS                  |
| read_dominated_fp_no_deadlocks                            | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        21.1244 | ✅ PASS                  |
| read_dominated_fp_parking_lot                             | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |         1.7951 | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide                        | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        25.5997 | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_aggressive             | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        26.6339 | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_component_based_delays | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        26.1363 | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_gentle                 | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        25.683  | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_lock_order             | Traditional FP | Lock Order |     10 |         0 |          0 |                 0 |        25.6736 | ✅ PASS                  |
| thread_local_hierarchy_fp_deloxide_random_default         | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        25.9105 | ✅ PASS                  |
| thread_local_hierarchy_fp_no_deadlocks                    | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |       303.949  | ✅ PASS                  |
| thread_local_hierarchy_fp_parking_lot                     | Traditional FP | Wait-For   |     10 |         0 |          0 |                 0 |        22.7894 | ✅ PASS                  |

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
  - Known FP (lock order detectors): 20
  - Unexpected FP (wait-for detectors): 0
- **Total false positives across all tests**: 20

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
