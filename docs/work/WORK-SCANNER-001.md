# WORK-SCANNER-001 — Deterministic ADP project discovery and parsing

## Outcome
A tested, read-only analyzer that identifies supported ADP project structures, parses canonical project records, and emits explainable versioned JSON without overstating conformance.

## Scope
- Define supported ADP versions/layouts.
- Parse .project/WORK.json, root AGENTS, status/continuity, and linked contracts as supported.
- Enforce bounded traversal, safe symlink behavior, error reporting, and evidence locations.
- Add fixtures for valid, missing, contradictory, malformed, and hostile inputs.
- Distinguish KNOWN, UNKNOWN, PARTIAL, NOT_APPLICABLE, and NOT_ASSESSED.
- Validate machine-readable schema and deterministic ordering.

## Out of scope
Dashboard, SQLite history, release launchers, and autonomous project modification.

## Acceptance criteria
- Golden fixtures produce stable outputs with traceable source paths.
- Unsupported formats and incomplete scans do not generate false passing verdicts.
- No target-project commands are executed or files modified.
- CI passes formatting, lint, tests, and build.
- Rule applicability and known limitations are documented.

## Dependencies
WORK-BOOTSTRAP-001.

## Current state
Candidate; not active until bootstrap is verified.

## Plan / atomic units
- AU-01: choose fixtures and normalized evidence model.
- AU-02: implement safe parsing and scan completeness.
- AU-03: add high-confidence structural rules and JSON schema.
- AU-04: add regression/security tests and CI evidence.
- AU-05: review false positives and update continuity.

## Validation / evidence
GitHub Actions and reproducible fixture test outputs; explicit scope and limitations.

## Risks / decisions
Canonical ADP does not guarantee all projects use identical layouts. Use version-aware applicability and avoid filename-only conformance claims.

## Handoff / next recommended phase / exact next action
After bootstrap completion, inspect existing src/main.rs and implement the smallest parsing vertical slice with fixtures.
