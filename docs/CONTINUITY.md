# Continuity

## Active objective
Build a reliable, optional, local-first Observatory that lets humans visualize existing ADP evidence without requiring additional agent tokens, records, or project mutations.

## Active milestone / workstream / contract
M1 — analysis engine — WORK-SCANNER-001 (in_progress). Parallel experimental distribution: WORK-DISTRIBUTION-001 (in_progress). Initial offline UI: WORK-DASHBOARD-001 (verified). Original WORK-UI-001 umbrella retained as deferred/superseded historical work.

## Last known good state
- Development 0.4.0 implements `adp-observatory .`, `report . --output ...` and headless `scan . --json` with standalone offline HTML output.
- Analyzer reports declared work contracts, referenced files, context/conflicts, source-backed limits and three distinct compatibility dimensions. No full ADP conformance verdict.
- Output is generated outside the analyzed project; HTML is escaped and has no external page resources/listening server.
- Rust formatting, lint, tests and build: VERIFIED in CI run 39596170547.
- Experimental Windows x64/Linux x64/macOS binaries with GitHub/Sigstore-signed build provenance: VERIFIED at CI job level, run 39596794393. No official tagged release or OS code signature.

## Unfinished work
- M1: representative real-ADP-project pilots; version/layout coverage; false positives; hostile-input and scale tests; deeper source linking.
- M3: historical snapshots, richer evidence navigation, browser accessibility.
- M4: clean-machine/manual attestation verification, protected tagged releases, platform-specific trusted install/update/uninstall. Do not ship blind download-to-shell.
- ADP's canonical normative files remain unchanged; ADP docs/OBSERVATORY.md is optional human documentation only.

## Exact next action
1. Inspect GitHub default-branch HEAD, latest CI, work index and active contracts. Preserve unrelated changes.
2. Test the scanner on representative ADP projects and record supported/unsupported evidence. Fix the narrowest reproducible issue with fixtures.
3. For distribution, download a pinned CI artifact and independently run `gh attestation verify <binary> --repo FrederikLive/ADP-Observatory` on clean supported machines; do not claim code signing or publish an installer prematurely.
4. Run format, lint, unit/integration tests and platform package jobs for modified code. Update this file and STATUS with exact evidence.

## Decisions
- ADP optimizes agents and low token use; Observatory adapts to optional, imperfect, and version-diverse records.
- Presence of a referenced file is not verification. Unknown, partial, and unsupported data never become zero or a passing conformance result.
- The first dashboard is a generated offline report, avoiding an unnecessary HTTP server.
- Signed build provenance identifies GitHub build identity but does not automatically authorize trusting a downloaded binary, nor replace OS code signing.
- Separate verified dashboard slice and in-progress distribution contract; retain superseded umbrella as history.

## Git recovery / authorization
Branch main, updates through GitHub connector under user's continue request. Inspect HEAD before changes. Local worktree cannot be inferred via API. Generated policy is not standing authorization for destructive operations, deployment, or release publication.

## Last checkpoint
2026-10-21 — offline dashboard 0.4.0, platform-attested experimental binaries, ADP work-contract reconciliation, and human usage guidance.
