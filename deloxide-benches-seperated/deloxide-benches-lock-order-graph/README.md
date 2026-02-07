# deloxide-benches-lock-order-graph

This suite contains benchmarks for `deloxide` with the **Lock Order Graph** feature enabled.

## Purpose

These benchmarks measure the runtime overhead associated with maintaining the global lock dependency graph. When enabled, `deloxide` tracks lock acquisition orders to build and update a directed graph, which is essential for detecting deadlock cycles.

This suite runs the same scenarios as the baseline `deloxide-benches` to provide a direct comparison of the performance cost of graph maintenance.

## Content

*   **Lock Order Check Overhead**: Measures the cost of checking and updating the graph on every lock acquisition.
*   **Scalability**: Assesses how the graph maintenance scales with increasing number of locks and threads.

## Usage

To run these benchmarks:

```bash
cargo bench
```
