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
CLI smoke tests are implemented through `tests/scanner.rs`. Future dashboard smoke tests must cover lifecycle, path escaping, and offline behavior.

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
Fixtures and unit/integration tests now exist, with bounded contract-file reference inspection and explicit scan coverage. The parser remains a limited ADP 2.7.1 WORK schema-v1 reader. No full conformance engine, complete cross-file evidence linking, comprehensive hostile-input fixture suite, snapshots, dashboard, released platform binaries, or signing. Full recursive JSON Schema conformance tests are not yet implemented; the integration test checks top-level required report fields. CI for the 0.3.0 scanner implementation and tests passed in GitHub Actions run 39575987006; check the most recent repository CI after subsequent documentation updates.

## Dashboard and cross-platform prototype checkpoint
- Rust formatting, lint, unit/integration tests, and build passed: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596011803.
- Experimental unsigned Windows, macOS, and Linux test/build/smoke/package workflow passed: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39596011855.
- Report renderer has HTML-escaping and no-external-assets tests; CLI integration tests cover offline export, no-open mode, and target write refusal.
- No real-browser accessibility testing, signed publisher identity, installation verification, or complete conformance engine yet.

