# Detector Comparison

**Note**: Detection rates exclude timeouts. Only runs that completed (either with detection or without) are counted.

## Detection Rates by Test Scenario

|                                | DX     | DX+LO   | DX-Rand   | DX-Agg   | DX-Gent   | DX-Comp   | PL     | ND     |
|:-------------------------------|:-------|:--------|:----------|:---------|:----------|:----------|:-------|:-------|
| dining_philosophers            | 39.0%  | 100.0%  | 68.8%     | 84.0%    | 51.5%     | 99.0%     | 51.0%  | 75.0%  |
| five_lock_cycle                | 100.0% | 100.0%  | 100.0%    | 100.0%   | 100.0%    | 100.0%    | 100.0% | 100.0% |
| rwlock_deadlock                | 39.0%  | 100.0%  | 70.5%     | 85.5%    | 54.0%     | 100.0%    | 57.5%  | 100.0% |
| three_lock_cycle               | 88.0%  | 100.0%  | 99.0%     | 99.5%    | 93.0%     | 100.0%    | 78.0%  | 99.0%  |
| two_lock                       | 16.5%  | 100.0%  | 59.0%     | 82.5%    | 32.5%     | 99.0%     | 26.0%  | 72.0%  |
| guaranteed_condvar_deadlock    | 100.0% | 100.0%  | 100.0%    | 100.0%   | 100.0%    | 100.0%    | 100.0% | 100.0% |
| guaranteed_dining_philosophers | 100.0% | 100.0%  | 100.0%    | 100.0%   | 100.0%    | 100.0%    | 100.0% | 100.0% |
| guaranteed_rwlock_deadlock     | 100.0% | 100.0%  | 100.0%    | 100.0%   | 100.0%    | 100.0%    | 100.0% | 100.0% |
| guaranteed_three_lock          | 100.0% | 100.0%  | 100.0%    | 100.0%   | 100.0%    | 100.0%    | 100.0% | 100.0% |
| guaranteed_two_lock            | 100.0% | 100.0%  | 100.0%    | 100.0%   | 100.0%    | 100.0%    | 100.0% | 100.0% |

## Analysis

### Best Detector by Scenario

- **dining_philosophers**: DX+LO (100.0%)
- **five_lock_cycle**: DX (100.0%)
- **guaranteed_condvar_deadlock**: DX (100.0%)
- **guaranteed_dining_philosophers**: DX (100.0%)
- **guaranteed_rwlock_deadlock**: DX (100.0%)
- **guaranteed_three_lock**: DX (100.0%)
- **guaranteed_two_lock**: DX (100.0%)
- **rwlock_deadlock**: DX+LO (100.0%)
- **three_lock_cycle**: DX+LO (100.0%)
- **two_lock**: DX+LO (100.0%)

### Overall Statistics

- **DX**: Average detection rate = 78.2%
- **DX+LO**: Average detection rate = 100.0%
- **DX-Rand**: Average detection rate = 89.7%
- **DX-Agg**: Average detection rate = 95.2%
- **DX-Gent**: Average detection rate = 83.1%
- **DX-Comp**: Average detection rate = 99.8%
- **PL**: Average detection rate = 81.2%
- **ND**: Average detection rate = 94.6%

## Recommendations

**Best overall detector**: DX+LO (100.0% average)

- `Deloxide + Aggressive Stress` shows 95.2% detection rate
- `Deloxide (vanilla)` shows 78.2% detection rate

💡 For production use, balance detection rate with performance overhead.
