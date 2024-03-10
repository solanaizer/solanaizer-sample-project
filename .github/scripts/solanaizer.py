import os
from ai_validator import analyze_vulnerability_with_gpt
from pathlib import Path
import json

API_KEY = "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"

def validate_file_content(file_path: Path):
    if file_path.suffix != ".rs":
        print("Not a Rust file.")
        return

    with open(file_path, 'r') as file:
        content = file.read()

    return analyze_vulnerability_with_gpt(API_KEY, content, file_path)

def get_rust_files(directory):
    rust_files = []
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".rs"):
                rust_files.append(os.path.join(root, file))
    return rust_files

if __name__ == "__main__":
    suffix = "src/lib.rs"

    dir_to_search = "programs/"

    rust_files = get_rust_files(dir_to_search)

    json_dumps = []
    for rust_file in rust_files:
        rust_file_path = Path(rust_file)
        json_dumps += validate_file_content(rust_file_path)

    print(json.dumps(json_dumps))

