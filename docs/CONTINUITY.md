# Continuity

## Active objective
Adapt ADP-Observatory into a fully ADP-governed project without replacing its existing work.

## Active milestone
M0 — Repository foundation and ADP bootstrap.

## Active workstream
Project control plane / verification foundations.

## Active contract
WORK-BOOTSTRAP-001 (see docs/work/WORK-BOOTSTRAP-001.md).

## Current position
Existing product documents, Rust discovery CLI, schema, and CI are present. ADP control-plane files are being established. The project is a companion to, not an amendment of, canonical ADP.

## Last known good state
GitHub content API confirmed initial repository commits. Previous CI runs had formatting failures; code formatting was corrected but a green CI result has not yet been established in this continuity record.

## Current verification
- Source files committed through GitHub API: VERIFIED (write responses).
- Rust formatting, lint, test, build for latest commit: NOT RUN / CI PENDING until GitHub Actions completes.
- ADP bootstrap self-review: IN PROGRESS; do not claim full conformance before reviewing applicable ADP requirements and evidence.

## Work in progress
Create STATUS, CONTINUITY, WORK.json, work contract, QUALITY, and an operational AGENTS router. Confirm repository consistency.

## Blocker / investigation
Check whether the project should embed a pinned ADP submodule, or use a remote canonical reference. Current integration uses the latter to avoid vendoring normative text and modifying Git submodule state through file-only API.

## Next recommended phase
Finish M0 verification, then move to M1 deterministic parser and fixture testing.

## Exact next action
Read root AGENTS.md, docs/STATUS.md, .project/WORK.json, and docs/work/WORK-BOOTSTRAP-001.md; inspect latest CI; run or inspect quality gates and update evidence/status.

## Decisions made since last checkpoint
- Canonical ADP is FrederikLive/ADP, version 2.7.1 at bootstrap.
- Root AGENTS.md is the router, not a substitute for the normative ADP specification.
- Existing project documentation remains canonical for its individual concern; avoid duplication.
- Do not assert verification without actual test results.

## Workspace / Git state
GitHub main branch, updated via authenticated API. Working-tree cleanliness cannot be inferred from the API; inspect before local work.

## Last checkpoint
2026-10-21 — ADP bootstrap adaptation.

## Git authorization and recovery
- Initialized: yes, existing GitHub repository.
- Branch: main.
- HEAD: inspect GitHub before resuming; do not rely on a stale SHA.
- Staged / unstaged / untracked: unavailable from remote API.
- Commit authorization: user requested repository bootstrap in this conversation.
- Automatic checkpoint commits: not separately authorized by generated policy.
- Push authorization: no standing permission inferred from this file.
- Next safe action: inspect CI and reconcile status/contract with evidence.
