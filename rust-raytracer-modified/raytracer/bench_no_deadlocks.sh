#!/bin/bash
set -e

# Default to release
CARGO_FLAGS="--release"

# Build only the necessary features
echo "Building no_deadlocks configuration..."
cargo build $CARGO_FLAGS --no-default-features --features no_deadlocks
# Ensure generate_scene is built (it typically doesn't depend on these features but good to match)
cargo build $CARGO_FLAGS --bin generate_scene

mkdir -p bench_output

# specific resolutions to test (720p and 1080p)
WIDTHS=(1920)
HEIGHTS=(1080)

# CSV Output header for this run
echo "Configuration,Resolution,AvgTime(ms),StdDev(ms),Locks,AvgRSS(MB)" > bench_output/manual_nd_results.csv
echo "Running 10-iteration sweep for No Deadlocks..."

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
        TOTAL_TIME=0
        TOTAL_RSS=0
        LOCKS=0
        
        echo -n "  $NAME: "
        
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
        
        echo "$NAME,${RES_WIDTH}x${RES_HEIGHT},$AVG_TIME,$STD_DEV,$LOCKS,$AVG_RSS_MB" >> bench_output/manual_nd_results.csv
    }

    # Run ONLY No Deadlocks
    run_avg_bench "no_deadlocks" "No Deadlocks" "no_deadlocks"
done

echo "Manual Benchmark Suite Completed. Results in bench_output/manual_nd_results.csv"
