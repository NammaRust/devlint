# DevLint Scope Document

This document defines the official scope for DevLint v1.0.0.

Anything not explicitly listed here is considered out of scope.

---

# Vision

DevLint is a framework-agnostic project quality validator that helps developers identify configuration, documentation, and contract issues before deployment.

---

# v1.0.0 Objectives

Deliver a production-ready CLI capable of validating:

1. Environment Configuration
2. Project Documentation
3. API Contracts

---

# Module 1: Environment Validator

## Goal

Validate that environment variables used by an application are correctly configured.

## Checks

### Missing Variables

Used in source code but not defined.

### Unused Variables

Defined but never referenced.

### Empty Variables

Defined but contain no value.

## Supported Languages

* Rust
* Python
* JavaScript
* TypeScript

## Detection Strategy

Regex-based detection.

AST parsing is not included in v1.0.0.

---

# Module 2: README Analyzer

## Goal

Validate project documentation quality.

## Checks

* README exists
* Installation section exists
* Usage section exists
* Contributing section exists
* License section exists

## Output

A quality score from 0–100.

---

# Module 3: Contract Validator

## Goal

Validate JSON payloads against expected schemas.

## Checks

### Missing Fields

Required field not present.

### Extra Fields

Unexpected field present.

### Type Mismatch

Field type differs from expected type.

---

# Explicitly Out of Scope

The following features are not part of v1.0.0:

* OpenAPI Support
* Swagger Support
* GraphQL Support
* AST Parsing
* AI Features
* LLM Integration
* GitHub API Integration
* Web Dashboard
* Database Storage
* Security Scanning
* CI/CD Validation
* Docker Validation

---

# Success Criteria

DevLint v1.0.0 is considered complete when the following commands are functional:

```bash
devlint env .
devlint readme .
devlint contract --schema schema.json --input response.json
devlint scan .
```

and each validator produces actionable output for the user.

---

# Scope Freeze Policy

No new features may be added to v1.0.0 without maintainer approval.

Feature requests beyond this document must be scheduled for future milestones.
