# deloxide-benches-stress-test

This suite benchmarks `deloxide` with the **Stress Test** feature enabled.

## Purpose

The Stress Test feature instruments synchronization primitives with random delays, scheduler perturbations, and thread yielding to increase the likelihood of exposing concurrency bugs (e.g., race conditions, deadlocks).

These benchmarks quantify the performance impact of this instrumentation. It helps understand how much slower the application runs when aggressive stress testing is active.

## Content

The suite includes tests for different stress components and configurations:

*   **Component Benchmarks**: Isolates specific stress mechanisms.
*   **Randomized Stress**: Varies the intensity of the stress testing (Gentle, Default, Aggressive) to show the trade-off between bug detection probability and performance.

## Usage

To run these benchmarks:

```bash
cargo bench
```
