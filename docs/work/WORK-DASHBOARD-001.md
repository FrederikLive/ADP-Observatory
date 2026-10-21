# WORK-DASHBOARD-001 — Offline human-facing project overview

## Outcome
Provide a useful optional visualization of supported existing ADP records without introducing agent overhead, target-project modifications, or a network service.

## Scope
- Self-contained light-themed HTML report with contract list, statuses, dependencies, file-reference states, conformance limitations, source conflicts, and scan coverage.
- Headless `scan . --json` remains available; `adp-observatory .` generates and opens a report when possible.
- Safe escaping and strict offline Content Security Policy, local filter, reports written outside the analyzed project.
- No promises of complete ADP conformance or persistent history.

## Excluded
- SQLite snapshots, time-series metrics, multi-user account, cloud uploads, HTTP API/server, installer, official release.

## Acceptance and evidence
- [x] Generate self-contained offline report for a project with supported records.
- [x] Display all structured work contracts even without linked contract files.
- [x] Preserve missing, malformed, unsupported, and conflicting states without fabricated passing verdicts.
- [x] Refuse report output within target project, escape untrusted HTML, and avoid external network resources.
- [x] Keep JSON scan mode and legacy positional JSON shortcut.
- [x] Pass Rust formatter, Clippy, tests, and release build.
- [x] Confirm current code-level CI on 2026-10-21, run 39596170547.

## Verification scope
**VERIFIED for the initial offline viewer only.** The web UI is a generated standalone report, not a live dashboard; source navigation, complete historical metrics, snapshots, a release installer, and full conformance evaluation remain unimplemented.

## Handoff
The M3 interactive/history scope remains open. Next add only evidence-based metrics and snapshots once M1 scanner calibration supports them; do not add mandatory files to ADP projects.
