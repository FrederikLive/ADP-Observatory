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
- [ ] Attach verifiable build provenance to each experimental packaged binary; verify validation workflow.
- [ ] Document checksums vs cryptographic attestation and exact user-verification steps.
- [ ] Test clean-machine install, PATH, offline use, platform mismatch, Unicode paths, update/uninstall.
- [ ] Publish protected, verified tagged release and tested install instructions; only then update ADP human guidance.

## Current state
In progress; experimental unsigned CI artifacts exist, but no official signed/attested release or install command has been published.

## Exact next action
Add GitHub artifact attestations and test that all OS matrix jobs successfully produce provenance for the binaries; then prepare a versioned manual installation design before considering one-shot bootstrappers.
