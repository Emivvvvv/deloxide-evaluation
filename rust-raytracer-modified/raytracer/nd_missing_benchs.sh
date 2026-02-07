#!/bin/bash
set -e

CARGO_FLAGS="--release"
mkdir -p bench_output

# The missing resolutions for No Deadlocks (ND)
WIDTHS=(1280 1920)
HEIGHTS=(720 1080)

# Create the specific CSV for these long runs
CSV_FILE="bench_output/nd_long_runs.csv"
echo "Configuration,Resolution,AvgTime(ms),StdDev(ms),Locks,AvgRSS(MB)" > "$CSV_FILE"

echo "Starting ND Long Run Benchmark..."

for i in "${!WIDTHS[@]}"; do
    RES_WIDTH=${WIDTHS[$i]}
    RES_HEIGHT=${HEIGHTS[$i]}
    SCENE_FILE="/tmp/scene_${RES_WIDTH}x${RES_HEIGHT}.json"
    
    # 1. Ensure Scene File exists (Generate with same settings as original)
    echo "Generating scene for ${RES_WIDTH}x${RES_HEIGHT}..."
    cargo run --release --bin generate_scene -- $RES_WIDTH $RES_HEIGHT 128 50 > $SCENE_FILE

    echo "=== Benchmarking ND (No Deadlocks) at ${RES_WIDTH}x${RES_HEIGHT} ==="

    FEATURE="no_deadlocks"
    NAME="No Deadlocks"
    SLUG="no_deadlocks"
    OUTPUT_IMG="bench_output/${SLUG}_${RES_WIDTH}.png"
    
    TIMES=()
    TOTAL_RSS=0
    LOCKS=0
    
    echo -n "  $NAME: "
    
    # Build the binary with no_deadlocks feature
    cargo build $CARGO_FLAGS --no-default-features --features $FEATURE --quiet
    
    for iter in {1..10}; do
          # Run the binary and capture /usr/bin/time -l output
          # Note: Assumes macOS/BSD 'time -l' syntax based on your previous script
          OUTPUT=$(/usr/bin/time -l ./target/release/raytracer "$SCENE_FILE" "$OUTPUT_IMG" 2>&1)
          
          TIME_MS=$(echo "$OUTPUT" | grep "Frame time:" | grep -o "[0-9]*" | head -1)
          LOCKS_RUN=$(echo "$OUTPUT" | grep "Total Lock Acquisitions:" | grep -o "[0-9]*" | head -1)
          RSS=$(echo "$OUTPUT" | grep "maximum resident set size" | grep -o "[0-9]*" | head -1)
          
          TIMES+=($TIME_MS)
          
          let "TOTAL_RSS += RSS"
          LOCKS=$LOCKS_RUN # Assumes constant lock count per scene/algo
          
          echo -n "."
    done
    
    # Calculate Statistics using awk
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
    
    read AVG_TIME STD_DEV <<< "$STATS"
    
    # Convert RSS to MB (assuming time -l outputs bytes)
    let "AVG_RSS_BYTES = TOTAL_RSS / 10"
    let "AVG_RSS_MB = AVG_RSS_BYTES / 1024 / 1024"
    
    echo " Done. Avg: ${AVG_TIME}ms +/- ${STD_DEV}ms, Locks: $LOCKS, RSS: ${AVG_RSS_MB}MB"
    
    # Append to the new CSV file
    echo "$NAME,${RES_WIDTH}x${RES_HEIGHT},$AVG_TIME,$STD_DEV,$LOCKS,$AVG_RSS_MB" >> "$CSV_FILE"

done

echo "ND Long Runs Completed. Output saved to $CSV_FILE"
