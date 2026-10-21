# Changelog

All notable changes to ADP-Observatory will be documented here.

## [Unreleased]

### Current ADP 2.7.1 / ADP Actions compatibility
- Tested the *actual pinned* latest ADP repository at commit `9b5ab4694321735e83b93962e1b9975cced130bf` (protocol 2.7.1) in Observatory CI, asserting accurate version/optional-WORK/conformance results and HTML generation (run 39659907217).
- Added a small, static ADP Actions reference in the offline report; these are suggested coding-agent prompts, not commands executed by the dashboard, and never require extra project metadata.
- Clarified Agent Skill routing: "Show Observatory" invokes the optional visualization skill; "Show ADP Actions" and "Update ADP" remain separate ADP agent tasks.
- Reconciled offline-viewer security guidance and confirmed cross-platform Unicode/offline installer smoke tests (run 39660008091).

### Optional agent-activated dashboard
- Added a progressively disclosed `adp-observatory` Agent Skill for explicit
  user requests such as "show Observatory", with conditional trust/install
  reference and no mandatory ADP changes.
- Added a standard-library Python launch helper that finds an existing
  approved executable, refuses project-local executable paths, distinguishes
  desktop/headless execution and returns a verified offline report path.
- Added unit and end-to-end CI tests for browser fallback, missing/unsafe
  executables and actual compiled Observatory HTML generation.


### Release-candidate preparation (not published)
- Added a read-only, fail-closed candidate gate checking full source SHA, main branch, Cargo package version, VERSION, and report schema version.
- Added source-pinned multi-platform candidate builds and provenance verification, with temporary versioned artifacts, manifest and checksums, but no GitHub tag/release publication.
- Added negative and positive release-gate unit tests; Windows x64, Linux x64 and macOS ARM64 candidate builds passed (run 39624399120).
- Prepared exact owner-side GitHub branch/tag ruleset and release-approval checklist; no protected public release exists yet.


### Experimental per-user install workflow
- Added pinned GitHub Actions preview installers for Linux x64/macOS ARM64 (Bash) and Windows x64 (PowerShell). Download only the requested run/commit, verify SHA-256 plus signed provenance and workflow identity, and install outside the target project without overwriting existing executables.
- Added follow-on GitHub Actions smoke tests for SHA-mismatch refusal, download, verification, per-user installation, and headless scanning on all three supported CI platforms (run 39601029440).
- Explicitly documented that no official protected/tagged, OS-signed public installer has shipped.


### Added in development baseline 0.4.0
- Standalone offline light-themed HTML project viewer, with no listening server or remote page resources.
- Dashboard/report/scan CLI modes, local contract search, and display of structured contracts even when no contract_file path is provided.
- Escaped project-supplied text and out-of-project HTML report storage with cross-platform canonical-path safety checks.
- Tested Windows/macOS/Linux experimental GitHub Actions binary packages with GitHub/Sigstore-signed build-provenance attestations (not OS code-signed or an official release).
- Versioned JSON report schema 0.4.0. The original WORK-UI-001 umbrella was later deferred and split into WORK-DASHBOARD-001 (verified offline viewer) and WORK-DISTRIBUTION-001 (in-progress authenticated releases).


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
- Full ADP conformance evaluation, historical metrics, snapshot comparison, platform installer, and official signed release remain unimplemented.
- The initial offline dashboard, limited structural analysis, unit/CLI tests and experimental provenance-attested binaries are implemented; platform code signing and real-browser accessibility review remain outstanding.
