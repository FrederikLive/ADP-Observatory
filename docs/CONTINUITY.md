# Continuity

## Active objective
Develop a tolerant, version-aware, read-only Observatory that visualizes existing ADP project records without adding any mandatory agent token or maintenance overhead.

## Active milestone
M1 — Discovery and deterministic scanner.

## Active workstream
Analysis engine.

## Active contract
WORK-SCANNER-001 — docs/work/WORK-SCANNER-001.md.

## Current position
The first compatibility implementation has passed CI. Files src/compatibility.rs, src/work.rs, src/records.rs, src/main.rs, and schema/scan-report.schema.json support scoped format/semantic/conformance status, explicit ADP version evidence, future WORK schema handling, independent context reads, source-linked active-contract conflicts, and JSON schema 0.2.0. tests/scanner.rs runs current/older/future/missing/malformed/conflicting CLI fixtures and read-only/determinism checks.

## Last known good state
GitHub Actions run 39573431375 succeeded for code, formatting, lint, unit and CLI integration tests, and build. Code changes since that run must be verified separately; documentation changes are not new executable evidence.

## Current verification
- ADP M0 baseline: VERIFIED, run 39567705133.
- M1 compatibility scanner subset: VERIFIED, run 39573431375.
- Full ADP conformance evaluation, comprehensive adapters and evidence metrics: NOT IMPLEMENTED / NOT ASSESSED.
- Broader hostile-input fixtures, performance bounds and contract path resolution: NOT RUN / outstanding.

## Work in progress
Complete M1 acceptance: scan completeness semantics, secure contract-file reference inspection, missing/broken path reports, and larger fixture coverage.

## Blocker / investigation
Canonical ADP WORK.json is optional. Reference fields may be absent and historical project layouts vary; preserve unknowns and report non-applicability, not false errors.

## Next recommended phase
Harden discovery and bounded reference checks, then calibrate against real ADP projects before starting M2 metrics.

## Exact next action
1. Inspect GitHub main HEAD and newest CI result; preserve unrelated changes.
2. Read docs/work/WORK-SCANNER-001.md and existing work/context/version parsers.
3. Implement safe optional contract_file reference resolution constrained to project root; report missing, symlinked, outside-root, and ambiguous targets independently.
4. Add fixtures/tests for bad references and scan-completeness boundaries.
5. Run formatting, lint, unit/integration tests, and build; record verification and update STATUS and this file.

## Decisions made since last checkpoint
- Canonical ADP (2.7.1 baseline) remains unchanged; Observatory must adapt with zero extra required metadata.
- Compatibility dimensions are separate; semantic support currently only covers WORK schema v1.
- Conflicts preserve source references; no silent precedence by timestamp.
- Invalid/unavailable input must not erase independent records.
- Unknown/future versions have no unverified conformance verdict.

## Workspace / Git state
GitHub main is updated via API; local staged/unstaged/untracked state is unavailable remotely.

## Last checkpoint
2026-10-21 — M1 compatibility scanner verified; remaining scanner work recorded.

## Git authorization and recovery
- Initialized: existing GitHub repository.
- Branch: main.
- HEAD: inspect GitHub before resuming, do not trust stale SHAs.
- Staged/unstaged/untracked changes: unknown from remote API.
- Authorization: user requested continued Observatory development; no normative ADP modifications.
- Generated policy cannot grant commits/pushes/history rewrites.
- Next safe action: verify branch and proceed under active M1 contract.
