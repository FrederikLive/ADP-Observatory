# Metric Catalogue and Semantics

Metrics summarize evidence; they do not replace judgment. Every implemented metric needs a stable ID, version, formula, unit, applicability, evidence requirements, limitations, and tests.

## Required states
- **KNOWN:** all required inputs are available and calculation is valid.
- **PARTIAL:** some inputs are available; exclusions are explicit.
- **UNKNOWN:** required data is missing, ambiguous, contradictory, or unsupported.
- **NOT_APPLICABLE:** applicability predicate is false, with a reason.
- **NOT_ASSESSED:** analyzer cannot evaluate the metric or rule.

Never convert these states to zero or pass. Show denominators and coverage beside percentages. Avoid an unqualified overall project-health score.

## Delivery
- delivery.declared_completion: declared completed units / applicable declared units, only with a reliable set and statuses.
- delivery.verified_completion: units meeting required verification / applicable units; distinguish declared from verified completion.
- delivery.completed_unverified: units declared complete but missing required verification.
- delivery.unresolved_dependencies: explicitly recorded unresolved dependencies; do not infer blocker-hours.

### Implemented dashboard metric: `delivery.declared_completion` (view rule v1)

- **Input:** optional structurally valid schema-v1 `.project/WORK.json`, with a declared count matching the accepted contract records and every status recognized by the scanner. This is *recorded* delivery, not independent verification or engineering-hour weighting.
- **Formula:** `100 × count(status == verified) / count(status != cancelled)`, rounded to the nearest integer. Cancelled items are excluded; deferred, candidate, ready, blocked, in_progress, implemented and verifying remain in the denominator. All nine statuses are mutually exclusive.
- **States:** `KNOWN` only for a valid ledger with nonzero applicable records; `UNKNOWN` for invalid/missing/mismatched/unsupported ledgers or zero denominator. No total is inferred when inputs are missing. Snapshot trend remains `NOT_ASSESSED`.
- **Display:** show numerator/denominator and excluded-cancelled count on the card; workstream charts use the same semantics per workstream. Group recorded `verified`, underway (`in_progress`/`implemented`/`verifying`), blocked, planned/deferred (`ready`/`candidate`/`deferred`), cancelled.
- **Coverage caveat:** `Reference presence` is the share of declared contract-file references whose metadata state is `PRESENT`, and is shown only when all such references are inspected; it **does not** mean required evidence was provided or accepted. `evidence.required_coverage` remains `NOT_ASSESSED` until an independent required-evidence inventory/rule exists.
- **Limits:** version-independent heuristic of *declared record status*, not full ADP semantics; no new scan actions, files, source excerpts or external access. Test cases cover missing/invalid ledger, unmatched count, unsupported status and cancelled-only denominator.

## Evidence and traceability
- evidence.required_coverage: applicable required evidence present / applicable required evidence.
- evidence.missing_required: count of missing required evidence references.
- traceability.requirement_chain_coverage: requirements with linked implementation and verification evidence / requirements requiring such a chain.
- evidence.stale: only where policy defines freshness and trustworthy timestamps exist.
- evidence.contradictions: candidate conflicting claims referencing both sources; heuristics remain candidates until reviewed.

## Conformance
- conformance.checks_passed, conformance.checks_failed, conformance.checks_not_assessed: applicable checks grouped by state.
- conformance.applicability_coverage: applicable checks assessed / applicable checks identified.
- conformance.protocol_compatibility: supported/unsupported/unknown with detected protocol version and analyzer support range.

## Continuity and delegation
- continuity.checkpoint_completeness: required checkpoint fields present / fields required by applicable template.
- continuity.handoff_completeness: required handoff context present / required context.
- delegation.unreconciled_results: recorded delegated results awaiting reconciliation.
- Retry counts and blocked durations require explicit trustworthy event/timestamp records; never infer from prose.

## External analysis reconciliation
- analysis.findings_by_disposition: counts by accepted, rejected, deferred, duplicate, unresolved, or project-defined disposition where supported.
- analysis.accepted_verified: accepted findings with implementation and required verification / accepted findings requiring implementation.
- analysis.unresolved: findings without final disposition.
- analysis.archive_traceability: intakes with resolvable original source and reconciliation record / applicable intakes.

## Change and maintainability
- change.regressions: newly failing findings compared with a compatible earlier snapshot.
- change.resolved_findings: previously present findings no longer present; resolution is provisional until validated.
- change.scope_delta: explicitly recorded scope changes; do not infer intent from line counts.
- File churn, hotspots, test coverage, CI reliability, and dependency risk are optional extensions requiring reliable source data and defined scope.

## Documentation
- docs.broken_references: internal references that fail within declared scan scope.
- docs.orphaned_artifacts: artifacts with no detected inbound links; candidates only because standalone files may be intentional.
- Freshness requires explicit policy; modification time alone does not prove factual correctness.

## Snapshot comparisons
Compare only compatible schema, analyzer/rule versions, scope, and applicable inputs, or annotate incompatibility. First scan is a baseline, not a trend. Missing prior data means no baseline, not zero. A finding disappearing may reflect renaming, scope changes, parser changes, or unsupported input; explain uncertain matching. Preserve completeness and errors.

## Evidence and confidence
Metric detail links to inputs and shows calculation time, rule version, denominators, exclusions, and limitations. Without reproducible evidence, do not present a metric as precise fact.

## Machine-readable V2 metrics: `metrics` (0.4.1 report)
The Rust analyzer computes `metrics` deterministically from the already-parsed optional WORK index, using rule `declared-metrics-v1`. It does **not** open additional files or run the analyzed project. The headless JSON report and offline HTML use these same numbers; the implementation version and evidence path accompany each metric.

- `delivery`: state, completed, applicable denominator, excluded cancelled count, percentage, five mutually exclusive recorded-state groups, and per-workstream totals. Display percentages only when inputs support them. The unit is *contract count*, not time, cost, effort or independently assessed progress.
- `references`: counts of explicit `contract_file` paths observed by the existing metadata scanner: expected, inspected, present, missing and other unsafe/unreadable/unsupported. A percentage is emitted only when a nonzero complete set was inspected; partial observations emit `PARTIAL`, not a deceptive fraction.
- `dependencies`: number of declared `depends_on` links and links whose target contract is *not recorded* `verified`, with at most 200 explicit relationship examples. An unresolved/unknown dependency produces `PARTIAL`. “Pending” is a status comparison, **not** an independently proven blocker.
- `independently_verified`, `historical_trend`: always `NOT_ASSESSED` until separate tested analysis and durable user-controlled history exist.

When optional WORK.json is invalid, missing or unsupported, the delivery state is `UNKNOWN`; numeric zeroes in the serialized structure are placeholders for unavailable counts and **must never be presented** as known observations. `NOT_APPLICABLE` is used when a valid ledger has only cancelled work or no applicable references.

Threat-model caveat: `PRESENT` describes prior path metadata observation, not safely opened source bytes. The separate bounded reader and legacy-reader hardening issues remain independent security work.
