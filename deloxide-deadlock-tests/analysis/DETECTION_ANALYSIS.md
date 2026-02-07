# Detection Rate Analysis

**Type Legend**: `P` = Probabilistic (heisenbug), `G` = Guaranteed (barrier-synchronized)

## Summary

| Type   | Test                                                           |   Total Runs |   Detected |   Detection Rate (%) |   Timed Out |   Timeout Rate (%) |   Avg Runtime (ms) |
|:-------|:---------------------------------------------------------------|-------------:|-----------:|---------------------:|------------:|-------------------:|-------------------:|
| P      | dining_philosophers_deloxide                                   |          159 |         64 |                 40.3 |           0 |                0   |               5.88 |
| P      | dining_philosophers_deloxide_aggressive                        |          159 |        133 |                 83.6 |           0 |                0   |               7.87 |
| P      | dining_philosophers_deloxide_component_based_delays            |          159 |        157 |                 98.7 |           0 |                0   |               4.15 |
| P      | dining_philosophers_deloxide_gentle                            |          159 |         80 |                 50.3 |           0 |                0   |               5.09 |
| P      | dining_philosophers_deloxide_lock_order                        |          159 |        159 |                100   |           0 |                0   |               2.73 |
| P      | dining_philosophers_deloxide_random_default                    |          159 |        104 |                 65.4 |           0 |                0   |               5.82 |
| P      | dining_philosophers_no_deadlocks                               |          159 |        120 |                 75.5 |           0 |                0   |            2130.75 |
| P      | dining_philosophers_parking_lot                                |          159 |         86 |                 54.1 |           0 |                0   |               3.94 |
| P      | five_lock_cycle_deloxide                                       |          159 |        159 |                100   |           0 |                0   |               2.16 |
| P      | five_lock_cycle_deloxide_aggressive                            |          159 |        159 |                100   |           0 |                0   |               7.4  |
| P      | five_lock_cycle_deloxide_component_based_delays                |          159 |        159 |                100   |           0 |                0   |               4.36 |
| P      | five_lock_cycle_deloxide_gentle                                |          159 |        159 |                100   |           0 |                0   |               2.4  |
| P      | five_lock_cycle_deloxide_lock_order                            |          159 |        159 |                100   |           0 |                0   |               2.35 |
| P      | five_lock_cycle_deloxide_random_default                        |          159 |        159 |                100   |           0 |                0   |               4.35 |
| P      | five_lock_cycle_no_deadlocks                                   |          159 |        158 |                 99.4 |           1 |                0.6 |            2605.44 |
| P      | five_lock_cycle_parking_lot                                    |          159 |        159 |                100   |           0 |                0   |               2.13 |
| P      | rwlock_deadlock_deloxide                                       |          159 |         66 |                 41.5 |           0 |                0   |               3.14 |
| P      | rwlock_deadlock_deloxide_aggressive                            |          159 |        140 |                 88.1 |           0 |                0   |               6.05 |
| P      | rwlock_deadlock_deloxide_component_based_delays                |          159 |        159 |                100   |           0 |                0   |               3.94 |
| P      | rwlock_deadlock_deloxide_gentle                                |          159 |         85 |                 53.5 |           0 |                0   |               3.07 |
| P      | rwlock_deadlock_deloxide_lock_order                            |          159 |        159 |                100   |           0 |                0   |               2.36 |
| P      | rwlock_deadlock_deloxide_random_default                        |          159 |        113 |                 71.1 |           0 |                0   |               3.67 |
| P      | rwlock_deadlock_no_deadlocks                                   |          159 |        159 |                100   |           0 |                0   |            1077.85 |
| P      | rwlock_deadlock_parking_lot                                    |          159 |         96 |                 60.4 |           0 |                0   |               2.28 |
| P      | three_lock_cycle_deloxide                                      |          159 |        141 |                 88.7 |           0 |                0   |               2.44 |
| P      | three_lock_cycle_deloxide_aggressive                           |          159 |        158 |                 99.4 |           0 |                0   |               6.71 |
| P      | three_lock_cycle_deloxide_component_based_delays               |          159 |        159 |                100   |           0 |                0   |               4.03 |
| P      | three_lock_cycle_deloxide_gentle                               |          159 |        147 |                 92.5 |           0 |                0   |               2.49 |
| P      | three_lock_cycle_deloxide_lock_order                           |          159 |        159 |                100   |           0 |                0   |               2.4  |
| P      | three_lock_cycle_deloxide_random_default                       |          159 |        157 |                 98.7 |           0 |                0   |               3.9  |
| P      | three_lock_cycle_no_deadlocks                                  |          159 |        157 |                 98.7 |           0 |                0   |            1252.75 |
| P      | three_lock_cycle_parking_lot                                   |          159 |        123 |                 77.4 |           0 |                0   |               2.46 |
| P      | two_lock_deloxide                                              |          159 |         28 |                 17.6 |           0 |                0   |               3.53 |
| P      | two_lock_deloxide_aggressive                                   |          159 |        130 |                 81.8 |           0 |                0   |               6.44 |
| P      | two_lock_deloxide_component_based_delays                       |          159 |        157 |                 98.7 |           0 |                0   |               3.88 |
| P      | two_lock_deloxide_gentle                                       |          159 |         50 |                 31.4 |           0 |                0   |               2.98 |
| P      | two_lock_deloxide_lock_order                                   |          159 |        159 |                100   |           0 |                0   |               2.27 |
| P      | two_lock_deloxide_random_default                               |          159 |         91 |                 57.2 |           0 |                0   |               3.59 |
| P      | two_lock_no_deadlocks                                          |          159 |        117 |                 73.6 |           0 |                0   |             811.42 |
| P      | two_lock_parking_lot                                           |          159 |         40 |                 25.2 |           0 |                0   |               2.43 |
| G      | guaranteed_condvar_deadlock_deloxide                           |           10 |         10 |                100   |           0 |                0   |               2.47 |
| G      | guaranteed_condvar_deadlock_deloxide_aggressive                |           10 |         10 |                100   |           0 |                0   |               5.92 |
| G      | guaranteed_condvar_deadlock_deloxide_component_based_delays    |           10 |         10 |                100   |           0 |                0   |               2.94 |
| G      | guaranteed_condvar_deadlock_deloxide_gentle                    |           10 |         10 |                100   |           0 |                0   |               5.02 |
| G      | guaranteed_condvar_deadlock_deloxide_lock_order                |           10 |         10 |                100   |           0 |                0   |               2.72 |
| G      | guaranteed_condvar_deadlock_deloxide_random_default            |           10 |         10 |                100   |           0 |                0   |               3.71 |
| G      | guaranteed_condvar_deadlock_no_deadlocks                       |           10 |         10 |                100   |           0 |                0   |            1022.87 |
| G      | guaranteed_condvar_deadlock_parking_lot                        |           10 |         10 |                100   |           0 |                0   |               3.47 |
| G      | guaranteed_dining_philosophers_deloxide                        |           10 |         10 |                100   |           0 |                0   |               2.97 |
| G      | guaranteed_dining_philosophers_deloxide_aggressive             |           10 |         10 |                100   |           0 |                0   |               8.41 |
| G      | guaranteed_dining_philosophers_deloxide_component_based_delays |           10 |         10 |                100   |           0 |                0   |               6.58 |
| G      | guaranteed_dining_philosophers_deloxide_gentle                 |           10 |         10 |                100   |           0 |                0   |               3.11 |
| G      | guaranteed_dining_philosophers_deloxide_lock_order             |           10 |         10 |                100   |           0 |                0   |               2.14 |
| G      | guaranteed_dining_philosophers_deloxide_random_default         |           10 |         10 |                100   |           0 |                0   |               5.95 |
| G      | guaranteed_dining_philosophers_no_deadlocks                    |           10 |         10 |                100   |           0 |                0   |            1090.98 |
| G      | guaranteed_dining_philosophers_parking_lot                     |           10 |         10 |                100   |           0 |                0   |               2.99 |
| G      | guaranteed_rwlock_deadlock_deloxide                            |           10 |         10 |                100   |           0 |                0   |               3.05 |
| G      | guaranteed_rwlock_deadlock_deloxide_aggressive                 |           10 |         10 |                100   |           0 |                0   |               6.9  |
| G      | guaranteed_rwlock_deadlock_deloxide_component_based_delays     |           10 |         10 |                100   |           0 |                0   |               4.14 |
| G      | guaranteed_rwlock_deadlock_deloxide_gentle                     |           10 |         10 |                100   |           0 |                0   |               3.11 |
| G      | guaranteed_rwlock_deadlock_deloxide_lock_order                 |           10 |         10 |                100   |           0 |                0   |               5.44 |
| G      | guaranteed_rwlock_deadlock_deloxide_random_default             |           10 |         10 |                100   |           0 |                0   |               3.87 |
| G      | guaranteed_rwlock_deadlock_no_deadlocks                        |           10 |         10 |                100   |           0 |                0   |            1033.71 |
| G      | guaranteed_rwlock_deadlock_parking_lot                         |           10 |         10 |                100   |           0 |                0   |               2.32 |
| G      | guaranteed_three_lock_deloxide                                 |           10 |         10 |                100   |           0 |                0   |               1.53 |
| G      | guaranteed_three_lock_deloxide_aggressive                      |           10 |         10 |                100   |           0 |                0   |               8.34 |
| G      | guaranteed_three_lock_deloxide_component_based_delays          |           10 |         10 |                100   |           0 |                0   |               5.25 |
| G      | guaranteed_three_lock_deloxide_gentle                          |           10 |         10 |                100   |           0 |                0   |               3.02 |
| G      | guaranteed_three_lock_deloxide_lock_order                      |           10 |         10 |                100   |           0 |                0   |               3.01 |
| G      | guaranteed_three_lock_deloxide_random_default                  |           10 |         10 |                100   |           0 |                0   |               5.43 |
| G      | guaranteed_three_lock_no_deadlocks                             |           10 |         10 |                100   |           0 |                0   |            1080.41 |
| G      | guaranteed_three_lock_parking_lot                              |           10 |         10 |                100   |           0 |                0   |               3.14 |
| G      | guaranteed_two_lock_deloxide                                   |           10 |         10 |                100   |           0 |                0   |               2.3  |
| G      | guaranteed_two_lock_deloxide_aggressive                        |           10 |         10 |                100   |           0 |                0   |               5.82 |
| G      | guaranteed_two_lock_deloxide_component_based_delays            |           10 |         10 |                100   |           0 |                0   |               3.61 |
| G      | guaranteed_two_lock_deloxide_gentle                            |           10 |         10 |                100   |           0 |                0   |               1.73 |
| G      | guaranteed_two_lock_deloxide_lock_order                        |           10 |         10 |                100   |           0 |                0   |               3.6  |
| G      | guaranteed_two_lock_deloxide_random_default                    |           10 |         10 |                100   |           0 |                0   |               3.93 |
| G      | guaranteed_two_lock_no_deadlocks                               |           10 |         10 |                100   |           0 |                0   |            1054.46 |
| G      | guaranteed_two_lock_parking_lot                                |           10 |         10 |                100   |           0 |                0   |               3.48 |

## Key Findings

### Overall Statistics

- **Total test configurations**: 80
  - Probabilistic (heisenbug) tests: 40
  - Guaranteed deadlock tests: 40

- **Probabilistic tests average**: 80.6%
- **Guaranteed tests average**: 100.0%

### Performance Range

- **Best performer**: dining_philosophers_deloxide_lock_order (P) - 100.0%
- **Worst performer**: two_lock_deloxide (P) - 17.6%
