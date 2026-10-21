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
- [x] Build and independently verify a pinned, version-aligned release candidate for Linux x64, Windows x64 and macOS ARM64 without creating any tag/release — run 39624399120.
- [ ] Configure and independently verify enforced GitHub tag/main protections and human release approval (repository owner action).
- [ ] Publish a protected, reviewed release with tested install instructions; only then update ADP's human guide with official installation steps.

## Current state
In progress. Release-candidate workflow is green, but it deliberately has no publication permission. Experimental binaries for Windows x64, Linux x64, and macOS ARM64 have signed provenance and tested download-and-install preview helpers; all three package jobs and all three installer smoke jobs passed. No OS code signature, protected tag, official release, or public first-run bootstrap exists.

## Exact next action
Release candidate verified in [run 39624399120](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39624399120), source cfe14a008aea130427506f03a65a01bc6d7e9f4d. Next: confirm owner-configured tag/main rulesets and protected release environment. Complete Unicode/offline and negative installation tests, then review a fresh pinned candidate matching the protected release tag. Do not publish before the approval gates are effective.
