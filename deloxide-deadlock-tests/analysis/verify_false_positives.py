#!/usr/bin/env python3
"""
False Positive Verification Script
Verifies that false positive tests completed successfully without panics.

Two types of FP tests:
1. Traditional FP tests: Should NEVER trigger any detector
2. Lock order FP tests: Should trigger lock order detectors but NOT wait-for graph detectors
"""

import pandas as pd
import glob
import os
import sys

# Tests that specifically target lock order graph false positives
LOCK_ORDER_FP_TESTS = [
    'lock_order_inversion_fp',
    'complex_lock_order_fp'
]

def is_lock_order_fp_test(test_name):
    """Check if this is a lock order specific FP test."""
    return any(lo_test in test_name for lo_test in LOCK_ORDER_FP_TESTS)

def is_lock_order_detector(test_name):
    """Check if this test uses a lock order detector."""
    # Check if the detector variant (after _fp_) contains lock_order
    # e.g., "test_fp_deloxide_lock_order" or "test_fp_deloxide_random_default_lock_order"
    # Split on _fp_ and check the detector part
    if '_fp_' in test_name:
        detector_part = test_name.split('_fp_')[1]
        return detector_part.endswith('_lock_order') or '_lock_order' in detector_part
    return False

def verify_fp():
    """Verify false positive tests - presence of data means no false positives."""
    
    csv_pattern = '../fp_tests/*.csv'
    csv_files = glob.glob(csv_pattern)
    
    if not csv_files:
        print(f"❌ No CSV files found matching pattern: {csv_pattern}")
        print(f"   Current directory: {os.getcwd()}")
        print(f"   Run this script from: deloxide-deadlock-tests/analysis/")
        sys.exit(1)
    
    print(f"🔍 Verifying {len(csv_files)} false positive test files...\n")
    
    results = []
    total_false_positives = 0
    total_expected_detections = 0
    total_correct_detections = 0
    
    for csv_file in sorted(csv_files):
        name = os.path.basename(csv_file).replace('.csv', '')
        
        try:
            # FP tests log: tool_flagged, timed_out, elapsed
            df = pd.read_csv(csv_file, names=['tool_flagged', 'timed_out', 'elapsed'], skipinitialspace=True)
            
            if len(df) == 0:
                print(f"⚠️  Warning: {name} is empty - test may not have run")
                status = '⚠️  EMPTY'
                false_positives = '?'
            else:
                total = len(df)
                avg_time = df['elapsed'].mean()
                min_time = df['elapsed'].min()
                max_time = df['elapsed'].max()
                std_time = df['elapsed'].std()
                
                # Count how many times the tool flagged
                flagged_count = df['tool_flagged'].sum()
                
                # Determine expected behavior
                is_lock_order_test = is_lock_order_fp_test(name)
                uses_lock_order_detector = is_lock_order_detector(name)
                
                # ALL FP tests should NEVER flag - any flag is a false positive
                expected_flags = 0
                false_positives = flagged_count
                total_false_positives += false_positives
                
                if is_lock_order_test and uses_lock_order_detector:
                    # Lock order FP test with lock order detector
                    # These WILL flag (known limitation), but it's still a false positive
                    total_expected_detections += total
                    total_correct_detections += flagged_count
                    
                    if flagged_count == total:
                        status = '❌ KNOWN FP (Lock Order)'
                    elif flagged_count > 0:
                        status = f'❌ {flagged_count} FALSE POS'
                    else:
                        status = '⚠️  UNEXPECTED PASS'
                        
                elif flagged_count == 0:
                    # No flags - correct behavior
                    status = '✅ PASS'
                else:
                    # Unexpected flags - true false positive
                    status = f'❌ {false_positives} FALSE POS'
                
                test_type = 'Lock Order FP' if is_lock_order_test else 'Traditional FP'
                detector_type = 'Lock Order' if uses_lock_order_detector else 'Wait-For'
                
                results.append({
                    'Test': name,
                    'Type': test_type,
                    'Detector': detector_type,
                    'Runs': total,
                    'Flagged': flagged_count,
                    'Expected': expected_flags,
                    'False Positives': false_positives if not (is_lock_order_test and uses_lock_order_detector) else 0,
                    'Avg Time (s)': f"{avg_time:.4f}",
                    'Status': status
                })
        except Exception as e:
            print(f"⚠️  Error processing {name}: {e}")
            results.append({
                'Test': name,
                'Runs': 0,
                'False Positives': '?',
                'Suspicious': '?',
                'Avg Time (s)': 'ERROR',
                'Min Time (s)': 'ERROR',
                'Max Time (s)': 'ERROR',
                'Std Dev (s)': 'ERROR',
                'Status': '❌ ERROR'
            })
            continue
    
    if not results:
        print("❌ No results to display")
        sys.exit(1)
    
    # Create DataFrame and display
    df_results = pd.DataFrame(results)
    
    print("=" * 160)
    print("FALSE POSITIVE VERIFICATION")
    print("=" * 160)
    print(df_results.to_string(index=False))
    print("=" * 160)
    
    # Calculate statistics
    passed = len([r for r in results if '✅' in r['Status']])
    total = len(results)
    
    traditional_fp_tests = [r for r in results if r['Type'] == 'Traditional FP']
    lock_order_fp_tests = [r for r in results if r['Type'] == 'Lock Order FP']
    
    traditional_false_pos = sum(r['False Positives'] for r in traditional_fp_tests)
    lock_order_false_pos = sum(r['False Positives'] for r in lock_order_fp_tests if r['Detector'] == 'Wait-For')
    
    # Save to markdown file
    output_file = 'FALSE_POSITIVE_ANALYSIS.md'
    with open(output_file, 'w') as f:
        f.write("# False Positive Verification\n\n")
        f.write("## Overview\n\n")
        f.write("This analysis verifies two types of false positive tests:\n\n")
        f.write("1. **Traditional FP Tests**: Should NEVER trigger any detector\n")
        f.write("2. **Lock Order FP Tests**: Should trigger lock order detectors but NOT wait-for graph detectors\n\n")
        
        f.write("## Test Results\n\n")
        f.write(df_results.to_markdown(index=False))
        
        f.write("\n\n## Interpretation\n\n")
        f.write("- **Type**: Traditional FP or Lock Order FP\n")
        f.write("- **Detector**: Wait-For (runtime) or Lock Order (static analysis)\n")
        f.write("- **Flagged**: Number of times the detector flagged a deadlock\n")
        f.write("- **Expected**: Expected number of flags (should always be 0 for FP tests)\n")
        f.write("- **False Positives**: Number of incorrect detections\n")
        f.write("- **Status**:\n")
        f.write("  - ✅ PASS: No false positives detected\n")
        f.write("  - ❌ KNOWN FP (Lock Order): Known limitation of lock order graph detection\n")
        f.write("  - ❌ FALSE POS: Incorrect detection (false positive)\n")
        
        f.write("\n## Summary\n\n")
        f.write(f"- **Total test configurations**: {total}\n")
        f.write(f"- **Passed (no false positives)**: {passed}/{total}\n")
        f.write(f"- **Traditional FP tests**: {len(traditional_fp_tests)}\n")
        f.write(f"  - False positives: {traditional_false_pos}\n")
        f.write(f"- **Lock Order FP tests**: {len(lock_order_fp_tests)}\n")
        f.write(f"  - Total false positives: {sum(r['False Positives'] for r in lock_order_fp_tests)}\n")
        f.write(f"  - Known FP (lock order detectors): {total_correct_detections}\n")
        f.write(f"  - Unexpected FP (wait-for detectors): {lock_order_false_pos}\n")
        f.write(f"- **Total false positives across all tests**: {total_false_positives}\n\n")
        
        lock_order_known_fps = sum(1 for r in results if 'KNOWN FP' in r['Status'])
        
        f.write("## Analysis\n\n")
        f.write(f"### False Positive Breakdown:\n")
        f.write(f"- **Traditional FP tests**: {traditional_false_pos} false positives\n")
        f.write(f"- **Lock Order FP tests (known limitation)**: {lock_order_known_fps} configurations\n")
        f.write(f"- **Lock Order FP tests (unexpected)**: {lock_order_false_pos} false positives\n\n")
        
        if traditional_false_pos == 0 and lock_order_false_pos == 0:
            f.write("✅ **GOOD**: No unexpected false positives!\n\n")
            f.write("- Traditional FP tests: All passed correctly\n")
            f.write("- Lock order FP tests: Only flagged by lock order detectors (expected limitation)\n\n")
            if lock_order_known_fps > 0:
                f.write(f"⚠️  **Note**: {lock_order_known_fps} known false positives from lock order graph detection\n")
                f.write("   These demonstrate the limitation of static lock order analysis.\n")
        else:
            f.write(f"❌ **ISSUES DETECTED**:\n\n")
            if traditional_false_pos > 0:
                f.write(f"- {traditional_false_pos} unexpected false positives in traditional FP tests\n")
            if lock_order_false_pos > 0:
                f.write(f"- {lock_order_false_pos} unexpected false positives in lock order FP tests (wait-for detectors)\n")
    
    print(f"\n✅ Results saved to: {output_file}")
    
    lock_order_known_fps = sum(1 for r in results if 'KNOWN FP' in r['Status'])
    
    print(f"\n📊 Summary:")
    print(f"   Traditional FP tests: {len(traditional_fp_tests)} configurations, {traditional_false_pos} false positives")
    print(f"   Lock Order FP tests: {len(lock_order_fp_tests)} configurations")
    print(f"     - Known false positives (lock order detectors): {lock_order_known_fps}")
    print(f"     - Unexpected false positives (wait-for detectors): {lock_order_false_pos}")
    print(f"   Total false positives: {total_false_positives}")
    
    if traditional_false_pos == 0 and lock_order_false_pos == 0:
        print(f"\n✅ No unexpected false positives!")
        if lock_order_known_fps > 0:
            print(f"⚠️  {lock_order_known_fps} known false positives from lock order graph detection")
            print(f"   (This demonstrates the limitation of static lock order analysis)")
    else:
        print(f"\n❌ {traditional_false_pos + lock_order_false_pos} unexpected false positives detected!")

if __name__ == '__main__':
    try:
        verify_fp()
    except KeyboardInterrupt:
        print("\n\n⚠️  Verification interrupted by user")
        sys.exit(1)



