#!/bin/bash
set -e

# Default to release
CARGO_FLAGS="--release"

# We will build inside the loop to ensure the correct features are active for the binary

mkdir -p bench_output

# Resolutions to test
WIDTHS=(426 854 1280 1920)
HEIGHTS=(240 480 720 1080)

# CSV Output header
echo "Configuration,Resolution,AvgTime(ms),StdDev(ms),Locks,AvgRSS(MB)" > bench_output/final_results.csv
echo "Running 10-iteration sweep..."

for i in "${!WIDTHS[@]}"; do
    RES_WIDTH=${WIDTHS[$i]}
    RES_HEIGHT=${HEIGHTS[$i]}
    SCENE_FILE="/tmp/scene_${RES_WIDTH}x${RES_HEIGHT}.json"
    
    # Generate Scene File (128 spp for quality/load)
    cargo run --release --bin generate_scene -- $RES_WIDTH $RES_HEIGHT 128 50 > $SCENE_FILE

    echo "=== Benchmarking ${RES_WIDTH}x${RES_HEIGHT} ==="

    # Define run function with 10x loop
    run_avg_bench() {
        FEATURE=$1
        NAME=$2
        SLUG=$3
        OUTPUT_IMG="bench_output/${SLUG}_${RES_WIDTH}.png"
        
        TIMES=()
        TOTAL_RSS=0
        LOCKS=0
        
        echo -n "  $NAME: "
        
        # Build the specific feature once
        cargo build $CARGO_FLAGS --no-default-features --features $FEATURE --quiet
        
        for iter in {1..10}; do
             # Use /usr/bin/time -l to capture output of the BINARY directly
             OUTPUT=$(/usr/bin/time -l ./target/release/raytracer "$SCENE_FILE" "$OUTPUT_IMG" 2>&1)
             
             TIME_MS=$(echo "$OUTPUT" | grep "Frame time:" | grep -o "[0-9]*" | head -1)
             LOCKS_RUN=$(echo "$OUTPUT" | grep "Total Lock Acquisitions:" | grep -o "[0-9]*" | head -1)
             # Handle potential missing output if run fails, but set -e should catch it
             RSS=$(echo "$OUTPUT" | grep "maximum resident set size" | grep -o "[0-9]*" | head -1)
             
             # Store time in array
             TIMES+=($TIME_MS)
             
             let "TOTAL_TIME += TIME_MS"
             let "TOTAL_RSS += RSS"
             LOCKS=$LOCKS_RUN
             
             echo -n "."
        done
        
        # Calculate Statistics using awk for floating point precision and SD
        # Pass the array elements to awk
        STATS=$(echo "${TIMES[@]}" | awk '{
            sum = 0;
            sumsq = 0;
            for (i = 1; i <= NF; i++) {
                sum += $i;
                sumsq += ($i * $i);
            }
            mean = sum / NF;
            if (NF > 1) {
                variance = (sumsq - (sum * sum / NF)) / (NF - 1);
                sd = sqrt(variance);
            } else {
                sd = 0;
            }
            printf "%.2f %.2f", mean, sd;
        }')
        
        # Read into variables
        read AVG_TIME STD_DEV <<< "$STATS"
        
        let "AVG_RSS_BYTES = TOTAL_RSS / 10"
        let "AVG_RSS_MB = AVG_RSS_BYTES / 1024 / 1024"
        
        echo " Done. Avg: ${AVG_TIME}ms +/- ${STD_DEV}ms, Locks: $LOCKS, RSS: ${AVG_RSS_MB}MB"
        
        echo "$NAME,${RES_WIDTH}x${RES_HEIGHT},$AVG_TIME,$STD_DEV,$LOCKS,$AVG_RSS_MB" >> bench_output/final_results.csv
    }

    run_avg_bench "use_std" "std::sync" "std_sync"
    run_avg_bench "parking_lot_deadlock" "parking_lot" "parking_lot"
    run_avg_bench "deloxide" "Deloxide (Default)" "deloxide_default"
    run_avg_bench "deloxide_lock_order" "Deloxide (Graph)" "deloxide_graph"
    # Stress variants logic implied? Usually sweeping average is for the base impls, but 
    # original script had stress variants too. I will keep them.
    run_avg_bench "stress_gentle" "Stress (Gentle)" "stress_gentle"
    run_avg_bench "stress_random" "Stress (Random)" "stress_random"
    run_avg_bench "stress_aggressive" "Stress (Aggressive)" "stress_aggressive"
    run_avg_bench "stress_component" "Stress (Component)" "stress_component"

    # Conditional no_deadlocks only for heights <= 480
    if [ "$RES_HEIGHT" -le 480 ]; then
        run_avg_bench "no_deadlocks" "No Deadlocks" "no_deadlocks"
    fi
done

echo "Benchmark Suite Completed."
