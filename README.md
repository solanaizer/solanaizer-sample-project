```
                     /$$                               /$$     
                    | $$                              |__/ 
  /$$$$$$$  /$$$$$$ | $$  /$$$$$$  /$$$$$$$   /$$$$$$  /$$ /$$$$$$$$  /$$$$$$   /$$$$$$
 /$$_____/ /$$__  $$| $$ |____  $$| $$__  $$ |____  $$| $$|____ /$$/ /$$__  $$ /$$__  $$
|  $$$$$$ | $$  \ $$| $$  /$$$$$$$| $$  \ $$  /$$$$$$$| $$   /$$$$/ | $$$$$$$$| $$  \__/
 \____  $$| $$  | $$| $$ /$$__  $$| $$  | $$ /$$__  $$| $$  /$$__/  | $$_____/| $$
 /$$$$$$$/|  $$$$$$/| $$|  $$$$$$$| $$  | $$|  $$$$$$$| $$ /$$$$$$$$|  $$$$$$$| $$
|_______/  \______/ |__/ \_______/|__/  |__/ \_______/|__/|________/ \_______/|__/
```

# @solanaizer/solanaizer-sample-project

[![Solanaizer AI Audit](https://github.com/solanaizer/solanaizer-sample-project/actions/workflows/solana-audit.yml/badge.svg)](https://github.com/solanaizer/solanaizer-sample-project/actions/workflows/solana-audit.yml)

- **Project:** Solanaizer
- **Event**: Encode Club AI Hackathon in London (Fri 8th March - Sun 10th March)

The purpose of this sample project is to:

- demonstrate Solanaizer, a Solana Smart Contract Auditing tool using AI, readily available on the [Github Action Marketplace](https://github.com/marketplace/actions/github-action-for-solanaizer)
- provide a sample Solana project using the [Anchor Framework](https://github.com/coral-xyz/anchor) ⚓️
- includes Rust smart contracts with vulnerabilities 👾, and some with no vulnerabilities ✅.

## Table of Content
- [Description](#description)
- [Getting Started](#getting-started)
  - [Create a Solana project](#create-a-solana-project)
  - [Add a Github Action using Solanaizer](#add-a-github-action-using-solanaizer)
  - [Add Code](#add-code)
- [Vulnerabilities](#vulnerabilities)
- [Preview](#preview)
- [References](#references)
- [Team](#team-)

## Description

**Solanaizer** is a comprehensive Solana Smart Contrat Auditing Tool using AI 🤖 specifically designed for [Solana](https://solana.com/). Its primary purpose is to thoroughly review and analyze the code governing smart contracts written in [Rust](https://www.rust-lang.org/) for vulnerabilities. 🐛

- **Objective:** 🎯
  - _Solanaizer_ aims to identify potential vulnerabilities, flaws, and security issues within smart contract code.
- **Importance:** ⚠️
  - Given the decentralized nature of blockchain technology, it’s crucial to ensure that smart contracts are secure before deployment. _Solanaizer_ provides this critical layer of protection.
- **Auditing Process:** 🕵🏻‍♀️
  - The tool conducts a detailed analysis of the contract’s code, performed by AI 🤖. It looks for security weaknesses, inefficient code, and other potential risks.
- **LLMs (Language Models):** 💭
  - To enhance its auditing capabilities, _Solanaizer_ leverages various LLMs, including:
    - [Stability.AI Stable Code 3B](https://stability.ai/news/stable-code-2024-llm-code-completion-release)
    - [GPT-4](https://openai.com/gpt-4)
    - [Claude3 Opus](https://www.anthropic.com/news/claude-3-family)

By using **Solanaizer**, developers and users have an extra level of confidence about their smart contracts being free from critical bugs, vulnerabilities, and security flaws, ensuring a safer and more reliable **Solana** ecosystem.

Feel free to explore the repository for more in-depth details and documentation!

## Getting Started

### Create a Solana project

```sh
anchor init my-solana-project && cd my-solana-project
```

### Add a Github Action using Solanaizer

```yml
name: Solanaizer AI Audit

on:
  push:
    branches: main
  pull_request:
    branches: main

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - name: Check-out the repository
        uses: actions/checkout@v4
      - name: Solanaizer Audit
        continue-on-error: false
        uses: solanaizer/solanaizer-action
```

### Add Code

Add your own smart contracts, create a pull request and you should see the reporting in your pull request!

## Vulnerabilities

1. Integer overflow.
2. Integer underflow.
3. Unsafe memory.
4. Incorrect execution of authorization.
5. Depth of cross-contract call over four.
6. Reentrancy attack.
7. Errors in logic and arithmetic.
8. Computational units limit.

## Preview

### On Success

<img width="855" alt="image" src="https://github.com/solanaizer/solanaizer-sample-project/assets/1114325/c9ce0e8c-68f0-4fba-be6b-56627246b8a6">

### On Error

#### In the Logs (Developer-friendly)

<img width="1507" alt="image" src="https://github.com/solanaizer/solanaizer-sample-project/assets/1114325/093f8aaa-1b58-4f89-ae1f-db3113097a79">

#### In the Pull Request (Reviewer-friendly)

<img width="860" alt="image" src="https://github.com/solanaizer/solanaizer-sample-project/assets/1114325/759d4e3a-ed0c-4ba6-bef5-b453a92bc750">

## References

- [Solanaizer on the Github Action Marketplace](https://github.com/marketplace/actions/github-action-for-solanaizer)
- [solanaizer/solanaizer-action Repository](https://github.com/solanaizer/solanaizer-action)

## Team 👨🏻‍💻👨🏻‍💻👨🏻‍💻👨🏻‍💻

- [Alex Straw](https://github.com/alex-straw) (Engineer)
- [Dimitris Dinis Gridian](https://github.com/DinisDimitris) (Engineer)
- [Jean-Philippe Monette](https://github.com/jpmonette) (Engineer)
- [Michal Hoffman](https://github.com/MZHoffman) (Engineer)
- [Mikołaj Kącki](https://github.com/mkacki98) (Engineer)
