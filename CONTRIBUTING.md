# Contributing to DevLint

Thank you for contributing to DevLint.

Our goal is to build a high-quality Rust developer tool while providing contributors with a professional open-source experience.

---

## Development Workflow

### 1. Fork and Clone

Fork the repository and clone it locally.

### 2. Create a Branch

Use the following naming convention:

```text
feature/<issue-number>-short-description
```

Examples:

```text
feature/12-env-parser
feature/18-readme-score
```

---

## Pull Request Rules

* One feature per pull request
* Keep pull requests focused and small
* Reference the related GitHub issue
* Ensure tests pass before requesting review

---

## Coding Standards

### Rust

* Follow Rust formatting conventions
* Run cargo fmt before committing
* Run cargo clippy before opening a PR

### Error Handling

Prefer structured error handling over panics.

Avoid:

```rust
unwrap()
expect()
```

unless absolutely necessary.

---

## Commit Messages

Use clear commit messages.

Examples:

```text
feat: add env variable parser
fix: handle missing readme file
docs: update roadmap
```

---

## Scope Policy

Only work on issues assigned to the current milestone.

Features outside the scope of the active milestone will not be accepted.

---

## Communication

If you are blocked:

1. Comment on the issue
2. Ask questions early
3. Do not spend multiple days stuck

Open communication is preferred over silent delays.

---

## First-Time Contributors

Look for issues labeled:

```text
good-first-issue
```

These issues are designed to help new contributors become familiar with the codebase.

Thank you for helping build DevLint.
