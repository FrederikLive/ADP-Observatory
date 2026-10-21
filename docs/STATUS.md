# Project Status

## Current milestone / phase
M1 — Scanner calibration (active contract: WORK-SCANNER-001). Early M3/M4 slices completed in parallel: offline viewer and experimental cross-platform binaries.

## Verified results
- M0 ADP bootstrap: VERIFIED (CI run 39567705133).
- M1 scanner: bounded read-only discovery, optional schema-v1 work records, version/compatibility evidence, conflicts and safe file references. CI verified (run 39596170547). Full ADP conformance remains NOT_ASSESSED.
- Development 0.4.0 offline HTML viewer: VERIFIED at implementation/test level (run 39596170547); command `adp-observatory .` works with a built executable, `scan . --json` remains headless. No listening web server.
- Windows x64, Linux x64, macOS runner prototype binaries: test/build/smoke/provenance-attestation jobs PASSED (run 39596794393).
- WORK-DASHBOARD-001 verified; original WORK-UI-001 deferred/superseded to avoid duplicate task ownership; WORK-DISTRIBUTION-001 owns secure delivery.

## Not released / not yet verified
- No protected, official OS-code-signed release, trusted automatic installer, or universal first-run download command.
- No full ADP conformance engine, complete evidence graph, historical snapshots or real-project calibration across ADP versions.
- Cross-platform CI success is not a substitute for clean-machine install, browser accessibility and signer verification by users.

## In progress
- WORK-SCANNER-001: real-repository format coverage, false-positive calibration, and adversarial fixtures.
- WORK-DISTRIBUTION-001: authenticated release process, manual verification and install-once experience.

## Next recommended phase
- Outcome: complete M1 scanner calibration, then pilot trusted platform installation.
- Why: maintain useful, honest results without increasing ADP agent overhead or distributing unverified software.
- Exact first action: check current CI/HEAD, select representative ADP projects, record scanner mismatches and fixtures; separately test `gh attestation verify` on each platform artifact and document release protections.

## Last verification and state
- Rust CI: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596170547 — success at that commit.
- Attested packaging matrix: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596794393 — success.
- Documentation-only commits after those runs do not imply additional runtime validation; inspect the newest Actions status.
- Branch: main, GitHub API commits. Local staged/unstaged/untracked state unavailable remotely.
- Canonical ADP rules, schemas, and agent reading policy unchanged; human-only ADP Observatory guide updated.
