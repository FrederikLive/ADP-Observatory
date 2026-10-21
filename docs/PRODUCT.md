# Product Requirements

**Status:** Initial product baseline. Requirements describe intended behavior, not proof of implementation.

## Vision
ADP-Observatory analyzes a local ADP-enabled project and presents a clear, traceable account of project state, evidence quality, verification, conformance, continuity, external-analysis reconciliation, and change over time. A dashboard must not replace evidence with a score.

## Users
- Project owner: understand delivery state, risk, missing proof, and next actions.
- Reviewer: inspect findings and trace results to sources.
- AI coding agent: consume deterministic machine-readable analysis.
- Maintainer: compare snapshots and identify regressions without fabricated history.

## Primary journey
1. Obtain a release from the official channel.
2. Launch with an OS-appropriate launcher or direct binary.
3. Select a project directory and inspect the resolved path and scan scope.
4. Run a read-only scan with progress and clear error handling.
5. Review overview, findings, metric definitions, and source references.
6. Optionally export a local report.
7. Optionally save a snapshot and compare later scans.

## Functional requirements
- FR-01 Discovery: identify supported ADP markers and relevant files without assuming every project has the same layout.
- FR-02 Compatibility: detect recorded ADP version when possible; report unknown/conflicting versions.
- FR-03 Scope: show paths and formats scanned, excluded, ignored, unsupported, or failed.
- FR-04 Analysis: parse supported records, check consistency, and evaluate applicable rules.
- FR-05 Findings: stable ID, severity, status, explanation, source references, remediation, confidence/limitations, and rule version.
- FR-06 Metrics: documented formula, applicability, inputs, units, time window, limitations, and confidence.
- FR-07 Honest unknowns: missing, unsupported, ambiguous, or incomplete data is not zero, success, or failure by default.
- FR-08 Dashboard: accessible local overview, filtering, detail inspection, and evidence navigation.
- FR-09 History: optional local snapshots; distinguish current-state from trend metrics.
- FR-10 Export: explicit user-triggered local JSON and human-readable report.
- FR-11 Read-only: no target-file modification. Any future write-capable feature must be separately designed, opt-in, auditable, and disabled by default.
- FR-12 Automation interface: documented versioned machine-readable output, usable without the dashboard.
- FR-13 Platform launch: official release artifacts and launchers verify integrity before execution.
- FR-14 Explainability: users can inspect why a rule applies or does not apply and what evidence supports the result.
- FR-15 Failure handling: partial scans remain explicitly partial; parse errors never produce misleading green summaries.

## Non-functional requirements
- Local-first and offline-capable after installation except explicitly networked update checks.
- Deterministic output for the same repository, tool version, configuration, and platform where practical.
- No telemetry or remote upload by default.
- Safe handling of symlinks, large files, malformed input, hostile filenames, and path traversal.
- Accessible UI, keyboard navigation, responsive layout; light-theme-first.
- Report scan duration and skipped work rather than hiding it.
- Versioned schemas for output and persisted snapshots.

## Out of scope for initial release
- Hosted dashboard or mandatory account.
- Autonomous modification of target projects.
- Automatic execution of project scripts, tests, hooks, CI jobs, or arbitrary agent instructions.
- Universal score claiming to quantify project quality without context.
- Automatic normative ADP changes based only on aggregate metrics.
- Guaranteed analysis of every repository format.

## Success measures
Measure finding precision/recall on curated fixtures, percentage of evidence-linked findings, false-positive rate, formula-to-code agreement, scan completeness transparency, reproducibility, time to first useful result, regression detection, and user understanding of unknown states.

## Release gate
Do not call a release production-ready until platform support, artifact integrity, threat model, fixture coverage, false-positive review, accessibility basics, version compatibility, update/uninstall behavior, and machine-readable schema are documented and tested.
