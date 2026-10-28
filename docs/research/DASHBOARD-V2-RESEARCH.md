# Dashboard V2 — Research, metrics contract and interactive design proposal

**Status:** DESIGN DIRECTION ACCEPTED by user — 2026-10-28; prototype remains NON-PRODUCTION and fictional  
**Artifact:** `design_examples/observatory-dashboard-v2.html`  
**Source references:** [`docs/METRICS.md`](../METRICS.md), [`docs/work/WORK-EXPLORER-001.md`](../work/WORK-EXPLORER-001.md), [`docs/work/WORK-DASHBOARD-001.md`](../work/WORK-DASHBOARD-001.md), [`docs/SECURITY.md`](../SECURITY.md).  
**Evidence level:** UI concept; synthetic dataset; no actual user-project scan, verification, historical snapshots, independent security assessment, or ADP protocol modification.

## Why redesign

The existing report is honest and lightweight, but its narrow central column is dominated by lengthy contract cards. Important state is hard to compare, `COMPLETE_WITH_EXCLUSIONS` wraps awkwardly, risk prioritization requires manual reading, and the page does not clearly separate declared progress from reference coverage and independently assessed completion. The new proposal offers a professional command-center style while retaining local-first, offline, read-only behavior.

## Information hierarchy

1. **Trust and scope banner:** generated snapshot versus live data; precise scan limitations; no unsupported compliance verdict. A user must see that the report is not real-time.
2. **Four headline metrics:** declared completion percentage with denominator, active contracts, explicit evidence-reference coverage, and recorded blockers. Each number opens its calculation/evidence explanation in production.
3. **Two visual summaries:** delivery completion trend *only if real compatible snapshots exist* and mutually exclusive declared-state distribution.
4. **Workstream decomposition:** stacked counts and declared completion percentage for named workstreams; click-to-filter rather than showing many static repetitions.
5. **Evidence and traceability:** source/reference presence, read limitations and `NOT_ASSESSED` independent verification. Evidence presence is not verification.
6. **Contract Explorer:** search and state filters, selected contract detail panel with source path, source outline, exact line ranges and evidence flags (once supported).
7. **Risks, blockers and next action links:** explicit recorded blockers and missing references only; no invented risk score or causality.

## Metric semantics and proposed implementation IDs

| On-screen measure | Proposed metric | Formula / source | Missing or uncertain input |
|---|---|---|---|
| Declared completion | `delivery.declared_completion` | `count(applicable completed-state records) / count(applicable declared units)`; WORK.json statuses must use a documented versioned mapping | `UNKNOWN` if reliable ledger/status mapping unavailable; `PARTIAL` with denominator and excluded status count if some records unusable |
| Active contracts | `delivery.declared_active` *(proposed)* | Count statuses mapped to in-progress in the same ledger | No invented work when the optional ledger is missing |
| Blocked contracts | `delivery.declared_blocked` *(proposed)* | Count explicitly recorded blocked states; do not infer blocker from an unresolved source reference alone | Label as recorded; separate structurally broken references |
| Evidence reference coverage | `evidence.required_coverage` | `applicable required references present / applicable required references`; requires a reliable inventory of *required* evidence, not just available links | `NOT_ASSESSED` until required evidence rules exist; no universal 0% |
| Source readability | `source.readable_coverage` *(proposed)* | `readable supported sources / applicable linked sources`, with skipped/unsafe/oversized counts | `PARTIAL` for bounded truncation; show path classification |
| Independently verified completion | `delivery.verified_completion` | Count of units actually meeting independently assessed verification rules / applicable units | `NOT_ASSESSED` until such rules exist; never derived from recorded `verified` status |
| Historical declared completion | `history.declared_completion` *(proposed)* | Comparable, explicitly saved, timestamped, versioned local Observatory snapshots; same denominator policy | No trend graph if no comparable prior snapshots; label baseline only |
| Workstream progress | `delivery.workstream_completion` *(proposed)* | Apply `delivery.declared_completion` independently per declared workstream | Preserve `No workstream` / unknown rather than hiding it |
| Unresolved dependencies | `delivery.unresolved_dependencies` | Declared links that target unknown/unresolved contracts; exact relationship source | Do not treat every dependency as blocking |

**The 61% on the mock is NOT measured project completion.** Synthetic records have 33 complete of 54 total, so `100×33/54 = 61.1%` → displayed **61%**. In the same fictional sample, the 54 states partition as `33 complete + 12 active + 5 blocked + 4 planned`. Workstreams partition as Platform 14 (9+4+1+0), Agents 14 (8+3+1+2), Trust 11 (6+2+2+1), UX 8 (6+1+0+1), Operations 7 (4+2+1+0), summing to the same 54. The synthetic reference measure is 29 linked out of 40 required, `72.5%` → **73%**. These values must *not* ship as defaults or be inferred from Trumpdup's real project screenshot.

The viewer screenshot that motivated this proposal shows a WORK-index format state of `INVALID` and an ADP version of `2.7.1`, with 54 contracts. For that report, a real progress percentage must **not** appear as verified or confidently known until the input can be interpreted safely. The mock intentionally depicts a *hypothetically* structurally valid dataset, separate from the screenshot.

## Data and chart design

- **Donut:** four mutually exclusive *declared* contract states; no inference of verification.
- **Trend chart:** declared-completion percentage vs evidence-reference coverage only if saved snapshots use comparable source coverage, schemas, metric version and denominators. Label illustrative points in the prototype; hide or show “No history recorded” in production without prior snapshots.
- **Workstream bars:** four-state counts; never confuse share-of-stream with share-of-total; percentage label means declared complete within that workstream.
- **Evidence module:** counts and denominators, broken/missing/skipped source states, independent-verification state. The report is a discovery tool, not a conformance certificate.
- **No generic score:** avoid a single “Project health 92%” rating or confidence guess. Where a health summary is requested, offer a transparent multidimensional scorecard instead.

## Privacy, architecture and UI acceptance

The standalone prototype uses only inline HTML/CSS/JavaScript, native SVG for charts, and no CDN, fetch, telemetry, cookies, local storage or third-party chart library. User data never appears: all values, contract titles, examples and trend points are fabricated and explicitly labeled. The production implementation must use the existing bounded, capability-relative reader (once security review is complete), HTML escaping and CSP, and a strict separation between inert source text and UI script. Importantly, **do not reuse** this prototype's permissive mock embedding as permission to expose private source passages in real reports. No scripts or tests should be run against the analyzed project just to populate metrics.

The prototype includes usable anchor navigation, chart-range toggle, workstream click-to-filter, contract search, state filter, contract-details drawer, methodology modal, Escape-to-close and mobile layout. A future production build should additionally provide full focus trapping in modal/drawer, screen-reader announcements for chart data, robust no-JS fallback, keyboard/a11y automated testing and real long-name/large-ledger performance runs.

## Suggested ADP-Observatory delivery order

1. **Integrate known scanner work safely:** review PR #4, PR #6 and PR #8 in dependency order; keep main protected; resolve legacy-reader boundary issue #7. The design mock itself should **not** be coupled to those PRs.
2. **M2 metric computation** (small PRs): add `delivery.declared_completion` with explicit statuses/denominator/exclusions and fixtures for malformed/unknown/partial ledgers; then workstream partition and blockers. Verify semantics against `docs/METRICS.md` before showing any metric as known.
3. **M3 new visual shell:** replace the narrow report layout with responsive summary cards, compact contract navigation and trustworthy static distribution/workstream charts. Reuse current report schema only where supported; feature gate metrics until available.
4. **Evidence and Explorer:** build on the bounded reader and source-provenance spec in `WORK-EXPLORER-001.md`; honor the unmerged PR #8 native disclosure implementation. Add bounded snippets/lines only through separately reviewed opt-in work.
5. **History snapshots:** explicit user-local storage outside the target repository, retention/deletion/export controls, schema/version comparison, no background polling. Ship trends **only then**.
6. **Real local acceptance:** check 30- and 300-contract projects, old/partial WORK.json, missing evidence, screen readers, offline behavior, immutability, Git-status before/after and Windows desktop preview. Avoid changes to the scanned repository.

## Out of scope for this design study

Actual verification execution; full ADP conformance evaluation; score based on subjective heuristics; automatic file reconciliation or Git writes; cloud service; remote APIs; LLM summarization; durable historical snapshots; new mandatory ADP records; release approval; or changing the current production dashboard.

**Decision recorded 2026-10-28:** the user accepted Dashboard V2 as the design direction and requested integration into the repository. See [`DESIGN.md`](../../DESIGN.md). Implementation remains separate from this design-only PR; no production, security, or release verification is implied.