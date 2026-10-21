# ADP Integration — Compatibility-First Decision

**Status: design decision, not a normative ADP amendment.** Canonical ADP remains in FrederikLive/ADP. No changes to ADP.md, ADP.min.md, or its schema are authorized or needed at this stage.

## Protected ADP intent
ADP optimizes agent throughput, low token consumption, minimal context retrieval, project understanding, durable history, and reliable cold-session continuity. Observatory exists to help humans visualize that existing information. **The analyzer must adapt to the protocol, never require the protocol to serve the analyzer.**

## Assessment against ADP 2.7.1
ADP already has a reading policy, AGENTS.md router, source-of-truth ownership, continuity records, evidence and verification requirements, optional WORK.json, and external-analysis reconciliation. Its WORK schema already supports IDs, statuses, dependencies, contract paths, and execution metadata. These are sufficient to start Observatory without changing the protocol.

## Decision
1. Do **not** introduce a tenth protocol, mandatory interoperability profile, mandatory evidence registry, event log, relationship manifest, dashboard metadata, or additional routine agent reads.
2. Do **not** change the existing WORK.json schema solely for Observatory.
3. Reuse IDs, links, and evidence references where they already exist. Infer candidate relationships conservatively with provenance; do not promote guesses to authoritative relationships.
4. Implement version-aware, fault-tolerant adapters in Observatory, including unknown-version and incomplete-project modes.
5. Separate parse compatibility, semantic compatibility, and conformance coverage; never turn unsupported or missing data into passing or failing verdicts.
6. Keep optional snapshots in Observatory, not in the scanned project.

## Future ADP change gate
Only revisit ADP when repeated real-project tests show a concrete interoperability gap that also matters for agent efficiency, understanding, or continuity *without Observatory*. A candidate must:
- Identify the existing rule it clarifies and avoid duplication.
- Demonstrate benefit for projects without Observatory.
- Estimate additional agent reading/writing and token overhead.
- Be backward-compatible or include migration and versioning rationale.
- Preserve one canonical owner per concern.
- Pass tests on minimal, mature, old, and incomplete ADP projects.

Prefer no change. If evidence warrants one, propose the smallest clarification to an existing ADP section and follow canonical ADP's versioning and validation procedures. This repository does not grant itself authority to amend ADP.

## Compatibility testing
Use fixtures for no WORK.json, missing continuity, conflicting claims, invalid JSON, broken contract links, older ADP versions, future/unknown versions, and extra project-specific files. Track false positives, unsupported coverage, and partial-scan behavior.

## Feedback loop
Observatory may identify candidate ADP improvements, but observations do not become normative rules automatically. A human-reviewed proposal must show interoperability benefit and acceptable cost to agent operation.
