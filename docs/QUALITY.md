# Quality Strategy

## Quality goals
Reproducible, secure, read-only analysis with accurate uncertainty semantics, explainable findings, and reliable cross-platform behavior.

## Static checks
Rust formatter and Clippy (warnings as errors).

## Unit tests
Rust cargo test. Existing unit tests cover work-index parsing, optional-field validation, version extraction, safe read limits, symlink refusal, independent context failures, and conflicting claims. Continue adding regression tests for unsupported layouts and edge cases.

## Integration tests
CLI fixture repositories under `tests/fixtures/` cover current, older, future, missing, malformed, and conflicting records. `tests/scanner.rs` checks output, deterministic results, and unchanged target records. Additional tests now cover symlinked and outside-root references, scan-depth and entry-budget limits, oversized exclusions, and an end-to-end self-scan. More large-repository and adversarial race tests remain.

## Contract/schema tests
Validate .project/WORK.json against canonical ADP schema/work.schema.json. Validate scan-report JSON against schema/scan-report.schema.json. Ensure schemas match emitted data and compatibility/version policy.

## End-to-end tests
CLI smoke tests are implemented through `tests/scanner.rs`. Offline HTML dashboard CLI tests cover generated report contents, escaping, output outside the project, headless mode and no-open mode. Real-browser accessibility, offline rendering and platform-specific browser launching still need independent testing.

## Accessibility
Keyboard navigation, semantics, readable contrast, focus management, and accessible result states once dashboard exists.

## Security
No target execution, no automatic upload, bounded scanning, symlink containment, hostile-input fixtures, and local API origin/host protections before dashboard release.

## Performance
Define and test bounded memory/time budgets for representative repositories; record scan completeness when limits are reached.

## Standard quality commands
- Setup: cargo fetch
- Development: cargo run -- <project-directory>
- Formatting: cargo fmt --all -- --check
- Lint: cargo clippy --all-targets --all-features -- -D warnings
- Tests: cargo test --all-targets --all-features
- Build: cargo build --release
- CI-equivalent: run all four quality commands above.

## Required pre-merge gate
Successful formatting, lint, test, and build; schema validation and fixture tests when implemented; reviewed source/documentation alignment.

## Known gaps
Fixtures and unit/integration tests now exist, with bounded contract-file reference inspection and explicit scan coverage. The parser remains a limited ADP 2.7.1 WORK schema-v1 reader. No full conformance engine, complete cross-file evidence linking, comprehensive hostile-input fixture suite, snapshots, production installer, or OS code signing. The initial standalone offline dashboard exists. Full recursive JSON Schema conformance tests are not yet implemented; the integration test checks top-level required report fields. Development 0.4.0 passed Rust CI at https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596170547; Windows/Linux/macOS experimental matrix with signed GitHub build provenance passed at https://github.com/FrederikLive/ADP-Observatory/actions/runs/39597842680.

## Dashboard and cross-platform prototype checkpoint
- Rust formatting, lint, unit/integration tests, and build passed: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596011803.
- Experimental unsigned Windows, macOS, and Linux test/build/smoke/package workflow passed: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596011855.
- Report renderer has HTML-escaping and no-external-assets tests; CLI integration tests cover offline export, no-open mode, and target write refusal.
- The preview binary workflow signs GitHub build provenance and verifies the signer workflow before upload. Windows/Linux/macOS preview installers now pass end-to-end download, attestation verification, install and JSON scan CI tests: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39601029440.
- Still missing: real-browser accessibility review, official protected release and OS code signing, independent end-user install tests, and complete conformance engine.

## Versioned release-candidate checkpoint — 2026-10-21

- `scripts/check_release_candidate.py` validates immutable full SHA, `refs/heads/main`, VERSION, Cargo.toml and JSON report schema version. Positive and negative tests in `tests/test_release_validation.py`.
- The [release candidate workflow 39624399120](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39624399120) passed its source gate, Rust formatting, Clippy, unit/integration tests, release build, and independently verified GitHub/Sigstore binary attestations on Windows x64, Linux x64 and macOS ARM64.
- CI artifacts are explicitly marked `release_state: candidate_only`, are stored for only 14 days, and do not constitute an official released or operating-system-signed application.
- Unicode/spaced-path and offline scan/HTML report checks in `.github/workflows/installer-smoke.yml` **passed** on Windows x64, Linux x64 and macOS ARM64 in [installer run 39660008091](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39660008091). This covers CI environments, not independent user-machine browser visibility.
- GitHub repository tag rulesets were absent at check time. Publishing remains held on [owner release-governance issue #1](https://github.com/FrederikLive/ADP-Observatory/issues/1), independent clean-machine review, and release approval.

## On-demand Agent Skill test checkpoint — 2026-10-21

- `tests/test_agent_launch.py` tests absent or project-local executable refusal, explicit-path safeguards, desktop/headless output, and Unicode paths without downloading or opening a browser.
- `ci.yml` runs those tests and uses `launch.py --binary ... --no-open` with the real compiled Observatory executable, asserting `REPORT_SAVED` and a generated HTML report outside the project.
- First end-to-end verification: [GitHub Actions 39652171472](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39652171472) — success.
- No claim that cloud agents can open a user's local browser, or that the helper authenticates executables. Trusted first-time installation is gated by separate release/approval policy.
- Canonical ADP validation after adding optional discovery references: [ADP validation 39652005147](https://github.com/FrederikLive/ADP/actions/runs/39652005147) — success.

## ADP Actions real-repository compatibility checkpoint — 2026-10-21

- [CI 39659907217](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39659907217) pinned `FrederikLive/ADP` at `9b5ab4694321735e83b93962e1b9975cced130bf`, verified optional router and Actions docs, then scanned the real repository and exported HTML.
- Confirmed known source version 2.7.1, absent optional work index correctly reported as not present / not assessed, and no invented conformance verdict or error for missing optional WORK.
- Confirmed Observatory's own work index stays structurally readable and version uncertainty isn't promoted to a known version solely from a remote reference.
- The dashboard's example ADP Actions are static copyable phrases for humans to ask agents, **not** local commands, enforcement rules or another ADP metadata source.
- This is a pinned-commit smoke test plus existing fixtures, not an exhaustive historical compatibility, conformance or security certification.
