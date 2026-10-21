# Project Status

## Current milestone / phase
M1 — Discovery and deterministic scanner. Active contract: WORK-SCANNER-001.

## Recently completed
- M0 ADP bootstrap control plane established and verified via GitHub Actions run 39567705133.
- Added a first bounded, read-only parser for .project/WORK.json, structural findings, and unit tests.
- Extended CLI JSON output and report schema for work-index inspection.

## In progress
- WORK-SCANNER-001: parser and test hardening; CI for latest scanner commits pending.

## Blocked
- No known external blocker. Do not mark M1 verified until latest CI succeeds.

## Next recommended phase
- Outcome: validate M1 first vertical slice, then expand parser fixtures and supported ADP layouts.
- Why next: accurate evidence and uncertainty handling must precede metrics/dashboard.
- First action: inspect latest GitHub Actions run; fix failures; add tests for missing work index, duplicate IDs, and hostile/symlinked input.

## Known issues
- Structural validity is not ADP conformance.
- Work parser does not yet validate every optional schema property or detect dependency cycles.
- Scanner remains incomplete: no full record linking, metrics, snapshots, dashboard, or launchers.
- Remote API does not reveal local working-tree state.

## Last verified
- M0: GitHub Actions https://github.com/FrederikLive/ADP-Observatory/actions/runs/39567705133 succeeded.
- M1 latest commit: CI pending.

## Git state summary
- Branch: main.
- Baseline committed; latest scanner changes require verification.
- User authorized continuing project development. No history rewrite or normative ADP change.
