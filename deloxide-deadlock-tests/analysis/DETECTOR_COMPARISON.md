# Detector Comparison

**Note**: Detection rates exclude timeouts. Only runs that completed (either with detection or without) are counted.

## Detection Rates by Test Scenario

|                                | DX     | DX+LO   | DX-Rand   | DX-Agg   | DX-Gent   | DX-Comp   | PL     | ND     |
|:-------------------------------|:-------|:--------|:----------|:---------|:----------|:----------|:-------|:-------|
| dining_philosophers            | 40.3%  | 100.0%  | 65.4%     | 83.6%    | 50.3%     | 98.7%     | 54.1%  | 75.5%  |
| five_lock_cycle                | 100.0% | 100.0%  | 100.0%    | 100.0%   | 100.0%    | 100.0%    | 100.0% | 100.0% |
| rwlock_deadlock                | 41.5%  | 100.0%  | 71.1%     | 88.1%    | 53.5%     | 100.0%    | 60.4%  | 100.0% |
| three_lock_cycle               | 88.7%  | 100.0%  | 98.7%     | 99.4%    | 92.5%     | 100.0%    | 77.4%  | 98.7%  |
| two_lock                       | 17.6%  | 100.0%  | 57.2%     | 81.8%    | 31.4%     | 98.7%     | 25.2%  | 73.6%  |
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

- **DX**: Average detection rate = 78.8%
- **DX+LO**: Average detection rate = 100.0%
- **DX-Rand**: Average detection rate = 89.2%
- **DX-Agg**: Average detection rate = 95.3%
- **DX-Gent**: Average detection rate = 82.8%
- **DX-Comp**: Average detection rate = 99.7%
- **PL**: Average detection rate = 81.7%
- **ND**: Average detection rate = 94.8%

## Recommendations

**Best overall detector**: DX+LO (100.0% average)

- `Deloxide + Aggressive Stress` shows 95.3% detection rate
- `Deloxide (vanilla)` shows 78.8% detection rate

💡 For production use, balance detection rate with performance overhead.
