from hardcoded_vulnerability_checks import vulnerability_checks
from vulnerability_types import VulnerabilityResult
import re
import os
import sys
from pathlib import Path

def validate_file_content(file_path):
    if file_path.suffix != ".rs":
        print("Not a Rust file.")
        return
    
    with open(file_path, 'r') as file:
        content = file.read()

    vulnerabilities = []
    for check in vulnerability_checks:
        pattern_regex = re.compile(check.pattern)
        safe_pattern_regex = re.compile(check.safe_pattern) if check.safe_pattern else None
        
        captures = pattern_regex.search(content)
        if captures:
            line_of_code = captures.group(0)
            line_number = content[:captures.start()].count('\n') + 1
            status, fix = "Status", "Fix"
            vulnerabilities.append(VulnerabilityResult(
                vulnerability_id=check.id,
                file=str(file_path),
                title=check.title,
                severity=check.severity,
                status=status,
                description=check.description,
                fix=fix,
                persistence_of_safe_pattern="No",
                safe_pattern=check.safe_pattern
            ))

        if safe_pattern_regex:
            for _mat in safe_pattern_regex.finditer(content):
                pass

    # Print vulnerabilities
    print("Vulnerabilities found:")
    for vulnerability in vulnerabilities:
        print(vulnerability)


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