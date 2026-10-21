# Changelog

All notable changes to ADP-Observatory will be documented here.

## [Unreleased]

### Added in development baseline 0.4.0
- Standalone offline light-themed HTML project viewer, with no listening server or remote page resources.
- Dashboard/report/scan CLI modes, local contract search, and display of structured contracts even when no contract_file path is provided.
- Escaped project-supplied text and out-of-project HTML report storage with cross-platform canonical-path safety checks.
- Experimental, unsigned Windows/macOS/Linux GitHub Actions packages and tests (not official signed releases).
- Versioned JSON report schema 0.4.0 and ADP work contract WORK-UI-001.



### Added in development baseline 0.3.0
- Bounded, read-only inspection of explicit contract-file paths with missing/unsafe reference states.
- Explicit scan scope: complete, complete with exclusions, or partial, with separately counted errors and exclusions.
- Tests for links, traversal limits, symbolic links, and self-analysis of Observatory's ADP work records.
- Expanded pre-stable report schema 0.3.0; no full ADP conformance claims.


### Added in development baseline 0.2.0
- Explicit ADP protocol-version discovery with source provenance and version-conflict reporting.
- Independent format, semantic, and conformance compatibility states (conformance remains unassessed).
- Best-effort STATUS/CONTINUITY record inspection with independent error isolation and source-linked conflict reports.
- Tolerant handling of future WORK.json schema versions without falsely marking them invalid.
- Additional tests for missing records, malformed input, conflicting evidence, unknown versions, symlinks, and read-only operation.
- Evolving scan report schema 0.2.0; not yet a stable public contract.


### Added
- Initial product, architecture, security, metrics, ADP integration proposal, decisions, and roadmap.
- Agent operating instructions, MIT license, contributor guidance, and CI workflow.
- Rust CLI discovery prototype with text and JSON output.
- Initial scan-report JSON schema.

### Limitations
- The scanner recognizes a small set of ADP-related filenames only; it does not evaluate ADP conformance.
- Dashboard, metric calculations, persistent history, platform launchers, and release signing are not implemented.
- CI has been configured but has not yet been confirmed green.
