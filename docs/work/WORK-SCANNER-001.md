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
In progress (active M1 contract). First WORK schema-v1 and contextual compatibility vertical slice VERIFIED in GitHub Actions run 39573431375. Full M1 scope remains incomplete.

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
Continue from the verified compatibility slice: define rigorous scan-completeness reporting, add safe work contract-file reference checks, expand hostile/missing-file fixtures, and inspect CI before claiming further completion.

## Compatibility-first scope refinement (2026-10-21)

The current parser is a structural prototype, not a version-aware conformance engine. Preserve existing implementation while incrementally adding safe adapters and source provenance.

### Added atomic units
- AU-06: define three compatibility dimensions (format, semantics, conformance coverage), with unknown and unsupported states.
- AU-07: add independent-file fault isolation; preserve partial results for missing/malformed/conflicting records.
- AU-08: create version and completeness fixture matrix, including optional WORK.json and unknown/newer ADP versions.
- AU-09: verify no target modifications, no extra ADP metadata requirement, and no false conformance pass.
- AU-10: test source-linked conflict reporting and conservative relationship extraction.

### Acceptance refinements
- No analyzer-specific file or mandatory ADP metadata in target projects.
- Missing optional work ledger yields UNKNOWN/NOT_ASSESSED, not an automatic violation.
- A readable record under an unsupported ADP version is not automatically semantically understood.
- A bad file cannot suppress unrelated safe results.
- Unknown conformance is never represented as a passing verdict.

### Current verification
Limited compatibility behavior is implemented in src/compatibility.rs, src/records.rs, and src/work.rs, with CLI report schema v0.2.0. GitHub Actions run 39573431375 passed formatting, Clippy, unit and integration tests, and release build. This verifies the implemented subset only; adapter coverage beyond ADP 2.7.1/WORK schema v1 and complete protocol conformance remain unavailable.

## Evidence checkpoint (2026-10-21)

- AU-01: fixtures for current/older/newer/unknown, malformed, missing, and conflicting records — verified.
- AU-02: safe work-index and context reads with symlink refusal — implemented; full scan-completeness semantics remain open.
- AU-03: WORK schema-v1 field checks, compatibility dimensions, JSON report schema — verified as a subset.
- AU-04: unit and CLI integration tests, including repeatability and unchanged target records — verified.
- AU-05: representative real-project coverage and false-positive calibration — outstanding.
- AU-06: three compatibility dimensions and explicit WORK_JSON_SCHEMA_V1_ONLY scope — verified.
- AU-07: missing/malformed context isolation and sourced claim conflicts — verified for tested cases.
- AU-08: version/missing/conflict fixtures — initial matrix verified; broken-link and larger hostile fixtures remain outstanding.
- AU-09: basic read-only tests — verified; independent security review and stress tests remain outstanding.
- AU-10: explicit conflict reporting — verified; broader conservative relationship extraction remains outstanding.

Validation: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39573431375

**Next exact action:** inspect active branch/CI, implement bounded file-reference checks (missing/symlinked/outside-root contract references), and improve scan completeness without turning missing optional ADP records into errors.

## Verified scanner hardening checkpoint (2026-10-21)

- Added src/references.rs: bounded metadata-only checks for explicitly referenced optional contract files, with source JSON pointers and statuses PRESENT, MISSING, SYMLINK, OUTSIDE_ROOT, INVALID_PATH, NOT_FILE, NOT_DIRECTORY, UNREADABLE.
- Added src/discovery.rs: deterministic bounded traversal with separate counts for ignored directories, symlinks, oversized files, metadata/read errors, depth and entry limits. scan_complete now describes traversal under disclosed policy rather than evidence validity.
- Extended WORK.json read-only inspection, CLI reporting, and schema/scan-report.schema.json 0.3.0. Advanced development package to 0.3.0.
- Added CLI fixtures for valid/missing/unsafe references and a test that scans this repository's own ADP work records.
- CI verification: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39575987006 (formatting, lint, tests, build all passed at that code revision).

**Still open in M1:** broader adversarial and real-project tests, full parser coverage for historical versions, source-link and scan-scope calibration, and more complete schema validation. Do not mark the full scanner VERIFIED from one green subset.

**Exact next action:** test selected real repositories against current discovery rules; investigate false positives/negatives and parser/layout variation, then implement only the next high-value improvement with regression fixtures. Do not amend canonical ADP for the analyzer.

## Real upstream ADP Actions smoke checkpoint — 2026-10-21

- Added CI checkout of actual `FrederikLive/ADP` at source commit
  `9b5ab4694321735e83b93962e1b9975cced130bf`, including the optional
  Actions catalogue, update guide and agent router.
- JSON and HTML scanning verified: recognized protocol version 2.7.1,
  `WORK.json` absent as an **optional** record, no false conformance pass,
  and no added requirement to inspect Actions/Updater documents.
- The Observatory self-scan also found structurally valid actual work
  records without incorrectly inferring a local protocol version from an
  upstream-only reference.
- [CI 39659907217](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39659907217)
  passed Rust formatting, Clippy, tests, real upstream checkout/scan,
  real project self-scan and Agent Skill end-to-end launch.
- **M1 still OPEN**: wider real-project samples, stress/hostile-input
  calibration, historical version adapters and complete evidence linking
  remain out of scope for this specific compatibility checkpoint.

**Next action:** expand independent real-project fixtures and false-positive
analysis without introducing a mandatory canonical ADP data interface.

## ADP 2.8.0 compatibility regression slice (2026-10-28; CI passed on PR branch)
- Observed concrete mismatch: schema-v1 `execution.assignment` was newly legal in ADP 2.8.0, but Observatory rejected its key and treated valid work as invalid.
- Branch changes validate optional nullable assignment and recognized fields, add end-to-end fixture and unit tests, and pin CI to reviewed upstream 2.8.0 commit.
- Supports format/semantics for the tested WORK schema-v1 subset only; independent ADP conformance is still NOT_ASSESSED. Does not start the deferred document explorer or add agent runtime instrumentation.
- Branch CI passed at [40756821398](https://github.com/FrederikLive/ADP-Observatory/actions/runs/40756821398); PR #4 remains draft. Next evidence: independent consumer project calibration, followed by scoped safe read-time source inspection.

## Bounded source reading slice — 2026-10-28 (CI passed; review pending)
- Introduced capability-relative Markdown inspection with strict resource budgets, content digests, line/heading counts and explicit error states. Unlike the previous `PRESENT` reference check, this opens document handles under the selected root and declines to follow symlinks at each component.
- Added unit fixtures for hostile paths, symlinks, large content, malformed UTF-8 and absent optional WORK.json. No Markdown body/heading text is exported yet; this deliberately precedes the opt-in Explorer feature.
- Report v0.5.0 surfaces metadata-only source coverage. The new Windows/macOS CI jobs and Linux checks passed in run 40759004986, but this is not a substitute for adversarial race/security review and 30/300-contract performance sampling.
- Gate: complete independent security and real-project review before treating the reader as VERIFIED or activating the deferred Explorer contract.
