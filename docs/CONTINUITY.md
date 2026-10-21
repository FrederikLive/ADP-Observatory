# Continuity

## Active objective
Build a tolerant, version-aware, read-only Observatory that visualizes existing ADP project information without adding agent token or maintenance overhead.

## Active milestone
M1 — Discovery and deterministic scanner.

## Active workstream
Analysis engine.

## Active contract
WORK-SCANNER-001 — docs/work/WORK-SCANNER-001.md.

## Current position
The M0 ADP bootstrap and initial M1 WORK.json structural parser have passing CI runs. A compatibility-first design reassessment has been incorporated into docs/ARCHITECTURE.md, docs/PRODUCT.md, docs/DECISIONS.md, docs/ADP-INTEGRATION.md, docs/ROADMAP.md, and the M1 contract. ADP issue #8 now defers protocol changes.

## Last known good state
GitHub Actions run 39568724881 passed for the initial structural parser. The newer design documentation is not evidence of implemented adapter behavior.

## Current verification
- M0 bootstrap: VERIFIED — run 39567705133.
- M1 structural parser: VERIFIED — run 39568724881.
- Compatibility adapters, independent failure isolation, and full version support: NOT IMPLEMENTED / NOT RUN.
- Documentation updates: committed through GitHub API; inspect current HEAD and CI on resume.

## Work in progress
Implement format compatibility, semantic compatibility, and conformance coverage separately. Support unknown versions and incomplete records without false failures/passes.

## Blocker / investigation
Historical ADP versions and representative project fixtures must be selected based on real formats, not guessed semantics.

## Next recommended phase
Complete M1 compatibility foundations and regression fixtures before M2 rules and metrics.

## Exact next action
Inspect current GitHub HEAD/CI, read docs/ARCHITECTURE.md and the active contract, define compatibility result types, add fixtures for missing WORK.json, malformed input, contradictory state, and unknown/newer versions, then run formatting/lint/tests/build.

## Decisions made since last checkpoint
- ADP remains unchanged unless evidence independently supports agent-beneficial clarification.
- Observatory imposes zero mandatory extra records or agent reads/writes.
- Optional WORK.json remains optional.
- Separate format parsing, semantic interpretation, and conformance coverage.
- Preserve conflicting source claims and partial scan results.
- No silent migration or project mutation.

## Workspace / Git state
GitHub main updated through API. Local staged/unstaged/untracked state is unknown and must be inspected before local work.

## Last checkpoint
2026-10-21 — compatibility-first design decision recorded; executable changes pending.

## Git authorization and recovery
- Initialized: existing GitHub repository.
- Branch: main.
- HEAD: inspect GitHub before resuming; do not rely on stale SHA.
- Staged/unstaged/untracked: not available from remote API.
- Authorization: user requested compatibility-first Observatory updates; no normative ADP amendment.
- Automatic checkpoint commits/push: not granted by generated documents.
- Next safe action: implement M1 adapter/fixture vertical slice with evidence.
