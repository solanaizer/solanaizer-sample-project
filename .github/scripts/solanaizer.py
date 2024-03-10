from openai import OpenAI
from utils import run_inference, get_exploits

client = OpenAI()

prompt = """
The following Solana smart contract written in Rust may have either no vulnerabilities or one of the following vulnerabilities:

{}

1. Integer overflow.
2  Integer underflow.
3. Unsafe memory.
4. Incorrect execution of authorization.
5. Depth of cross-contract call over four.
6. Reentrancy attack.
7. Errors in logic and arithmetic.
8. Computational units limit.

Return the number of the vulnerability found, or 0 if there is no vulnerability. 
"""


def inference_gpt35_turbo(code_snippet, prompt, client=client) -> str:
    print("Running inference for GPT-3.5-turbo.")
    content = prompt.format(code_snippet)

    response = client.chat.completions.create(
        model="gpt-3.5-turbo",
        messages=[
            {
                "role": "system",
                "content": "You are a Rust programmer who is finding security vulnerabilities in Solana projects.",
            },
            {"role": "user", "content": prompt.format(code_snippet)},
        ],
        temperature=0,
    )
    response_extracted = response.choices[0].message.content

    return response_extracted


if __name__ == "__main__":
    filenames, exploits = get_exploits("exploits")
    run_inference(exploits, filenames, inference_gpt35_turbo, "gpt-3.5-turbo", prompt)
