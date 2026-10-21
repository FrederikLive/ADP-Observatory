# ADP Integration — Compatibility-First Decision

**Status: optional user-invoked integration implemented; not a normative ADP amendment.** Canonical ADP remains in FrederikLive/ADP. At the user's explicit request, non-normative on-demand skill references were added to its portable/compact guidance and AGENTS template. No ADP schema, required project records, or agent workflow obligations have changed.

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

## Optional Agent Launch implementation (2026-10-21)

A user's request such as "show Observatory" conditionally leads an ADP-aware
agent to the official `skills/adp-observatory/SKILL.md`. All executable
discovery, desktop-versus-headless behavior and approved-installation guidance
are owned here, not in ADP. The associated standard-library Python helper
`skills/adp-observatory/scripts/launch.py` does not download software, runs
only an existing explicitly trusted executable outside the scanned project,
returns structured success/limitation states and produces an HTML report via
the existing offline dashboard.

**Trust boundary:** an ADP hint is *discovery*, not authorization to execute
remote code. A verified public release is still pending owner-controlled
release governance; first-use installation must follow the separately reviewed
provenance/approval process. A remote-only agent cannot promise to open the
user's local browser.

Future integrations should use the same discover-on-demand pattern rather
than adding another ADP protocol or mandatory event log. For older projects,
adopt the optional hint manually or install the skill at the harness/user level.
