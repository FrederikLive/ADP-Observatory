# ADP Integration Proposal

**Status: proposal only.** This document does not change the normative ADP specification. Any amendment must be reviewed and merged separately in FrederikLive/ADP under its contribution and versioning rules.

## Principle
ADP defines project behavior and evidence semantics. Observatory reads and analyzes those records. It must not become a required ADP dependency or create a competing source of truth.

## Candidate minimal additions
Prefer a short optional subsection in an existing ADP section rather than a new core protocol:
1. **Optional analysis tooling:** projects MAY use local or CI analyzers; no specific product or dashboard is required.
2. **Evidence traceability:** where machine-readable records already exist, tools SHOULD preserve stable identifiers and source references. Do not mandate a new manifest solely for visualization.
3. **Metric transparency:** metrics SHOULD state definition, applicability, evidence sources, calculation time, completeness, and limitations.
4. **Unknown semantics:** missing or unsupported data MUST NOT be silently treated as zero, complete, or passing.
5. **Source-of-truth protection:** analysis output is derived evidence and MUST NOT supersede canonical project records.
6. **Authorization boundary:** analysis tooling SHOULD be read-only by default; modifying status, reconciling findings, or writing canonical records requires explicit authorization.

## Change threshold
Add normative requirements only if they improve interoperable ADP behavior for projects that do not use Observatory too. Keep metric catalogues, schemas, UI, launchers, storage, and implementation-specific checks here. Before proposing an amendment, map candidate rules to existing ADP sections, check overlap/conflicts, explain compatibility/security/autonomy/continuity/burden, update ADP.md and ADP.min.md plus metadata/validator as required, and test against a project using an analyzer and one without one. Prefer PATCH for clarification/correction and MINOR for backward-compatible normative additions. Choose version only after reviewing the actual diff.

## Feedback loop
Observatory findings may reveal repeated friction or ambiguous conventions. They are research candidates, not automatic protocol changes. Require multiple examples, interoperability benefit, acceptable burden, and human review.
