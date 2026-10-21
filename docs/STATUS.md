# Project Status

## Current milestone / phase
M1 — scanner calibration (WORK-SCANNER-001 active), with a separately tracked M4 preview-distribution slice (WORK-DISTRIBUTION-001 in_progress). WORK-DASHBOARD-001 is verified; WORK-UI-001 superseded as historical work.

## Verified outcomes
- ADP bootstrap, read-only scanner, compatibility/missing-data semantics and offline HTML dashboard: code-level CI verified (e.g. run 39596170547).
- `adp-observatory .`, `adp-observatory scan . --json`, `report . --output ...` work in development executable version 0.4.0.
- Windows x64, Linux x64 and macOS ARM64 package matrix builds/tests, signs provenance and validates workflow/source: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39600862632 — success.
- Experimental per-user installers `scripts/install-preview.ps1` and `scripts/install-preview.sh` tested on the three operating systems: incorrect SHA rejected, pinned artifact downloaded, SHA-256 and signed provenance verified, binary installed outside target project, and headless scan passed. https://github.com/FrederikLive/ADP-Observatory/actions/runs/39601029440 — success.
- No normative changes to canonical ADP, no required project metadata, agent instructions, or telemetry.

## Not yet released / known limitations
- No official tag-protected GitHub release, OS code-signed installer, no-prerequisite automatic first-run downloader, or automatic PATH setup.
- Per-user installation currently requires a locally reviewed script and independently installed GitHub CLI, with explicit run ID and full source SHA. See docs/RELEASING.md.
- No complete conformance engine, real-project version calibration, live analysis dashboard or historical snapshots.
- GitHub repository rulesets were empty at last check; branch protection details could not be read via the integration. Owner must configure and verify required governance before official release.

## Active work and next recommended phase
- M1: test scanner against real ADP projects with missing/older/contradictory records and quantify false positives.
- M4: secure official release design, end-user installation/Unicode/offline tests and update/uninstall semantics.
- Outcome sought: a protected signed-provenance release users can install once and invoke by `adp-observatory .`.
- Exact first action: inspect current HEAD/CI, then test clean-machine installer behavior and configure verified tag/branch protection and release approval gates before publishing any official release.

## Latest verification / Git state
- Installer smoke: 39601029440 — VERIFIED at CI runner level.
- Package matrix: 39600862632 — VERIFIED.
- Documentation-only commits after these runs should not be confused with a new binary test.
- Branch: main via authorized GitHub connector. Remote API does not reveal local dirty/untracked changes. No destructive Git operation or release publication performed.
