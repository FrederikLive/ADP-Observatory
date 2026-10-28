# DESIGN — ADP Observatory

**Approved design direction (2026-10-28):** Dashboard V2 / Project Intelligence. The director selected this direction after reviewing the interactive local HTML mock. **Design approval is not code or release acceptance.**

## Source of truth and artifacts

- [Interactive standalone HTML prototype](design_examples/observatory-dashboard-v2.html). All example project names, counts, percentages and trend data are synthetic.
- [Research and metric semantics](docs/research/DASHBOARD-V2-RESEARCH.md), referencing the existing [metric catalogue](docs/METRICS.md) and [Contract & Evidence Explorer specification](docs/work/WORK-EXPLORER-001.md).
- Existing production report remains in `web/report.html`, `web/report.css`, `web/report.js` and `src/dashboard.rs` until an implementation PR is tested and merged.

## Approved product and interaction decisions

1. **Professional light workspace:** wide, responsive canvas; dark but discreet navigation rail; high information density without lengthy repeated full-width contract cards. Keep source disclosure and limitations visible.
2. **Headline scorecard:** four differentiated metrics: declared work completion, recorded active work, source/reference coverage *when measurable*, and explicitly recorded blockers. Every percentage needs numerator, denominator, rule version, source and exclusions. **Never infer verified completion from a `verified` status claim.**
3. **Charts:** mutually exclusive recorded-state distribution and workstream comparisons from genuine project records. Display history trends **only when real compatible saved snapshots exist**; on a first scan show a truthful empty state rather than fabricated historical points.
4. **Contract & Evidence Explorer:** searchable, filterable compact list with keyboard-accessible details. Later show bounded Markdown outlines, exact source lines and evidence relationships according to the Explorer's independently reviewed data model.
5. **Trust boundaries:** preserve `KNOWN`, `PARTIAL`, `UNKNOWN`, `NOT_APPLICABLE`, `NOT_ASSESSED`; distinguish status declared by the project, metadata observed by the scanner, and independent test/verification evidence.
6. **Read-only offline report:** one self-contained HTML file, strict CSP, inert escaped project content; no CDN, remote fonts, telemetry, automatic project Git changes, mandatory ADP fields or always-on local service. Generate only outside the analyzed project.
7. **Accessibility and performance:** responsive at narrow and wide widths; keyboard selection and focus; charts have readable counts/text equivalents; 30/300 contracts tested under bounded output and no target mutation.

## Production implementation order

- **Slice 1 — reliable measurements:** guarded recorded-completion formula, explicit state partitions, workstreams and limits. Invalid/unsupported work indexes show `UNKNOWN`, not a precise progress value.
- **Slice 2 — visual report:** V2 navigation/layout, KPI cards, genuine static summary charts, compact contract navigation. Honor existing report tests and security guarantees; preserve JSON compatibility.
- **Slice 3 — traceable evidence:** integrate reviewed bounded source reader and Explorer components; show evidence coverage only with a trustworthy required-evidence denominator.
- **Slice 4 — genuine history:** explicitly saved snapshots outside projects, provenance and comparisons before any trend line is shown.

## Non-goals and approval gates

The HTML prototype is **not** the shipped dashboard. It is a design baseline only. No automatic LLM call, full ADP conformance score, fabricated health score, history or source excerpts. Keep implementation PRs small and separately reviewed; integrate [compatibility #4](https://github.com/FrederikLive/ADP-Observatory/pull/4), [reader #6](https://github.com/FrederikLive/ADP-Observatory/pull/6), [Explorer #8](https://github.com/FrederikLive/ADP-Observatory/pull/8) and legacy-reader hardening [#7](https://github.com/FrederikLive/ADP-Observatory/issues/7) under their own security gates. Do not change canonical ADP to satisfy the visualizer.
