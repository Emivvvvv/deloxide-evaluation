# deloxide-evaluation

The benchmarks and tests for the evaluation of the `deloxide` deadlock detection tool.

## Evaluation Suites

This repository contains 5 distinct suites for evaluating different aspects of the tool:

1.  [deloxide-benches](deloxide-benches-seperated/deloxide-benches)
    *   **Baseline Performance**: Measures standard synchronization primitive overhead without deadlock detection.

2.  [deloxide-benches-lock-order-graph](deloxide-benches-seperated/deloxide-benches-lock-order-graph)
    *   **Graph Maintenance Overhead**: Benchmarks with the lock order graph feature enabled to measure detection cost.

3.  [deloxide-benches-stress-test](deloxide-benches-seperated/deloxide-benches-stress-test)
    *   **Stress Testing Impact**: Measures the performance impact of aggressive stress testing instrumentation.

4.  [deloxide-deadlock-tests](deloxide-deadlock-tests)
    *   **Correctness Verified**: A suite of True Positive (deadlocks), False Positive (valid patterns), and Guaranteed Deadlock tests.

5.  [rust-raytracer-modified](rust-raytracer-modified)
    *   **Real-World Application**: A modified ray tracer acting as a macro-benchmark for parallel workloads.

