# DevLint

> **A fast, framework-agnostic project quality validator for environment variables, documentation, and API contracts.**

<p align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge\&logo=rust)
![CLI](https://img.shields.io/badge/CLI-Framework%20Agnostic-blue?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-In%20Development-orange?style=for-the-badge)
![Version](https://img.shields.io/badge/Version-v1.0.0--alpha-lightgrey?style=for-the-badge)

</p>

---

## Overview

**DevLint** is a Rust-powered command-line tool designed to identify common software quality issues before they reach production.

Instead of discovering configuration mistakes, incomplete documentation, or API contract mismatches during deployment, DevLint performs lightweight static validation directly within your project.

Built to be **framework-agnostic**, it works across multiple programming languages and integrates naturally into local development workflows and CI/CD pipelines.

---

## Why DevLint?

Many production issues originate from simple but overlooked mistakes:

* Missing environment variables
* Unused configuration values
* Empty configuration entries
* Incomplete project documentation
* API contract mismatches

These issues are often detected only during testing—or worse, after deployment.

DevLint helps catch them early with fast, actionable validation.

---

# Features

## Environment Validation

Analyze project source code and compare detected environment variables against configuration files.

### Checks

* Missing environment variables
* Unused environment variables
* Empty variable values

### Supported Languages (v1)

* Rust
* Python
* JavaScript
* TypeScript

---

## README Analysis

Evaluate project documentation for completeness and maintainability.

### Checks

* README existence
* Installation section
* Usage section
* Contributing section
* License section

### Output

* Documentation Quality Score
* Missing section report
* Improvement suggestions

---

## API Contract Validation

Validate JSON payloads against expected schemas to ensure interface consistency.

### Checks

* Missing fields
* Unexpected fields
* Data type mismatches

---

# Installation

> **Coming soon**

```bash
cargo install devlint
```

---

# Usage

## Validate Environment Variables

```bash
devlint env .
```

---

## Analyze README

```bash
devlint readme .
```

---

## Validate API Contracts

```bash
devlint contract \
  --schema schema.json \
  --input response.json
```

---

## Run Every Validator

```bash
devlint scan .
```

---

# Example Output

```text
Environment Validation:
[PASS] Environment Validation

[PASS] 18 variables found
[PASS] 17 configured
[WARN] Missing:
  DATABASE_URL

[WARN] Unused:
  OLD_API_KEY

Documentation Analysis:
[PASS] README exists
[INFO] Documentation Score: 88/100

API Contract:
[PASS] Schema validation passed
```

---

# Roadmap

## v1.0.0

* [x] Environment Validator
* [x] README Analyzer
* [x] JSON Contract Validator

---

## Planned Features

* [ ] License Validator
* [ ] Docker Validator
* [ ] CI/CD Configuration Validator
* [ ] Security Configuration Validator
* [ ] Git Ignore Validator
* [ ] Dependency Health Checks
* [ ] Project Structure Validation
* [ ] GitHub Actions Support

---

# Contributing

Contributions are welcome.

Please read **CONTRIBUTING.md** before submitting issues or pull requests.

---

# License

This project is licensed under the **MIT License**.

---

<p align="center">
Built by the <strong>NammaRust Developer Community</strong>
</p>
