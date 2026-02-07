#!/usr/bin/env python3
"""
Detection Rate Analysis Script
Analyzes CSV files from deadlock detection tests to calculate detection rates,
timeout rates, and average detection times.
"""

import pandas as pd
import glob
import os
import sys

def analyze_detection_rate():
    """Analyze detection rates from all deadlock test CSV files."""
    
    # Check both probabilistic and guaranteed test folders
    csv_files = []
    for pattern in ['../deadlock_tests/*.csv', '../guaranteed_deadlock_tests/*.csv']:
        csv_files.extend(glob.glob(pattern))
    
    if not csv_files:
        print(f"❌ No CSV files found in deadlock_tests or guaranteed_deadlock_tests folders")
        print(f"   Current directory: {os.getcwd()}")
        print(f"   Run this script from: deloxide-deadlock-tests/analysis/")
        sys.exit(1)
    
    print(f"📊 Analyzing {len(csv_files)} test result files...")
    print(f"   Probabilistic tests: {len(glob.glob('../deadlock_tests/*.csv'))}")
    print(f"   Guaranteed tests: {len(glob.glob('../guaranteed_deadlock_tests/*.csv'))}")
    print()
    
    results = []
    
    for csv_file in sorted(csv_files):
        name = os.path.basename(csv_file).replace('.csv', '')
        # Add marker for guaranteed vs probabilistic tests
        test_type = 'G' if 'guaranteed_deadlock_tests' in csv_file else 'P'
        
        try:
            # Try reading with seed column first, fall back to old format
            try:
                df = pd.read_csv(csv_file, names=['flagged', 'timeout', 'elapsed', 'seed'], 
                               skipinitialspace=True)
            except:
                df = pd.read_csv(csv_file, names=['flagged', 'timeout', 'elapsed'], 
                               skipinitialspace=True)
            
            if len(df) == 0:
                print(f"⚠️  Warning: {name} is empty")
                continue
            
            total = len(df)
            detected = df['flagged'].sum()
            timed_out = df['timeout'].sum()
            
            detection_rate = (detected / total) * 100 if total > 0 else 0
            timeout_rate = (timed_out / total) * 100 if total > 0 else 0
            
            # Calculate average runtime for all test runs
            avg_time_ms = df['elapsed'].mean() * 1000
            
            results.append({
                'Type': test_type,
                'Test': name,
                'Total Runs': total,
                'Detected': int(detected),
                'Detection Rate (%)': f"{detection_rate:.1f}",
                'Timed Out': int(timed_out),
                'Timeout Rate (%)': f"{timeout_rate:.1f}",
                'Avg Runtime (ms)': f"{avg_time_ms:.2f}"
            })
        except Exception as e:
            print(f"⚠️  Error processing {name}: {e}")
            continue
    
    if not results:
        print("❌ No results to display")
        sys.exit(1)
    
    # Create DataFrame and display
    df_results = pd.DataFrame(results)
    
    print("=" * 120)
    print("DETECTION RATE ANALYSIS")
    print("=" * 120)
    print(df_results.to_string(index=False))
    print("=" * 120)
    
    # Save to markdown file
    output_file = 'DETECTION_ANALYSIS.md'
    with open(output_file, 'w') as f:
        f.write("# Detection Rate Analysis\n\n")
        f.write("**Type Legend**: `P` = Probabilistic (heisenbug), `G` = Guaranteed (barrier-synchronized)\n\n")
        f.write("## Summary\n\n")
        f.write(df_results.to_markdown(index=False))
        f.write("\n\n## Key Findings\n\n")
        
        # Calculate statistics separately for probabilistic and guaranteed
        prob_tests = df_results[df_results['Type'] == 'P']
        guaranteed_tests = df_results[df_results['Type'] == 'G']
        
        f.write(f"### Overall Statistics\n\n")
        f.write(f"- **Total test configurations**: {len(df_results)}\n")
        f.write(f"  - Probabilistic (heisenbug) tests: {len(prob_tests)}\n")
        f.write(f"  - Guaranteed deadlock tests: {len(guaranteed_tests)}\n\n")
        
        if len(prob_tests) > 0:
            avg_prob = prob_tests['Detection Rate (%)'].str.rstrip('%').astype(float).mean()
            f.write(f"- **Probabilistic tests average**: {avg_prob:.1f}%\n")
        
        if len(guaranteed_tests) > 0:
            avg_guar = guaranteed_tests['Detection Rate (%)'].str.rstrip('%').astype(float).mean()
            f.write(f"- **Guaranteed tests average**: {avg_guar:.1f}%\n\n")
        
        # Find best and worst performers
        df_with_numeric = df_results.copy()
        df_with_numeric['Detection Rate Numeric'] = df_with_numeric['Detection Rate (%)'].str.rstrip('%').astype(float)
        
        if len(df_with_numeric) > 0:
            best = df_with_numeric.loc[df_with_numeric['Detection Rate Numeric'].idxmax()]
            worst = df_with_numeric.loc[df_with_numeric['Detection Rate Numeric'].idxmin()]
            
            f.write(f"### Performance Range\n\n")
            f.write(f"- **Best performer**: {best['Test']} ({best['Type']}) - {best['Detection Rate (%)']}%\n")
            f.write(f"- **Worst performer**: {worst['Test']} ({worst['Type']}) - {worst['Detection Rate (%)']}%\n")
    
    print(f"\n✅ Results saved to: {output_file}")
    print(f"\n💡 Tip: Focus on tests with deloxide_aggressive for best detection rates")

if __name__ == '__main__':
    try:
        analyze_detection_rate()
    except KeyboardInterrupt:
        print("\n\n⚠️  Analysis interrupted by user")
        sys.exit(1)



