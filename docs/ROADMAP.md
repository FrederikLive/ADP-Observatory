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
**Status:** Read-only discovery, optional WORK.json v1 parsing, limited protocol-version discovery, contextual conflict reporting, and fixture tests committed; schema 0.3.0 with bounded contract-file checks and scan scope; CI passed in run 39575987006.
- [x] Add a bounded, read-only filesystem discovery prototype.
- [x] Add initial JSON output mode and schema.
- [x] Define and test explicit traversal coverage states and counters for ignored, oversized, symlinked, unreadable, depth-limited, and entry-limited cases (further stress tests remain).
- [x] Parse core and optional ADP WORK.json schema-v1 fields; full cross-document evidence linking remains open.
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
**Status:** Requirements documented in [docs/LAUNCHER.md](LAUNCHER.md); no installer or downloadable release exists yet.
- [ ] Confirm and test actual Windows, macOS, and Linux OS/CPU support.
- [ ] Package tagged executable artifacts and independently verifiable publisher/release metadata.
- [ ] Support install-once execution from any project directory: `adp-observatory .`; keep headless CLI mode for JSON and CI.
- [ ] Provide a trusted per-user installation path without administrator/root privileges; keep binaries outside target repositories.
- [ ] Evaluate an optional one-shot first-run launcher only when executable verification is feasible.
- [ ] Test Unix/Windows paths, Unicode, corrupted or malicious downloads, offline use, platform mismatch, and uninstall.
- [ ] Provide a verified update/version pinning story; document provenance/SBOM and signing.
- [ ] Update ADP's human-only Observatory guide with real, verified install commands only after releases exist.
**Exit:** an end user can install without a compiler and invoke Observatory from a project directory, with verified release integrity and repeatable platform acceptance tests.

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
- [x] Separate format, semantic, and conformance compatibility states for the WORK schema-v1 adapter.
- [x] Preserve independently readable context when WORK.json or a Markdown record is missing, malformed, or conflicting.
- [x] Establish initial fixture matrix for optional work records, historical/newer versions, broken contract-file links, traversal attempts and symlinks; extend adversarial and real-project coverage before M1 closure.
- [x] Confirm basic read-only behavior in unit and CLI integration tests.
- [x] Report source-backed contract links and explicit traversal coverage without inferred conformance; fuller source evidence traceability remains in M2.

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
