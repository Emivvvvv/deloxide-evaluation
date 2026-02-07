#!/usr/bin/env python3
"""
Master Analysis Script
Runs all analysis scripts and generates a comprehensive report.
"""

import subprocess
import sys
import os

def run_script(script_name):
    """Run a Python script and return success status."""
    print(f"\n{'='*80}")
    print(f"Running: {script_name}")
    print('='*80)
    
    try:
        result = subprocess.run(
            ['python3', script_name],
            cwd=os.path.dirname(os.path.abspath(__file__)),
            check=True,
            capture_output=False
        )
        print(f"✅ {script_name} completed successfully")
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ {script_name} failed with exit code {e.returncode}")
        return False
    except FileNotFoundError:
        print(f"❌ {script_name} not found")
        return False

def main():
    """Run all analysis scripts in sequence."""
    print("🚀 Running Complete Test Suite Analysis")
    print("="*80)
    
    scripts = [
        ('analyze_detection_rate.py', 'Detection Rate Analysis'),
        ('verify_false_positives.py', 'False Positive Verification'),
        ('compare_detectors.py', 'Detector Comparison'),
    ]
    
    results = {}
    
    for script, name in scripts:
        success = run_script(script)
        results[name] = success
    
    # Print summary
    print("\n" + "="*80)
    print("ANALYSIS SUMMARY")
    print("="*80)
    
    all_success = True
    for name, success in results.items():
        status = "✅ PASS" if success else "❌ FAIL"
        print(f"{status}: {name}")
        if not success:
            all_success = False
    
    print("="*80)
    
    if all_success:
        print("\n✅ All analyses completed successfully!")
        print("\nGenerated reports:")
        print("  - DETECTION_ANALYSIS.md")
        print("  - FALSE_POSITIVE_ANALYSIS.md")
        print("  - DETECTOR_COMPARISON.md")
        return 0
    else:
        print("\n⚠️  Some analyses failed. Check output above for details.")
        return 1

if __name__ == '__main__':
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("\n\n⚠️  Analysis interrupted by user")
        sys.exit(1)



