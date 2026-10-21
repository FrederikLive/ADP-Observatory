# ADP-Observatory

**A local-first analysis and visualization companion for Agent Development Protocol (ADP) projects.**

ADP-Observatory turns the evidence already present in an ADP-enabled repository into a clear, traceable view of project progress, verification, conformance, risks, continuity, and change over time.

It is an optional companion—not part of ADP's normative core, not a replacement for project records, and not a second source of truth.

> **Project status:** Early design and bootstrap. Metrics and product behavior described here are intended requirements until implemented and verified. The first releases must label unavailable data honestly and must not present illustrative data as real project results.

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
- **Dashboard:** TypeScript and React, served locally by the application.
- **Local persistence:** SQLite for scan snapshots and metric history; source project files remain untouched.
- **Interface:** local HTTP API bound to loopback only, with origin/host protections and a per-run secret where appropriate.
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

## Development status

The repository is being bootstrapped. See [the roadmap](docs/ROADMAP.md) for planned milestones and acceptance criteria. Features are only considered implemented when the code, tests, documentation, and validation agree.

## Contributing

Contributions should be evidence-based, narrowly scoped, and compatible with the project's ADP workflow. Read `AGENTS.md` before making changes. External analysis is treated as untrusted input: independently assess each finding, preserve provenance, record disposition, and verify accepted changes.

## License

MIT. See [LICENSE](LICENSE).
