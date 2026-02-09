# Detection Rate Analysis

**Type Legend**: `P` = Probabilistic (heisenbug), `G` = Guaranteed (barrier-synchronized)

## Summary

| Type   | Test                                                           |   Total Runs |   Detected |   Detection Rate (%) |   Timed Out |   Timeout Rate (%) |   Avg Runtime (ms) |
|:-------|:---------------------------------------------------------------|-------------:|-----------:|---------------------:|------------:|-------------------:|-------------------:|
| P      | dining_philosophers_deloxide                                   |          200 |         78 |                 39   |           0 |                0   |               5.76 |
| P      | dining_philosophers_deloxide_aggressive                        |          200 |        168 |                 84   |           0 |                0   |               7.78 |
| P      | dining_philosophers_deloxide_component_based_delays            |          200 |        198 |                 99   |           0 |                0   |               3.96 |
| P      | dining_philosophers_deloxide_gentle                            |          200 |        103 |                 51.5 |           0 |                0   |               4.88 |
| P      | dining_philosophers_deloxide_lock_order                        |          200 |        200 |                100   |           0 |                0   |               2.49 |
| P      | dining_philosophers_deloxide_random_default                    |          200 |        137 |                 68.5 |           1 |                0.5 |              55.39 |
| P      | dining_philosophers_no_deadlocks                               |          200 |        150 |                 75   |           0 |                0   |            2220.62 |
| P      | dining_philosophers_parking_lot                                |          200 |        102 |                 51   |           0 |                0   |               4.08 |
| P      | five_lock_cycle_deloxide                                       |          200 |        200 |                100   |           0 |                0   |               2.04 |
| P      | five_lock_cycle_deloxide_aggressive                            |          200 |        200 |                100   |           0 |                0   |               7.31 |
| P      | five_lock_cycle_deloxide_component_based_delays                |          200 |        200 |                100   |           0 |                0   |               4.2  |
| P      | five_lock_cycle_deloxide_gentle                                |          200 |        200 |                100   |           0 |                0   |               2.23 |
| P      | five_lock_cycle_deloxide_lock_order                            |          200 |        200 |                100   |           0 |                0   |               2.19 |
| P      | five_lock_cycle_deloxide_random_default                        |          200 |        200 |                100   |           0 |                0   |               4.18 |
| P      | five_lock_cycle_no_deadlocks                                   |          200 |        198 |                 99   |           2 |                1   |            2677.96 |
| P      | five_lock_cycle_parking_lot                                    |          200 |        200 |                100   |           0 |                0   |               2.1  |
| P      | rwlock_deadlock_deloxide                                       |          200 |         78 |                 39   |           0 |                0   |               3.01 |
| P      | rwlock_deadlock_deloxide_aggressive                            |          200 |        171 |                 85.5 |           0 |                0   |               5.79 |
| P      | rwlock_deadlock_deloxide_component_based_delays                |          200 |        200 |                100   |           0 |                0   |               3.72 |
| P      | rwlock_deadlock_deloxide_gentle                                |          200 |        108 |                 54   |           0 |                0   |               2.87 |
| P      | rwlock_deadlock_deloxide_lock_order                            |          200 |        200 |                100   |           0 |                0   |               2.19 |
| P      | rwlock_deadlock_deloxide_random_default                        |          200 |        141 |                 70.5 |           0 |                0   |               3.5  |
| P      | rwlock_deadlock_no_deadlocks                                   |          200 |        200 |                100   |           0 |                0   |            1079.41 |
| P      | rwlock_deadlock_parking_lot                                    |          200 |        115 |                 57.5 |           0 |                0   |               2.28 |
| P      | three_lock_cycle_deloxide                                      |          200 |        176 |                 88   |           0 |                0   |               2.32 |
| P      | three_lock_cycle_deloxide_aggressive                           |          200 |        199 |                 99.5 |           0 |                0   |               6.58 |
| P      | three_lock_cycle_deloxide_component_based_delays               |          200 |        200 |                100   |           0 |                0   |               3.82 |
| P      | three_lock_cycle_deloxide_gentle                               |          200 |        186 |                 93   |           0 |                0   |               2.31 |
| P      | three_lock_cycle_deloxide_lock_order                           |          200 |        200 |                100   |           0 |                0   |               2.22 |
| P      | three_lock_cycle_deloxide_random_default                       |          200 |        198 |                 99   |           0 |                0   |               3.73 |
| P      | three_lock_cycle_no_deadlocks                                  |          200 |        198 |                 99   |           0 |                0   |            1279.68 |
| P      | three_lock_cycle_parking_lot                                   |          200 |        156 |                 78   |           0 |                0   |               2.43 |
| P      | two_lock_deloxide                                              |          200 |         33 |                 16.5 |           0 |                0   |               3.36 |
| P      | two_lock_deloxide_aggressive                                   |          200 |        165 |                 82.5 |           0 |                0   |               6.19 |
| P      | two_lock_deloxide_component_based_delays                       |          200 |        198 |                 99   |           0 |                0   |               3.73 |
| P      | two_lock_deloxide_gentle                                       |          200 |         65 |                 32.5 |           0 |                0   |               2.85 |
| P      | two_lock_deloxide_lock_order                                   |          200 |        200 |                100   |           0 |                0   |               2.11 |
| P      | two_lock_deloxide_random_default                               |          200 |        118 |                 59   |           0 |                0   |               3.41 |
| P      | two_lock_no_deadlocks                                          |          200 |        144 |                 72   |           0 |                0   |             795.56 |
| P      | two_lock_parking_lot                                           |          200 |         52 |                 26   |           0 |                0   |               2.4  |
| G      | guaranteed_condvar_deadlock_deloxide                           |           20 |         20 |                100   |           0 |                0   |               1.93 |
| G      | guaranteed_condvar_deadlock_deloxide_aggressive                |           20 |         20 |                100   |           0 |                0   |               5.58 |
| G      | guaranteed_condvar_deadlock_deloxide_component_based_delays    |           20 |         20 |                100   |           0 |                0   |               2.76 |
| G      | guaranteed_condvar_deadlock_deloxide_gentle                    |           20 |         20 |                100   |           0 |                0   |               3.21 |
| G      | guaranteed_condvar_deadlock_deloxide_lock_order                |           20 |         20 |                100   |           0 |                0   |               2.06 |
| G      | guaranteed_condvar_deadlock_deloxide_random_default            |           20 |         20 |                100   |           0 |                0   |               2.88 |
| G      | guaranteed_condvar_deadlock_no_deadlocks                       |           20 |         20 |                100   |           0 |                0   |            1128.54 |
| G      | guaranteed_condvar_deadlock_parking_lot                        |           20 |         20 |                100   |           0 |                0   |               2.41 |
| G      | guaranteed_dining_philosophers_deloxide                        |           20 |         20 |                100   |           0 |                0   |               2.17 |
| G      | guaranteed_dining_philosophers_deloxide_aggressive             |           20 |         20 |                100   |           0 |                0   |               7.56 |
| G      | guaranteed_dining_philosophers_deloxide_component_based_delays |           20 |         20 |                100   |           0 |                0   |               4.79 |
| G      | guaranteed_dining_philosophers_deloxide_gentle                 |           20 |         20 |                100   |           0 |                0   |               2.28 |
| G      | guaranteed_dining_philosophers_deloxide_lock_order             |           20 |         20 |                100   |           0 |                0   |               1.8  |
| G      | guaranteed_dining_philosophers_deloxide_random_default         |           20 |         20 |                100   |           0 |                0   |               4.58 |
| G      | guaranteed_dining_philosophers_no_deadlocks                    |           20 |         20 |                100   |           0 |                0   |            1562.27 |
| G      | guaranteed_dining_philosophers_parking_lot                     |           20 |         20 |                100   |           0 |                0   |               2.33 |
| G      | guaranteed_rwlock_deadlock_deloxide                            |           20 |         20 |                100   |           0 |                0   |               2.24 |
| G      | guaranteed_rwlock_deadlock_deloxide_aggressive                 |           20 |         20 |                100   |           0 |                0   |               5.44 |
| G      | guaranteed_rwlock_deadlock_deloxide_component_based_delays     |           20 |         20 |                100   |           0 |                0   |               3.37 |
| G      | guaranteed_rwlock_deadlock_deloxide_gentle                     |           20 |         20 |                100   |           0 |                0   |               2.25 |
| G      | guaranteed_rwlock_deadlock_deloxide_lock_order                 |           20 |         20 |                100   |           0 |                0   |               3.42 |
| G      | guaranteed_rwlock_deadlock_deloxide_random_default             |           20 |         20 |                100   |           0 |                0   |               3.16 |
| G      | guaranteed_rwlock_deadlock_no_deadlocks                        |           20 |         20 |                100   |           0 |                0   |            1076.67 |
| G      | guaranteed_rwlock_deadlock_parking_lot                         |           20 |         20 |                100   |           0 |                0   |               1.84 |
| G      | guaranteed_three_lock_deloxide                                 |           20 |         20 |                100   |           0 |                0   |               1.45 |
| G      | guaranteed_three_lock_deloxide_aggressive                      |           20 |         20 |                100   |           0 |                0   |               6.89 |
| G      | guaranteed_three_lock_deloxide_component_based_delays          |           20 |         20 |                100   |           0 |                0   |               3.81 |
| G      | guaranteed_three_lock_deloxide_gentle                          |           20 |         20 |                100   |           0 |                0   |               2.21 |
| G      | guaranteed_three_lock_deloxide_lock_order                      |           20 |         20 |                100   |           0 |                0   |               2.22 |
| G      | guaranteed_three_lock_deloxide_random_default                  |           20 |         20 |                100   |           0 |                0   |               3.99 |
| G      | guaranteed_three_lock_no_deadlocks                             |           20 |         20 |                100   |           0 |                0   |            1232.94 |
| G      | guaranteed_three_lock_parking_lot                              |           20 |         20 |                100   |           0 |                0   |               2.25 |
| G      | guaranteed_two_lock_deloxide                                   |           20 |         20 |                100   |           0 |                0   |               1.84 |
| G      | guaranteed_two_lock_deloxide_aggressive                        |           20 |         20 |                100   |           0 |                0   |               5.64 |
| G      | guaranteed_two_lock_deloxide_component_based_delays            |           20 |         20 |                100   |           0 |                0   |               3.01 |
| G      | guaranteed_two_lock_deloxide_gentle                            |           20 |         20 |                100   |           0 |                0   |               1.55 |
| G      | guaranteed_two_lock_deloxide_lock_order                        |           20 |         20 |                100   |           0 |                0   |               2.49 |
| G      | guaranteed_two_lock_deloxide_random_default                    |           20 |         20 |                100   |           0 |                0   |               3.25 |
| G      | guaranteed_two_lock_no_deadlocks                               |           20 |         20 |                100   |           0 |                0   |            1052.26 |
| G      | guaranteed_two_lock_parking_lot                                |           20 |         20 |                100   |           0 |                0   |               2.4  |

## Key Findings

### Overall Statistics

- **Total test configurations**: 80
  - Probabilistic (heisenbug) tests: 40
  - Guaranteed deadlock tests: 40

- **Probabilistic tests average**: 80.4%
- **Guaranteed tests average**: 100.0%

### Performance Range

- **Best performer**: dining_philosophers_deloxide_lock_order (P) - 100.0%
- **Worst performer**: two_lock_deloxide (P) - 16.5%
