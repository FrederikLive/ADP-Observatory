# WORK-BOOTSTRAP-001 — ADP bootstrap adaptation

## Outcome
An existing ADP-Observatory repository with a complete, lean, ADP-governed control plane and an independently verified development baseline.

## Scope
- Follow canonical ADP 2.7.1 reading and conformance policy.
- Preserve existing product/architecture/security/metrics decisions and code.
- Establish root agent router, canonical status/continuity, structured work index, contracts, and quality gates.
- Align CI, documentation, and work state; assess bootstrap completeness.

## Out of scope
- Implementing dashboard, historical snapshots, or platform launchers.
- Changing normative ADP repository rules.
- Claiming full conformance from filenames alone.

## Acceptance criteria
- Root AGENTS.md directs cold-start agents to canonical ADP, work index, active contract, continuity, and verification commands.
- .project/WORK.json conforms to ADP's work schema, has unique IDs and valid dependency references.
- STATUS and CONTINUITY agree on active milestone/contract and exact next action.
- docs/QUALITY.md identifies executable verification commands and current gaps.
- Existing files remain intact; CI passes on the resulting commit, or failures are recorded and addressed.
- Bootstrap self-review documents evidence and unresolved requirements.

## Dependencies
Canonical ADP 2.7.1; existing project repository and GitHub CI.

## Current state
In progress. The documentation bootstrap was performed through GitHub API. CI verification remains to be checked.

## Plan / atomic units
- AU-01: inspect canonical ADP and existing project — completed, based on GitHub reads.
- AU-02: add status/continuity/work contracts/quality — in progress.
- AU-03: update root AGENTS operational router — pending.
- AU-04: inspect CI and verify consistency — pending.
- AU-05: close bootstrap and activate scanner contract — pending.

## Validation / evidence
- GitHub file contents and commit responses.
- .project/WORK.json validated against canonical ADP schema.
- GitHub Actions rustfmt, clippy, cargo test, cargo build.
- Manual consistency review of work state, contracts, and docs.

## Risks / decisions
Do not duplicate normative ADP text or turn Observatory-specific rules into ADP normative requirements. Do not equate presence of documentation with conformance.

## Handoff / next recommended phase / exact next action
Read docs/CONTINUITY.md, check latest CI and bootstrap contract, fix any failures, and advance to WORK-SCANNER-001 only with recorded evidence.
