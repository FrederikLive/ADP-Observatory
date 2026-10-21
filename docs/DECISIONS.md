# Decisions and Open Questions

## Accepted baseline decisions
- **D-001 Optional companion:** Observatory is not required for ADP conformance and cannot override canonical ADP records.
- **D-002 Local-first:** no mandatory account, hosted service, or telemetry.
- **D-003 Read-only:** scans do not modify target projects by default.
- **D-004 Honest uncertainty:** unknown/partial/not-assessed are not zero or pass.
- **D-005 Traceability:** metrics/findings reference evidence and disclose limitations.
- **D-006 Derived history:** snapshots are optional local data, not project truth; no trend without a baseline.
- **D-007 Verify before execute:** launchers verify official artifacts; no blind shell piping.
- **D-008 Separate ADP changes:** normative amendments are proposed/reviewed in the canonical ADP repository.

## Open decisions
- **Q-001 Core language:** validate Rust against packaging and maintenance needs.
- **Q-002 Dashboard framework:** choose the smallest maintainable UI after a vertical slice.
- **Q-003 Supported ADP layouts:** define with fixtures and real repositories.
- **Q-004 Platform matrix:** decide OS/architectures based on CI and release feasibility.
- **Q-005 Metric configuration:** how to express project-specific policy without forcing a new ADP manifest.
- **Q-006 Release signing:** choose a maintainable signing/provenance process.
- **Q-007 Data location/retention:** specify paths, migration, export, and deletion.
- **Q-008 Launcher UX:** compare signed binaries, PowerShell, batch, and shell wrappers; never execute unverified downloads.
- **Q-009 Initial rules:** prioritize correctness, applicability, and low false-positive risk.
