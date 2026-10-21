# WORK-DISTRIBUTION-001 — Authenticated cross-platform distribution

## Outcome
Make a local-first Observable executable obtainable on supported platforms without requiring a Rust toolchain, and establish trustworthy publisher provenance before an automatic download-and-run workflow.

## Scope
- Independently tested Windows, Linux, and macOS executable artifacts.
- Cryptographically verifiable build provenance tied to repository and commit.
- Clear provenance verification steps and platform compatibility notices.
- Future: install-once per-user launcher that works from any ADP project directory.

## Out of scope for current prototype
- Blind remote-shell installers; root/admin requirements; automatic update; unsigned auto-execution.
- Publishing an official release before protected tag/release and installation verification.
- Changing canonical ADP rules or introducing a dashboard-specific project manifest.

## Acceptance criteria
- [x] Matrix tests, builds and smoke checks for Windows x64, Linux x64 and macOS runner on 2026-10-21; run 39596011855.
- [x] Attach GitHub/Sigstore-signed build provenance to each experimental packaged binary; all three matrix jobs passed (run 39596794393).
- [x] Verify each attested executable against the exact repository and signer workflow before artifact upload; Windows, Linux and macOS matrix steps all passed (run 39597842680).
- [x] Document checksum vs signed provenance, exact manual verification and preview install commands in docs/RELEASING.md and docs/LAUNCHER.md.
- [x] CI test pinned download, signed provenance verification, per-user installation and headless scan on Windows x64, Linux x64 and macOS ARM64; see run 39601029440.
- [ ] Independently test end-user machines, actual PATH integration, offline behavior after installation, unsupported platforms, Unicode/spaced paths, update/uninstall and malicious downloads.
- [ ] Publish protected, verified tagged release and tested install instructions; only then update ADP human guidance.

## Current state
In progress. Experimental binaries for Windows x64, Linux x64, and macOS ARM64 have signed provenance and tested download-and-install preview helpers; all three package jobs and all three installer smoke jobs passed. No OS code signature, protected tag, official release, or public first-run bootstrap exists.

## Exact next action
After preview installer CI success (39601029440), complete end-user validation and configure enforceable release/tag protections. Prepare a reviewed protected-tag release with authentic asset verification; do not publish or auto-execute unverified downloads.
