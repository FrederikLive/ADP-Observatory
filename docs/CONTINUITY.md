# Continuity

## Active objective
Build a fault-tolerant, local-first ADP project scanner that supports human visualization with zero additional agent or protocol overhead.

## Active milestone
M1 — Discovery and deterministic scanner.

## Active workstream
Analysis engine.

## Active contract
WORK-SCANNER-001 — docs/work/WORK-SCANNER-001.md.

## Current position
Scanner development 0.3.0 adds explicit scan scope via src/discovery.rs and metadata-only checks for optional contract_file references via src/references.rs. src/work.rs reports referenced file statuses independently of structural WORK validity. src/main.rs and schema/scan-report.schema.json expose those results. Tests cover bounded traversal, symlinks, traversal rejection, malformed/missing references, and repository self-scan.

## Last known good state
GitHub Actions run 39575987006 succeeded at its revision for formatting, Clippy, tests, and build. Current branch may contain documentation-only commits after that run; do not assume any unverified code changes are green.

## Current verification
- M0 baseline: VERIFIED — run 39567705133.
- M1 initial compatibility behavior: VERIFIED — run 39573431375.
- M1 reference safety, coverage reporting, self-scan, and version 0.3.0: VERIFIED — run 39575987006.
- Full historical format support, complete conformance, and real-world false-positive calibration: NOT IMPLEMENTED or NOT ASSESSED.

## Human-facing integration checkpoint
- The separate canonical ADP repository now has a non-normative docs/OBSERVATORY.md guide and short README/USING_ADP links; ADP validator passed on documentation-only commit 3dd321e7 (GitHub Actions run 39592315852).
- Observatory docs/LAUNCHER.md defines a future, trusted install-once binary and `adp-observatory .` local-dashboard UX, plus planned headless `scan . --json` command.
- The launcher, signed downloads, and dashboard remain NOT IMPLEMENTED. Avoid copying one-shot examples into docs until supported release artifacts are independently verified. The active M1 scanner contract and exact next action remain unchanged.

## Work in progress
Calibrate parser and scope results against small/mature real ADP projects and grow targeted regression tests. M1 remains in progress.

## Blocker / investigation
No external blocker. Continue to distinguish optional source absence and unsupported semantics from normative ADP violations. Check actual adopted protocol versions in candidate projects.

## Next recommended phase
Complete M1 pilot calibration and source-link robustness before rule/metric expansion.

## Exact next action
1. Inspect GitHub main HEAD/CI and project worktree if local.
2. Read AGENTS.md, docs/STATUS.md, .project/WORK.json, and docs/work/WORK-SCANNER-001.md.
3. Inspect real, representative ADP project layouts against current scanner output and record gaps with evidence.
4. Fix a narrow verified issue, add fixture/tests, run CI quality gates, and update ADP status/continuity.
5. Do not modify canonical ADP unless independently agent-beneficial and authorized.

## Decisions made since last checkpoint
- Project remains optional and ADP-first with no mandatory new metadata or agent reads.
- scan_complete describes policy-bounded traversal; scan_scope counts deliberate exclusions separately from incomplete traversals.
- Contract-file links are checked using safe relative metadata lookups; PRESENT does not imply verification.
- Reference checks have a cap; incomplete inspection is explicitly reported.
- Missing optional WORK.json is unknown, not noncompliance; full conformance is never inferred from schema readability.

## Workspace / Git state
GitHub default branch main updated through connector. Local staged/unstaged/untracked changes cannot be established remotely.

## Last checkpoint
2026-10-21 — M1 scanner hardening and documentation synchronized.

## Git authorization and recovery
- Initialized: existing GitHub repo.
- Branch: main.
- HEAD: inspect current remote; do not rely on stale SHA.
- Staged/unstaged/untracked: unknown remotely.
- Source of authorization: current user's instruction to continue Observatory development.
- Generated policies do not self-authorize commits/push/history rewrites.
- Next safe action: inspect CI and calibrate real project coverage.

## Implemented dashboard vertical slice — 2026-10-21
- Development version 0.4.0: `src/dashboard.rs`, `web/report.html`, `web/report.css`, `web/report.js` render a self-contained offline HTML visualization. `src/cli.rs` adds dashboard/report/scan modes. `src/output.rs` handles path-safe HTML output and browser opener; `src/work.rs` exposes contract summaries.
- Cross-platform tests revealed Windows and macOS canonical path aliases; fixed by canonicalizing the analyzed project before output containment checks.
- Verified code run: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596011803.
- Verified experimental unsigned platform packages: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596011855.
- New related contract: WORK-UI-001. Active contract remains WORK-SCANNER-001; no normative ADP changes.
- No signed release or installer has shipped. A user may use Rust from a checkout; a system-wide `adp-observatory .` requires installing the executable.
- Exact next UI action: evaluate tested publisher verification/signing and installation flow, then run clean-machine manual browser accessibility checks. Do not advertise unsigned artifacts as official trusted downloads.

