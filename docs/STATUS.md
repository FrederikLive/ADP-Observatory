# Project Status

## Current milestone / phase
M1 — Discovery and deterministic scanner. Active Contract: WORK-SCANNER-001 (in_progress).

## Recently completed
- M0 ADP bootstrap: GitHub Actions run 39567705133 passed.
- M1 compatibility baseline and fixture suite: run 39573431375 passed.
- M1 scanner hardening: development 0.3.0; bounded contract-file reference checks, traversal coverage semantics, fixture and self-scan tests. Run 39575987006 passed formatting, lint, tests, and build.
- Canonical ADP stays unchanged. Observatory adds no mandatory token or record overhead to analyzed ADP projects.

## User-facing integration decision
- Added docs/LAUNCHER.md: planned install-once native release and `adp-observatory .` UX, with optional authenticated first-run helper only after safe verification is implemented.
- ADP's human-facing README and docs/OBSERVATORY.md now explain current prototype usage and the proposed launch experience; ADP's normative rules, AGENTS.md, and schemas remain unchanged.
- No release, installed launcher, or dashboard has shipped; do not claim the conceptual command works today.

## In progress
- WORK-SCANNER-001: broaden tested layouts, adversarial inputs, and reference/evidence correctness.
- Assess historical ADP version behavior using real projects, not inferred rules.

## Blocked
No known external blocker. M1 acceptance is not complete.

## Next recommended phase
- Outcome: calibrate scanner against representative real ADP projects, then harden highest-impact mismatches.
- Why next: avoid false positives and misleading visuals before M2 metrics and M3 dashboard.
- First action: check current GitHub Actions run, inspect M1 acceptance criteria, and identify representative project fixtures.

## Known issues
- Only limited WORK.json schema-v1 semantics are supported for explicitly recognized ADP 2.7.1.
- Full ADP conformance remains NOT_ASSESSED.
- Reference PRESENT means a regular file exists; it does not prove content correctness, verification, or completion.
- scan_complete means traversal finished under declared exclusions, not full analysis coverage.
- No dashboard, snapshots, metrics engine, or signed cross-platform releases.
- Local worktree state is not available through remote GitHub API.

## Last verified
- Code checkpoint: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39575987006 — success.
- Follow-up documentation commits: verify latest repository CI before claiming same-SHA validation.

## Git state summary
- Branch: main; updates committed through GitHub API with user authorization to continue.
- No history rewriting, auto-deployment, or canonical ADP modification.

## Offline dashboard checkpoint — development 0.4.0
- The Rust CLI now supports a local offline HTML report from `adp-observatory .`; headless JSON is `adp-observatory scan . --json`. Requires building/obtaining the executable: no signed installer or global command yet.
- All declared WORK schema-v1 contracts are displayed even without optional contract_file references. HTML escapes target-provided text; reports are saved outside the analyzed project.
- Core CLI/dashboard tests passed in Actions run 39596011803; experimental **unsigned** Windows/macOS/Linux package tests passed in run 39596011855.
- WORK-UI-001 tracks early M3/M4 visualization/distribution work. M1 scanner remains active and incomplete.
- ADP's human-only Observatory usage guide was updated; its normative core remains unchanged.

