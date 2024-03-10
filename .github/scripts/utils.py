import os
import hashlib
import json
from typing import List


def get_prompt_id(prompt_string: str) -> str:
    """Get a unique id for a prompt string."""

    print("Getting prompt id.")
    hasher = hashlib.sha256()
    hasher.update(prompt_string.encode("utf-8"))

    return hasher.hexdigest()[:10]


def get_exploits(directory: str) -> list:
    """Read all .rs exploits from a given directory and return them as a list of strings."""
    example_exploits = []
    filenames = []

    for filename in os.listdir(directory):
        filenames.append(filename)
        if filename.endswith(".rs"):
            with open(os.path.join(directory, filename), "r") as file:
                contents = file.readlines()
                contents = "".join(contents)
                example_exploits.append(contents)

    print("We have {} example exploits.".format(len(example_exploits)))

    return filenames, example_exploits


def save_responses(
    responses: List[str], filenames, inference_model_name: str, prompt: str
):
    """Flatten all responses (and prompt used) to a string and save in the right place."""

    directory = "bugs"

    prompt_id = get_prompt_id(prompt)
    filename = f"{inference_model_name}_{prompt_id}.txt"

    output_string = "\n".join(responses) + "\n \n" + "\n".join(filenames)

    output_string = prompt + "\n" + output_string

    os.makedirs(directory, exist_ok=True)

    with open(os.path.join(directory, filename), "w") as file:
        file.write(output_string)

    print("Saved inferenced exploits to a file.")


def extract_class(response):
    """Extract the class of an exploit from the response of an LLM.
    If no number found, return 0 (no exploit)."""
    reversed_s = response[::-1]

    for char in reversed_s:
        if char.isdigit():
            return int(char)

    return 0


def map_class_with_output(class_number):
    """Map a class number to a JSON object with title and description keys."""

    vulnerability_map = {
        0: {
            "title": "No vulnerability found.",
            "description": "The code does not contain any vulnerabilities.",
        },
        1: {
            "title": "Integer overflow.",
            "description": "An arithmetic operation resulted in a value that exceeds the maximum representable value for the data type.",
        },
        2: {
            "title": "Integer underflow.",
            "description": "An arithmetic operation resulted in a value that is less than the minimum representable value for the data type.",
        },
        3: {
            "title": "Unsafe memory.",
            "description": "Accessing memory in an unsafe manner, potentially leading to security vulnerabilities.",
        },
        4: {
            "title": "Incorrect execution of authorization.",
            "description": "Failure to properly check authorization before executing a privileged operation.",
        },
        5: {
            "title": "Depth of cross-contract call over four.",
            "description": "The depth of a cross-contract call exceeds the allowed limit.",
        },
        6: {
            "title": "Reentrancy attack.",
            "description": "A contract's reentrancy vulnerability allows it to be called repeatedly before previous invocations complete.",
        },
        7: {
            "title": "Errors in logic and arithmetic.",
            "description": "Logical or arithmetic errors in the code that can lead to unexpected behavior.",
        },
        8: {
            "title": "Computational units limit.",
            "description": "The computation exceeds the computational units limit imposed by the platform.",
        },
    }

    return str(
        vulnerability_map.get(
            class_number,
            {
                "title": "Unknown vulnerability.",
                "description": "The vulnerability class number provided does not match any known vulnerabilities.",
            },
        )
    )


def run_inference(
    exploits, filenames, inference_function, inference_model_name, prompt
):
    """Run inference for a given model and prompt used and save results that could be traced to a prompt and model used."""
    responses = []

    for exploit in exploits:
        responses.append(
            map_class_with_output(extract_class(inference_function(exploit, prompt)))
        )

    save_responses(responses, filenames, inference_model_name, prompt)
    return responses
