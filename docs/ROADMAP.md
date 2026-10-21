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
**Status:** First offline HTML viewer slice verified; full M3 remains open.
- [x] Build a responsive, self-contained offline project overview with contract filtering, source descriptions, compatibility limits and findings.
- [x] Escape project-supplied text, avoid external resources and listening network services, generate files outside target repository.
- [ ] Add complete evidence navigation, optional local snapshots and compatible historical comparisons.
- [ ] Test real-browser accessibility, keyboard behavior and representative project usability.
- [ ] A future live HTTP API, if justified, requires independent Host/Origin/rebinding and filesystem API security review; not required for the offline viewer.
- [x] Show scan scope, completeness, and unknown/not-assessed values in the offline report.
**Exit:** human-facing reports distinguish evidence from claims, history remains local and optional, and accessibility/security acceptance is verified.

## Optional Agent Launch (M3/M4 integration slice)

- [x] Provide an optional skill triggered by "show Observatory", owning
  implementation outside ADP (WORK-AGENT-LAUNCH-001).
- [x] Implement local/existing binary discovery, offline HTML launch, and
  graceful headless fallback; verify Python unit tests and actual compiled
  CLI end-to-end under CI.
- [x] Link from optional non-normative portable/compact ADP guidance and
  generated agent-router template.
- [ ] Support unattended first-time installation only after verified official
  distribution and user/harness authorization are available.
- [ ] Validate browser opening in supported desktop harnesses. Never confuse
  a cloud-browser report with the user's local browser.

## M4 — Cross-platform release and launchers
**Status:** Preview installation passes three-platform CI (39601029440); signed and SHA-pinned candidate assets pass three-platform packaging (39624399120). No GitHub tag rulesets yet and no official release, OS code-signing, or automatic first-run setup.
- [x] Test Windows x64, Linux x64 and macOS runner binaries in CI; other architectures and real-machine install tests remain outstanding.
- [x] Produce experimental workflow artifacts and signed build-provenance attestations.
- [x] Generate version/SHA-checked, provenance-verified, upload-only release candidates in three platforms (run 39624399120).
- [ ] Establish enforceable protected tag/main rulesets and reviewed official GitHub release assets.
- [x] Demonstrate pinned, verified, per-user preview installation and `adp-observatory .` executable behavior on CI runners (run 39601029440); [ ] graduate to an official general-user installation.
- [x] Provide manual preview helpers with per-user installation directories, no privilege escalation or target-repository writes; [ ] evaluate end-user security/installer UX and official packages.
- [ ] Evaluate an optional one-shot first-run launcher only when executable verification is feasible.
- [x] Test Linux/macOS/Windows pinned artifact acquisition, rejection of incorrect commit SHA, and successful installation in CI; [ ] add Unicode paths, malicious/expired artifact simulation, offline operation, update and uninstall acceptance.
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

## Early visualization and packaging checkpoint (development 0.4.0)
- [x] Implement a first **offline static HTML dashboard** with contracts, source references, compatibility, limitations, and local client-side filtering; not yet a full evidence or history dashboard.
- [x] Implement dashboard/report/headless scan CLI modes; a Rust checkout is still required absent an installed binary.
- [x] Test experimental unsigned Windows/macOS/Linux build artifacts and CLI smoke tests (run 39596011855).
- [ ] Independently authenticated release signing/provenance, verified first-run installer, stable package distribution and update/uninstall.
- [ ] Real browser accessibility/usability testing and full historical metrics/snapshots.
- [ ] Calibrate against real projects with missing and incompatible ADP artifacts.

This early M3/M4 work is tracked by verified WORK-DASHBOARD-001 and in-progress WORK-DISTRIBUTION-001. WORK-UI-001 is a superseded umbrella retained as history. M1 remains active; canonical ADP is unchanged.

