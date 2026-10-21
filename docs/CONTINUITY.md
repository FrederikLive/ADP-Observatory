# Continuity

## Active objective
Enable safe, optional local ADP visualization through a trusted one-command experience while preserving ADP's low-token, agent-first rules and support for incomplete/older project records.

## Current project/milestones/contracts
M1 active work: WORK-SCANNER-001 (in_progress), analysis calibration. M4 parallel: WORK-DISTRIBUTION-001 (in_progress), trusted release and per-user installer. WORK-DASHBOARD-001 verified. WORK-UI-001 deferred/superseded with history retained.

## Last known good state
- Observatory 0.4.0 renders a standalone offline HTML report and offers headless `scan . --json` without network service.
- Experimental platform artifacts are built on Windows x64, Linux x64 and macOS ARM64; GitHub Actions signed provenance is independently verified against the repository, signer workflow and source.
- Preview Bash/PowerShell installation helpers require an explicit successful CI run ID and full SHA, verify downloaded binaries and install per-user without touching the current project. Scripts do not auto-add PATH or overwrite installs.
- Installer smoke workflow runs after a successful package workflow and tests both wrong-SHA rejection and successful install+scan. Full success: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39601029440; package success https://github.com/FrederikLive/ADP-Observatory/actions/runs/39600862632.
- docs/RELEASING.md and docs/LAUNCHER.md explain current preview and official release trust requirements.

## Known gaps / safety
- Experimental installers require a trusted local checkout and GitHub CLI; not an official public first-run installer.
- No configured repository tag ruleset was returned in last ruleset read, and branch protection was inaccessible with current connector permissions. Never state that official releases or tag governance exist.
- Official OS signing/notarization and protected tagged release approval must be separately implemented/verified. A provenance attestation does not imply OS code-signing.
- Full ADP conformance, historical metrics and broad project calibration are still pending; unknown records must never be treated as compliance passes.
- Do not copy binaries/installers into scanned project or modify canonical ADP's normative agent protocol.

## Exact next action
1. Inspect GitHub main HEAD and newest CI run; reconcile documentation with executable state.
2. Continue scanner calibration using representative real ADP projects, including version differences, missing optional records and contradictory statuses, with fixtures and false-positive evidence.
3. Distribution: add end-user, Unicode/spaces and offline tests; ask repository owner to configure protected tags/main and release-environment approvals, then verify via available GitHub APIs/UI.
4. Create protected official release process and signed assets only after governance/installation tests pass. Update optional human ADP guide with official instructions only after release.
5. At transition, update WORK.json, this file, STATUS.md and work contracts with exact evidence, not claims.

## Source of authorization / recovery
User requested continuing Observatory implementation. Remote GitHub API writes to main were used; check current commit and branch on resume. This record cannot authorize unilateral release publication, destructive operations, or modification of ADP normative files.

## Last checkpoint
2026-10-21 — preview install helpers implemented, architecture identified (Linux x64/macOS ARM64/Windows x64), package provenance and three-platform installation CI successfully verified; official release held pending governance.
