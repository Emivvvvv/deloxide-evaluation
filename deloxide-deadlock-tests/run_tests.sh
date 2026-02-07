#!/bin/bash
# Comprehensive test runner for deadlock detection benchmarks
# Usage: ./run_tests.sh [command] [options]

set -e

# Generate a unique seed for Heisenbug tests
generate_seed() {
    # Check if date supports nanoseconds (Linux)
    local nano_test=$(date +%N 2>/dev/null)
    if [ "$nano_test" != "N" ] && [ -n "$nano_test" ]; then
        echo $(date +%s%N)
    else
        # macOS fallback: use seconds + random numbers for uniqueness
        echo $(($(date +%s) * 1000000 + RANDOM * 1000 + RANDOM))
    fi
}

FEATURES=("deloxide" "deloxide_random_default" "deloxide_component_based_delays" "deloxide_aggressive" "deloxide_gentle" "deloxide_lock_order" "parking_lot_deadlock" "no_deadlocks")
DELOXIDE_FEATURES=("deloxide" "deloxide_random_default" "deloxide_component_based_delays" "deloxide_aggressive" "deloxide_gentle" "deloxide_lock_order")
DETECTION_TESTS=("two_lock" "three_lock_cycle" "dining_philosophers" "rwlock_deadlock" "five_lock_cycle")
FALSE_POSITIVE_TESTS=("four_hier_fp" "gate_guarded_fp" "producer_consumer_fp" "lock_free_interval_fp" "conditional_locking_fp" "read_dominated_fp" "thread_local_hierarchy_fp" "lock_order_inversion_fp" "complex_lock_order_fp")
GUARANTEED_TESTS=("guaranteed_two_lock" "guaranteed_three_lock" "guaranteed_dining_philosophers" "guaranteed_rwlock_deadlock" "guaranteed_condvar_deadlock")

show_help() {
    cat << EOF
Deadlock Detection Test Runner

USAGE:
    ./run_tests.sh [COMMAND] [OPTIONS]

COMMANDS:
    all                     Run all tests (detection + false-positive + guaranteed)
    detection               Run all deadlock detection tests
    false-positive          Run all false positive tests
    guaranteed              Run all guaranteed deadlock tests
    
    single TEST FEATURE     Run a single test with specific feature
    test-all-features TEST  Run a single test with all features
    
    list                    List all available tests
    list-features           List all available features
    help                    Show this help message

OPTIONS:
    -n, --iterations N      Number of iterations (default: 1)
    -q, --quiet             Suppress build output
    -f, --features LIST     Comma-separated list of features to run (default: all)
    -D, --deloxide-all      Shortcut to run every Deloxide-based feature

EXAMPLES:
    # Run all tests once
    ./run_tests.sh all

    # Run detection tests 10 times
    ./run_tests.sh detection -n 10

    # Run with specific features only
    ./run_tests.sh detection -f deloxide,parking_lot_deadlock
    ./run_tests.sh all -f deloxide,deloxide_random_default,deloxide_aggressive

    # Run all deloxide variants + parking_lot (exclude no_deadlocks)
    ./run_tests.sh detection -f deloxide,deloxide_random_default,deloxide_component_based_delays,parking_lot_deadlock

    # Run single test with specific feature
    ./run_tests.sh single two_lock deloxide

    # Run single test with all features
    ./run_tests.sh test-all-features two_lock

    # Run with custom features
    ./run_tests.sh test-all-features two_lock -f deloxide,parking_lot_deadlock

    # Run false positive tests quietly with specific features
    ./run_tests.sh false-positive --quiet -f deloxide,parking_lot_deadlock

FEATURES:
    - deloxide
    - deloxide_random_default
    - deloxide_component_based_delays
    - deloxide_aggressive
    - deloxide_gentle
    - deloxide_lock_order
    - parking_lot_deadlock
    - no_deadlocks

EOF
}

list_tests() {
    echo "DEADLOCK DETECTION TESTS:"
    for test in "${DETECTION_TESTS[@]}"; do
        echo "  - $test"
    done
    echo ""
    echo "FALSE POSITIVE TESTS:"
    for test in "${FALSE_POSITIVE_TESTS[@]}"; do
        echo "  - $test"
    done
    echo ""
    echo "GUARANTEED DEADLOCK TESTS:"
    for test in "${GUARANTEED_TESTS[@]}"; do
        echo "  - $test"
    done
}

list_features() {
    echo "AVAILABLE FEATURES:"
    for feature in "${FEATURES[@]}"; do
        echo "  - $feature"
    done
    echo ""
    echo "FEATURE GROUPS:"
    echo "  All Deloxide variants (--deloxide-all):"
    echo "    ${DELOXIDE_FEATURES[*]}"
    echo ""
    echo "  All Stress Test variants:"
    echo "    deloxide_random_default,deloxide_aggressive,deloxide_gentle,deloxide_component_based_delays"
    echo ""
    echo "  Deloxide + parking_lot:"
    echo "    deloxide,deloxide_random_default,deloxide_component_based_delays,parking_lot_deadlock"
    echo ""
    echo "  Just basic detectors:"
    echo "    deloxide,parking_lot_deadlock,no_deadlocks"
}

run_single_test() {
    local test=$1
    local feature=$2
    local quiet=$3
    
    if [ "$quiet" = "true" ]; then
        HEISENBUG_SEED=$HEISENBUG_SEED cargo run --release --bin "$test" --no-default-features --features "$feature" --quiet 2>/dev/null || true
    else
        HEISENBUG_SEED=$HEISENBUG_SEED cargo run --release --bin "$test" --no-default-features --features "$feature"
    fi
}

run_test_all_features() {
    local test=$1
    local iterations=$2
    local quiet=$3
    
    echo "=========================================="
    echo "Running $test with selected features"
    echo "Features: ${SELECTED_FEATURES[*]}"
    echo "Iterations: $iterations"
    echo "=========================================="
    echo ""
    
    for ((i=1; i<=iterations; i++)); do
        # Generate a new seed for each iteration
        HEISENBUG_SEED=$(generate_seed)
        
        if [ $iterations -gt 1 ]; then
            echo "==== Iteration $i/$iterations (Heisenbug seed: $HEISENBUG_SEED) ===="
        fi
        for feature in "${SELECTED_FEATURES[@]}"; do
            run_single_test "$test" "$feature" "$quiet"
        done
    done
}

run_detection_tests() {
    local iterations=$1
    local quiet=$2
    
    echo "=========================================="
    echo "DEADLOCK DETECTION TESTS"
    echo "Features: ${SELECTED_FEATURES[*]}"
    echo "=========================================="
    echo ""
    
    for ((i=1; i<=iterations; i++)); do
        # Generate a new seed for each iteration
        HEISENBUG_SEED=$(generate_seed)
        
        if [ $iterations -gt 1 ]; then
            echo "==== Iteration $i/$iterations (Heisenbug seed: $HEISENBUG_SEED) ===="
        fi
        for test in "${DETECTION_TESTS[@]}"; do
            for feature in "${SELECTED_FEATURES[@]}"; do
                run_single_test "$test" "$feature" "$quiet"
            done
        done
    done
}

run_false_positive_tests() {
    local iterations=$1
    local quiet=$2
    
    echo "=========================================="
    echo "FALSE POSITIVE TESTS"
    echo "Features: ${SELECTED_FEATURES[*]}"
    echo "=========================================="
    echo ""
    
    for ((i=1; i<=iterations; i++)); do
        if [ $iterations -gt 1 ]; then
            echo "==== Iteration $i/$iterations ===="
        fi
        for test in "${FALSE_POSITIVE_TESTS[@]}"; do
            for feature in "${SELECTED_FEATURES[@]}"; do
                run_single_test "$test" "$feature" "$quiet"
            done
        done
    done
}

run_guaranteed_tests() {
    local iterations=$1
    local quiet=$2
    
    echo "=========================================="
    echo "GUARANTEED DEADLOCK TESTS"
    echo "Features: ${SELECTED_FEATURES[*]}"
    echo "=========================================="
    echo ""
    
    for ((i=1; i<=iterations; i++)); do
        if [ $iterations -gt 1 ]; then
            echo "==== Iteration $i/$iterations ===="
        fi
        for test in "${GUARANTEED_TESTS[@]}"; do
            for feature in "${SELECTED_FEATURES[@]}"; do
                run_single_test "$test" "$feature" "$quiet"
            done
        done
    done
}

run_all_tests() {
    local iterations=$1
    local quiet=$2
    
    echo "=========================================="
    echo "RUNNING ALL TESTS"
    echo "Features: ${SELECTED_FEATURES[*]}"
    echo "=========================================="
    echo ""
    
    run_detection_tests "$iterations" "$quiet"
    echo ""
    run_false_positive_tests "$iterations" "$quiet"
    echo ""
    run_guaranteed_tests "$iterations" "$quiet"
}

# Parse arguments
COMMAND=${1:-help}
shift || true

ITERATIONS=1
QUIET=false
SELECTED_FEATURES=()

while [[ $# -gt 0 ]]; do
    case $1 in
        -n|--iterations)
            ITERATIONS="$2"
            shift 2
            ;;
        -q|--quiet)
            QUIET=true
            shift
            ;;
        -f|--features)
            IFS=',' read -ra SELECTED_FEATURES <<< "$2"
            shift 2
            ;;
        -D|--deloxide-all)
            SELECTED_FEATURES=("${DELOXIDE_FEATURES[@]}")
            shift
            ;;
        *)
            EXTRA_ARGS+=("$1")
            shift
            ;;
    esac
done

# If no features specified, use all features
if [ ${#SELECTED_FEATURES[@]} -eq 0 ]; then
    SELECTED_FEATURES=("${FEATURES[@]}")
fi

# Execute command
case $COMMAND in
    all)
        run_all_tests "$ITERATIONS" "$QUIET"
        ;;
    detection)
        run_detection_tests "$ITERATIONS" "$QUIET"
        ;;
    false-positive)
        run_false_positive_tests "$ITERATIONS" "$QUIET"
        ;;
    guaranteed)
        run_guaranteed_tests "$ITERATIONS" "$QUIET"
        ;;
    single)
        if [ ${#EXTRA_ARGS[@]} -lt 2 ]; then
            echo "Error: 'single' requires TEST and FEATURE arguments"
            echo "Usage: ./run_tests.sh single TEST FEATURE"
            exit 1
        fi
        run_single_test "${EXTRA_ARGS[0]}" "${EXTRA_ARGS[1]}" "$QUIET"
        ;;
    test-all-features)
        if [ ${#EXTRA_ARGS[@]} -lt 1 ]; then
            echo "Error: 'test-all-features' requires TEST argument"
            echo "Usage: ./run_tests.sh test-all-features TEST [-f FEATURES]"
            exit 1
        fi
        run_test_all_features "${EXTRA_ARGS[0]}" "$ITERATIONS" "$QUIET"
        ;;
    list)
        list_tests
        ;;
    list-features)
        list_features
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo "Unknown command: $COMMAND"
        echo "Run './run_tests.sh help' for usage information"
        exit 1
        ;;
esac
