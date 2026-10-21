# Roadmap

Milestones are proposed work packages, not claims of completion.

## M0 — Repository foundation
**Status:** ADP control plane established; baseline CI passed on bootstrap commit.
- [x] Define initial scope, threat model, metric semantics, architecture, and roadmap.
- [x] Add operating instructions, license, changelog, and contributor guidance.
- [x] Add Rust package scaffold and formatting/lint/test/build workflow.
- [x] Review bootstrap CI result and establish ADP control plane.
**Exit:** fresh contributor can understand scope, run validation, and identify next task.

## M1 — Discovery and deterministic scanner
**Status:** Read-only discovery prototype and initial WORK.json structural parser committed; parser CI pending.
- [x] Add a bounded, read-only filesystem discovery prototype.
- [x] Add initial JSON output mode and schema.
- [ ] Define and test precise scan-completeness semantics for ignored, oversized, unreadable, and symlinked paths.
- [x] Begin parsing canonical ADP WORK.json records; deeper cross-document linking remains open.
- [ ] Add valid, incomplete, contradictory, malformed, and hostile-input fixtures.
**Exit:** deterministic results; no target code execution or target-file modification; tests cover scope and failure behavior.

## M2 — Rules and metrics
- [ ] Implement a small high-value rule set.
- [ ] Implement metric states/formulas in docs/METRICS.md.
- [ ] Add applicability, rule versions, source traceability, and false-positive fixtures.
- [ ] Compare results against independently reviewed ground truth.
**Exit:** every shipped rule has definition, tests, limitations, and source references.

## M3 — Local dashboard and snapshots
- [ ] Build overview, finding detail, evidence navigation, and filters.
- [ ] Harden loopback API against hostile origins, DNS rebinding, traversal, and XSS.
- [ ] Add opt-in local snapshots and compatible comparisons.
- [ ] Show scan scope, completeness, and unknown/unassessed values.
**Exit:** partial scan cannot appear fully healthy; history is local, optional, transparent.

## M4 — Cross-platform release and launchers
- [ ] Confirm OS/architecture support.
- [ ] Package versioned artifacts and publish integrity metadata.
- [ ] Implement launchers that download official artifacts and verify before execution.
- [ ] Test Unicode paths, spaces, offline use, corrupt downloads, interrupted updates, uninstall.
- [ ] Publish provenance/SBOM and signing where feasible.
**Exit:** supported platforms have repeatable release tests and safe update/uninstall.

## M5 — Pilot and calibration
- [ ] Run against ADP and representative permissioned projects.
- [ ] Record false positives/negatives, unsupported layouts, scan cost, and data gaps.
- [ ] Refine applicability and documentation.
- [ ] Publish limitations and stable JSON schema.
**Exit:** findings are explainable and useful; release scope is explicit.

## M6 — Evidence-driven ADP improvement
- [ ] Write a proposal based on recurring evidence; no telemetry default.
- [ ] Check overlap, normative impact, compatibility, and implementation burden.
- [ ] Submit protocol changes separately to the ADP repository.
**Exit:** no protocol change solely to accommodate Observatory or because a metric is easy to collect.

## Compatibility-first implementation constraint

All milestones must preserve ADP's low-token, fast-agent, continuity-first design. Observatory adapts to existing records; no extra ADP files or agent bookkeeping may be required. See docs/ADP-INTEGRATION.md and docs/ARCHITECTURE.md.

### M1 additional acceptance work
- [ ] Separate format, semantic, and conformance compatibility states.
- [ ] Preserve partial results when individual records are absent, malformed, or conflicting.
- [ ] Add fixture matrix for optional WORK.json, old/current/newer/unknown versions, broken links, and hostile input.
- [ ] Demonstrate scanning never changes the analyzed repository.
- [ ] Show source provenance and scan completeness; no inferred passing verdicts.

### M2 additional acceptance work
- [ ] Apply rules only where ADP version and evidence make applicability defensible.
- [ ] Keep inferred relationships labeled as candidates, not authoritative facts.
- [ ] Report unsupported coverage and denominators for every relevant metric.

### M5 additional acceptance work
- [ ] Validate no extra agent token or documentation burden is imposed on scanned projects.
- [ ] Test compatibility with small, mature, incomplete, and older-version projects.
- [ ] Quantify false positives, scan failure isolation, and coverage limitations.

### M6 gate
- [ ] Do not amend ADP unless repeated real-project evidence shows an agent-beneficial interoperability gap independent of Observatory.
