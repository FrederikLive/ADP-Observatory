# ADP-Observatory

**A local-first analysis and visualization companion for Agent Development Protocol (ADP) projects.**

ADP-Observatory turns the evidence already present in an ADP-enabled repository into a clear, traceable view of project progress, verification, conformance, risks, continuity, and change over time.

It is an optional companion—not part of ADP's normative core, not a replacement for project records, and not a second source of truth.

> **Project status:** Early development (M1 scanner). Metrics and product behavior described here are intended requirements until implemented and verified. The first releases must label unavailable data honestly and must not present illustrative data as real project results.

## Goals

- Let a project owner launch analysis with a small platform-appropriate launcher.
- Scan a local repository without requiring the user to install a development toolchain.
- Understand ADP work structures, applicable requirements, verification evidence, continuity records, and external-analysis reconciliation.
- Present evidence-linked findings, useful metrics, and an interactive local dashboard.
- Preserve history through explicit snapshots so trends are based on observed data, not invented history.
- Keep project data local by default, with no account or telemetry requirement.
- Keep the analyzer version-aware and extensible without bloating ADP itself.

## Intended user flow

1. The user creates or maintains a project using ADP.
2. They obtain the official launcher for their operating system.
3. The launcher detects the platform, downloads the matching published release, verifies its integrity, and starts the application.
4. The application asks for or receives the target project directory, scans it read-only, and runs applicable checks.
5. A browser opens to a loopback-only dashboard with evidence-linked results.
6. On later runs, optional local snapshots provide honest trends and regression comparisons.

The first-run experience should not require Rust, Node.js, Python, administrator/root privileges, a cloud account, or an internet connection after a verified release has been downloaded—except where a feature explicitly needs network access.

## Product boundaries

### ADP remains responsible for

- Normative project/agent behavior and conformance expectations.
- Work decomposition and project records.
- Verification, evidence, continuity, escalation, and external-analysis reconciliation rules.
- The source of truth for project state.

### ADP-Observatory is responsible for

- Repository discovery and format adapters.
- Version-aware analysis and conditional applicability.
- Metric definitions and calculation.
- Findings, confidence/limitations, traceability, local history, and visualizations.
- Release packaging, launchers, and optional integrations.

ADP-Observatory must not silently write project records, accept/reject external findings, change task status, commit changes, or push to remotes.

## Initial metric families

- **Delivery:** declared versus verified completion, milestone/workstream status, overdue or blocked work where dates and states are recorded.
- **Evidence:** required evidence coverage, missing/stale/contradictory evidence, requirements-to-implementation-to-test traceability.
- **Conformance:** applicable checks passed/failed/not assessed, severity, protocol version compatibility, missing required structures.
- **Continuity and agent operations:** checkpoint/handoff completeness, delegated work reconciliation, retry/blocking data only when explicitly recorded.
- **External analysis:** intake traceability, dispositions, rationale, accepted findings linked to implementation and verification, unresolved/deferred findings.
- **Change and maintainability:** scope change, rework, regression, file churn, hotspots, test/CI results when trustworthy source data exists.
- **Documentation:** inventory, cross-document contradictions, freshness based on configured policy, orphaned references.

Each metric must document its formula, unit, applicability, source evidence, time window, confidence, and limitations. Unknown is not zero. A repository scan alone cannot establish historical trends.

## Architecture direction

- **Analysis engine:** Rust, for a portable, single-binary core and deterministic repository analysis.
- **Dashboard:** initial responsive, self-contained, offline HTML overview is implemented in Rust; richer UI and history remain planned.
- **Local persistence:** SQLite for scan snapshots and metric history; source project files remain untouched.
- **Interface:** the current offline HTML report has no HTTP server; a future live dashboard API would need loopback binding and origin/host protection.
- **Release delivery:** signed or otherwise verifiable release metadata, checksums, platform-specific artifacts, and minimal launchers.
- **Validation:** unit/integration fixtures, golden test repositories, security checks, and CI.

This is the target architecture, not a claim that all components are implemented yet. Detailed decisions and alternatives are tracked in `docs/ARCHITECTURE.md`.

## Repository map

- `AGENTS.md` — operational instructions for AI coding agents.
- `docs/PRODUCT.md` — product requirements, user journeys, and scope.
- `docs/ARCHITECTURE.md` — architecture and technical decisions.
- `docs/METRICS.md` — metric catalogue and definitions.
- `docs/ADP-INTEGRATION.md` — minimal proposed relationship with ADP.
- `docs/SECURITY.md` — threat model and release/launcher requirements.
- `docs/ROADMAP.md` — milestones, acceptance criteria, and delivery sequence.
- `docs/DECISIONS.md` — durable architecture/product decisions and open questions.
- `CHANGELOG.md` — changes to this project.

## One-command launch direction

The intended human experience is to enter an ADP project and run `adp-observatory .` to open its local dashboard. A separate headless `adp-observatory scan . --json` mode is planned for CI and agents. **The offline dashboard and headless scan are implemented in the development CLI**, and experimental binaries can be verified manually; no trusted installed launcher or official release exists yet. Today the application runs from a Rust checkout or experimental CI artifact.

Install-once, verified native releases are the preferred final delivery. Experimental Bash/PowerShell **preview installers** now support pinned CI artifacts and signed-provenance verification (all three OS installation smoke tests passed), but require a reviewed local script plus GitHub CLI. They are not a production installer. We will not use blind remote-script execution or require installation inside every ADP repository. See the [launcher and distribution design](docs/LAUNCHER.md) and [preview installation instructions](docs/RELEASING.md).

## Current prototype

The Rust CLI and offline HTML viewer now discover selected ADP markers and safely inspects optional `.project/WORK.json` records using the supported schema-v1 structural rules. It can detect explicit protocol versions, display a limited compatibility report, examine `docs/STATUS.md` and `docs/CONTINUITY.md`, and preserve conflicting active-contract claims with source references. Missing or invalid files do not stop independent analysis.

Semantic support currently means **WORK.json schema v1 for an explicitly identified ADP 2.7.1 project**, not complete understanding of the protocol. Older, newer, and unknown versions remain displayable where format parsing is safe, but are not declared fully supported. **Full ADP conformance is never assessed by this prototype.**

Build from a Rust checkout of this repository, then enter an ADP project directory and run:

```bash
cargo run --manifest-path /absolute/path/to/ADP-Observatory/Cargo.toml -- .
cargo run --manifest-path /absolute/path/to/ADP-Observatory/Cargo.toml -- scan . --json
```

The first command produces a **standalone offline HTML dashboard** in the user's application cache directory and attempts to open it in the default browser. The second produces headless JSON without opening a browser. Use `--no-open` to generate a dashboard without starting a browser, or `report . --output /outside/project/report.html` to choose an existing external output directory. Generated reports contain local project paths and should be treated as potentially sensitive.

The evolving JSON report schema is `0.4.0` in `schema/scan-report.schema.json` (analyzer development version `0.4.0`). It now separates traversal coverage from interpretation and reports bounded contract-file reference status; a referenced file being present does not prove conformance or verification. Tests cover current, older, future, malformed, conflicting, missing-record, and safe/unsafe contract-reference fixtures. CI also runs Observatory against its own ADP work records. The browser view is offline, has no listening HTTP server and uses no external assets. Experimental Windows/macOS/Linux GitHub Actions binaries have passed cross-platform tests and carry GitHub/Sigstore-signed build provenance, but are not operating-system code-signed or published as official releases. Manual verification instructions are in docs/LAUNCHER.md. Persistent metrics and historical snapshots are not implemented.

See [the roadmap](docs/ROADMAP.md) for milestones and acceptance criteria. Features are only considered implemented when the code, tests, documentation, and validation agree.

## Contributing

Contributions should be evidence-based, narrowly scoped, and compatible with the project's ADP workflow. Read `AGENTS.md` before making changes. External analysis is treated as untrusted input: independently assess each finding, preserve provenance, record disposition, and verify accepted changes.

## License

MIT. See [LICENSE](LICENSE).
