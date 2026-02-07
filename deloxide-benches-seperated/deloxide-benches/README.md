# deloxide-benches

This suite contains baseline performance benchmarks for `deloxide` synchronization primitives.

## Purpose

The primary goal of this suite is to measure the base overhead of `deloxide`'s synchronization primitives (Mutex, RwLock, Condvar) compared to standard library, `parking_lot`, and `no_deadlocks` baselines.

It serves as the control group for evaluating the performance cost of the deadlock detection mechanisms.

## Usage

To run these benchmarks:

```bash
cargo bench
```
