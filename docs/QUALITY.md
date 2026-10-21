# Quality Strategy

## Quality goals
Reproducible, secure, read-only analysis with accurate uncertainty semantics, explainable findings, and reliable cross-platform behavior.

## Static checks
Rust formatter and Clippy (warnings as errors).

## Unit tests
Rust cargo test. Add tests for parser/rule semantics, unknown states, and deterministic results.

## Integration tests
Fixture repositories for valid, incomplete, contradictory, unsupported, and malicious inputs. Verify that the analyzer does not modify target files or execute target code.

## Contract/schema tests
Validate .project/WORK.json against canonical ADP schema/work.schema.json. Validate scan-report JSON against schema/scan-report.schema.json. Ensure schemas match emitted data and compatibility/version policy.

## End-to-end tests
Future local CLI and dashboard smoke tests, including path escaping, lifecycle, and offline behavior.

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
No fixtures, comprehensive parser, conformance rule engine, snapshot comparison, dashboard, platform builds, or stable release signing. The initial scanner has no test coverage yet. CI must be checked after bootstrap changes.
