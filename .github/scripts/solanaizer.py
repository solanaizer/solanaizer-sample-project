from hardcoded_vulnerability_checks import vulnerability_checks
from vulnerability_types import VulnerabilityResult
import os
import sys
from ai_validator import analyze_vulnerability_with_gpt
from pathlib import Path

API_KEY = "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"
def validate_file_content(file_path):
    if file_path.suffix != ".rs":
        print("Not a Rust file.")
        return

    with open(file_path, 'r') as file:
        content = file.read()

    analyze_vulnerability_with_gpt(API_KEY, content)

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description='Validate Rust files')
    parser.add_argument('--fp', metavar='filepath', required=True,
                        help='the path to the file to validate')

    args = parser.parse_args()
    file_path_str = args.fp

    file_path = Path(file_path_str)
    if not file_path.exists():
        print("File not found.")
        sys.exit(1)
    validate_file_content(file_path)