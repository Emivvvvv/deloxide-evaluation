#!/usr/bin/env python3
"""
Detector Comparison Script
Compares detection rates across different deadlock detectors and stress modes.
Groups results by test scenario to show which detector performs best.
"""

import pandas as pd
import glob
import os
import sys

# (suffix, column label, human-readable description)
DETECTOR_PATTERNS = [
    ('parking_lot', 'PL', 'parking_lot deadlock detector'),
    ('no_deadlocks', 'ND', 'no_deadlocks detector'),
    ('deloxide_random_default', 'DX-Rand', 'Deloxide + Random Stress'),
    ('deloxide_aggressive', 'DX-Agg', 'Deloxide + Aggressive Stress'),
    ('deloxide_gentle', 'DX-Gent', 'Deloxide + Gentle Stress'),
    ('deloxide_component_based_delays', 'DX-Comp', 'Deloxide + Component Stress'),
    ('deloxide', 'DX', 'Deloxide (vanilla)'),
]

DETECTOR_DESCRIPTIONS = {label: desc for _, label, desc in DETECTOR_PATTERNS}

def compare_detectors():
    """Compare detection rates across different detectors for each test scenario."""
    
    # Check both probabilistic and guaranteed test folders
    csv_files = []
    for pattern in ['../deadlock_tests/*.csv', '../guaranteed_deadlock_tests/*.csv']:
        csv_files.extend(glob.glob(pattern))
    
    if not csv_files:
        print(f"❌ No CSV files found in deadlock_tests or guaranteed_deadlock_tests folders")
        print(f"   Current directory: {os.getcwd()}")
        print(f"   Run this script from: deloxide-deadlock-tests/analysis/")
        sys.exit(1)
    
    print(f"📊 Comparing detectors across {len(csv_files)} test configurations...")
    print(f"   Probabilistic tests: {len(glob.glob('../deadlock_tests/*.csv'))}")
    print(f"   Guaranteed tests: {len(glob.glob('../guaranteed_deadlock_tests/*.csv'))}")
    print()
    
    # Data structure: {scenario: {detector_variant: detection_rate}}
    data = {}
    run_counts = {}
    
    for csv_file in sorted(csv_files):
        basename = os.path.basename(csv_file).replace('.csv', '')
        
        # Check if this is a lock order variant
        has_lock_order = '_lock_order' in basename
        
        # Remove lock_order suffix to get base detector name
        if has_lock_order:
            basename_no_lo = basename.replace('_lock_order', '')
        else:
            basename_no_lo = basename
        
        # Try to identify detector suffix from known patterns
        detector_base = None
        testname = None
        for suffix, label, _ in DETECTOR_PATTERNS:
            suffix_token = f"_{suffix}"
            if basename_no_lo.endswith(suffix_token):
                detector_base = label
                testname = basename_no_lo[: -len(suffix_token)]
                break

        if detector_base is None or not testname:
            print(f"⚠️  Warning: Cannot parse detector from {basename}")
            continue
        
        # Create column name: detector_base or detector_base+LO
        if has_lock_order:
            detector_col = f"{detector_base}+LO"
        else:
            detector_col = detector_base
        
        try:
            # Try reading with seed column first, fall back to old format
            try:
                df = pd.read_csv(csv_file, names=['flagged', 'timeout', 'elapsed', 'seed'], 
                               skipinitialspace=True)
            except:
                df = pd.read_csv(csv_file, names=['flagged', 'timeout', 'elapsed'], 
                               skipinitialspace=True)
            
            if len(df) == 0:
                continue
            
            # Exclude timeouts when calculating detection rate
            # Only count runs that didn't timeout (either detected or completed)
            non_timeout_runs = df[df['timeout'] == False]
            
            if len(non_timeout_runs) == 0:
                # All runs timed out, skip this test
                continue
            
            detection_rate = (non_timeout_runs['flagged'].sum() / len(non_timeout_runs)) * 100
            
            if testname not in data:
                data[testname] = {}
                run_counts[testname] = {}
            
            data[testname][detector_col] = detection_rate
            run_counts[testname][detector_col] = len(non_timeout_runs)
            
        except Exception as e:
            print(f"⚠️  Error processing {basename}: {e}")
            continue
    
    if not data:
        print("❌ No data to compare")
        sys.exit(1)
    
    # Create comparison DataFrame
    df_comparison = pd.DataFrame(data).T
    
    # Reorder columns: base detectors first, then lock order variants
    preferred_order = [
        'DX', 'DX+LO',
        'DX-Rand', 'DX-Rand+LO',
        'DX-Agg', 'DX-Agg+LO',
        'DX-Gent', 'DX-Gent+LO',
        'DX-Comp', 'DX-Comp+LO',
        'PL', 'ND'
    ]
    cols = [c for c in preferred_order if c in df_comparison.columns]
    cols += [c for c in df_comparison.columns if c not in cols]
    df_comparison = df_comparison[cols]
    
    # Format as percentages (DataFrame.applymap is deprecated)
    df_formatted = df_comparison.apply(
        lambda col: col.map(lambda x: f"{x:.1f}%" if pd.notnull(x) else "N/A")
    )
    
    print("=" * 160)
    print("DETECTOR COMPARISON - Detection Rates by Test Scenario")
    print("=" * 160)
    print("\nColumn Legend:")
    printed_legend = set()
    for label, description in DETECTOR_DESCRIPTIONS.items():
        if label in df_comparison.columns and label not in printed_legend:
            print(f"  {label:<14}= {description}")
            printed_legend.add(label)
        lo_label = f"{label}+LO"
        if lo_label in df_comparison.columns and lo_label not in printed_legend:
            print(f"  {lo_label:<14}= {description} + Lock Order")
            printed_legend.add(lo_label)
    if any(col.endswith('+LO') for col in df_comparison.columns):
        print("  (+LO columns indicate Deloxide tests paired with the lock-order checker)")
    if 'PL' in df_comparison.columns and 'PL' not in printed_legend:
        print("  PL            = parking_lot deadlock detector")
    if 'ND' in df_comparison.columns and 'ND' not in printed_legend:
        print("  ND            = no_deadlocks detector")
    print("=" * 160)
    print(df_formatted.to_string())
    print("=" * 160)
    
    # Save to markdown
    output_file = 'DETECTOR_COMPARISON.md'
    with open(output_file, 'w') as f:
        f.write("# Detector Comparison\n\n")
        f.write("**Note**: Detection rates exclude timeouts. Only runs that completed (either with detection or without) are counted.\n\n")
        f.write("## Detection Rates by Test Scenario\n\n")
        f.write(df_formatted.to_markdown())
        f.write("\n\n## Analysis\n\n")
        
        # Find best detector for each scenario
        best_detectors = {}
        for scenario in df_comparison.index:
            row = df_comparison.loc[scenario]
            best_detector = row.idxmax()
            best_rate = row.max()
            best_detectors[scenario] = (best_detector, best_rate)
        
        f.write("### Best Detector by Scenario\n\n")
        for scenario, (detector, rate) in sorted(best_detectors.items()):
            f.write(f"- **{scenario}**: {detector} ({rate:.1f}%)\n")
        
        # Overall statistics
        f.write("\n### Overall Statistics\n\n")
        for col in df_comparison.columns:
            avg = df_comparison[col].mean()
            if pd.notnull(avg):
                f.write(f"- **{col}**: Average detection rate = {avg:.1f}%\n")
        
        # Recommendations
        f.write("\n## Recommendations\n\n")
        
        # Find which detector is best overall
        overall_avg = df_comparison.mean()
        best_overall = overall_avg.idxmax()
        best_avg = overall_avg.max()
        
        f.write(f"**Best overall detector**: {best_overall} ({best_avg:.1f}% average)\n\n")
        
        if 'DX-Agg' in df_comparison.columns:
            aggressive_avg = df_comparison['DX-Agg'].mean()
            f.write(f"- `{DETECTOR_DESCRIPTIONS['DX-Agg']}` shows {aggressive_avg:.1f}% detection rate\n")
        
        if 'DX' in df_comparison.columns:
            vanilla_avg = df_comparison['DX'].mean()
            f.write(f"- `{DETECTOR_DESCRIPTIONS['DX']}` shows {vanilla_avg:.1f}% detection rate\n")
        
        f.write("\n💡 For production use, balance detection rate with performance overhead.\n")
    
    print(f"\n✅ Comparison saved to: {output_file}")
    
    # Print summary
    print(f"\n📈 Summary:")
    print(f"   - {len(data)} test scenarios compared")
    print(f"   - {len(cols)} detectors evaluated")
    print(f"   - Best performer: {best_overall} ({best_avg:.1f}% avg detection)")

if __name__ == '__main__':
    try:
        compare_detectors()
    except KeyboardInterrupt:
        print("\n\n⚠️  Comparison interrupted by user")
        sys.exit(1)



