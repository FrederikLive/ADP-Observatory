# WORK-UI-001 — Local dashboard and prototype distribution

## Outcome
A useful, read-only project visualization launched from a project directory, with headless scan output retained and a tested packaging path toward a trusted one-command installation.

## Status
In progress. The first standalone offline HTML dashboard and CLI modes are implemented and CI-verified; trusted installer, signed official releases, snapshots, and complete metric visualization remain open. Early M3/M4 vertical slice; M1 scanner is still the active project milestone.

## Scope
- Standalone self-contained HTML dashboard showing available ADP records, contracts, source references, conflicts, compatibility and scan limits.
- `adp-observatory .`, `scan . --json`, and `report . --output ...` modes.
- Escape untrusted project values; no external page resources or listening local server.
- Save reports outside target-project directories by default; refuse explicit in-project output.
- Cross-platform experimental CI build/test artifacts and an honest unsigned-release warning.
- Keep the canonical ADP protocol unchanged.

## Acceptance criteria
- [x] Browser-readable, self-contained offline report with project, contract statuses, findings, limitations, and coverage.
- [x] Headless JSON remains functional and backwards-compatible.
- [x] HTML rendering escapes untrusted text; no network dependency or active local server.
- [x] Output refuses writes into the canonical target project, including canonical-path alias cases on tested platforms.
- [x] Windows, macOS, Linux experimental builds and tests pass on GitHub Actions.
- [ ] Safe install-once flow, verifiable publisher identity/signing and official release.
- [ ] Accessible real-browser interaction testing, full dashboard usability review and representative-project pilot.
- [ ] Metric provenance, trends and optional snapshots, if justified.

## Verification evidence
- Rust formatting/lint/test/build: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596011803
- Windows/macOS/Linux test, build, smoke-test, experimental unsigned package: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596011855
- HTML renderer, CLI, report output, security checks: `src/dashboard.rs`, `src/cli.rs`, `src/output.rs`, `tests/scanner.rs`.
- Documentation and later commits must be checked against current HEAD before claiming same-commit verification.

## Risks and known limitations
Unsigned CI artifacts are not official releases or independently authenticated installations. User must obtain a compatible executable or use Rust locally. Reports are offline derived views and may include sensitive local paths. Conformance remains NOT_ASSESSED. Explicit paths must be treated as untrusted.

## Next exact action
Inspect latest CI, decide whether to publish a signed development preview versus continue hardening. Build a safe first-run installation route only after trustworthy artifact provenance/signing and per-platform acceptance tests are available. Do not ship blind curl-to-shell execution.
