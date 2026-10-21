# Project Status

## Current milestone / phase
M1 — Discovery and deterministic scanner. Active contract: WORK-SCANNER-001.

## Recently completed
- M0 ADP bootstrap verified in run 39567705133.
- M1 first compatibility scanner baseline VERIFIED in run 39573431375.
- Source-file based version detection and sourced version-conflict reporting.
- WORK.json schema-v1 core/optional field inspection with future schema degradation.
- Independent STATUS/CONTINUITY inspection, source claims, active-contract conflict reporting.
- Unit tests and CLI fixtures for current, older, future, missing, malformed, and inconsistent records.
- JSON report and analyzer development version aligned to 0.2.0.
- Canonical ADP remains unchanged; ADP issue #8 is evidence-gated.

## In progress
WORK-SCANNER-001: scanner completeness rules, bounded references to contract files, broader hostile-input tests, and real-project calibration.

## Blocked
No external blocker identified. Do not claim the full M1 scanner is complete.

## Next recommended phase
- Outcome: harden discovery and resolve only safe, explicitly referenced contract-file paths.
- Why next: reliable scan boundaries and evidence links are required before meaningful metrics or dashboard visualizations.
- First action: inspect latest GitHub Actions run; implement file-reference checks with missing/symlinked/outside-root fixtures and honest scan-completeness reporting.

## Known issues
- Semantic support is scoped to WORK_JSON_SCHEMA_V1_ONLY on explicitly recognized ADP 2.7.1; older/future versions are conservatively unsupported.
- Conformance remains NOT_ASSESSED for all projects. Structural validity is not normative ADP conformance.
- Complete cross-file evidence linking, version adapters, snapshots, dashboard, metrics, and safe release launchers are not implemented.
- Remote API access does not reveal an agent's local working-tree changes.

## Last verified
- M0 bootstrap: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39567705133 — success.
- M1 compatibility implementation and CLI fixture suite: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39573431375 — success.
- Documentation-only commits after that run need not be misrepresented as code-level test evidence.

## Git state summary
- Branch: main.
- Commits made through authorized GitHub connector; no normative ADP changes.
