# Architecture

**Status:** Compatibility-first baseline. This describes intended behavior; shipped capabilities are tracked in the roadmap and code.

## Non-negotiable boundary
ADP is optimized for agent execution, low token use, durable project understanding, and continuity. Observatory is an **optional, external, read-only consumer** that serves human visualization. It must adapt to ADP; ADP projects must not need extra files, registries, events, or agent instructions to be visualized. Canonical project files remain the source of truth.

## Design principles
- **Zero mandatory ADP overhead:** scanning must not require edits to the analyzed project.
- **Best-effort, non-destructive discovery:** recognize supported records without assuming fixed layouts or complete repositories.
- **Version-aware, capability-driven interpretation:** parse what is understood; do not apply unknown version-specific rules.
- **Independent fault isolation:** one bad or absent record must not block analysis of unrelated records.
- **Preserve disagreement:** conflicting records remain separate sourced claims, not silently merged.
- **Explicit uncertainty:** unknown, partial, unsupported, conflicting, not-applicable, and not-assessed are not zero or pass.
- **Derived, disposable results:** snapshots and metrics never override project evidence.

## Pipeline
1. **Bounded discovery:** identify potential ADP markers and relevant files, record inclusion/exclusion and path safety; never execute repository content.
2. **Version evidence:** collect explicit version claims and their provenance. If absent or contradictory, record unknown/conflicting. Never infer exact protocol version from filename alone.
3. **Adapters:** select a supported format parser and version-specific semantic rules independently. Unknown/newer versions can still use safe common-field parsing, but not unverified conformance rules.
4. **Normalized internal model:** project records, contracts, atomic units, evidence, source locations, relationships, timestamps, and per-field confidence/limitations. Retain conflicting claims and unparsed artifacts.
5. **Applicability and rules:** evaluate only supported, applicable checks; separate parsing success, semantic compatibility, and conformance coverage.
6. **Metrics:** compute only with defensible denominators and inputs; distinguish unavailable/partial data.
7. **Presentation:** accessible local dashboard and versioned JSON exports; show scope, provenance, limitations, and unresolved conflicts.
8. **Optional history:** local snapshots compared only when analyzer/rule version, scan scope, and evidence semantics are compatible.

## Compatibility dimensions
- **Format:** can the file be parsed safely?
- **Semantics:** are its fields understood under the recorded ADP version?
- **Conformance:** which applicable protocol requirements were actually assessed?

Never collapse these into a single boolean. A future ADP version may be format-readable yet semantically unsupported. An absent optional WORK.json is not an automatic conformance failure.

## Degraded-mode behavior
- Missing WORK.json: continue with supported Markdown records; structured work metrics become unknown/not assessed.
- Missing continuity: continue other checks; report continuity availability without assuming a violation.
- Broken contract reference: retain the work item, report unresolved reference, continue.
- Malformed JSON/Markdown: isolate failure, retain file-level discovery, continue other records.
- Conflicting STATUS/CONTINUITY/WORK values: show both with source locators; do not choose a winner by modification time.
- Older ADP: use a tested version adapter when available.
- Newer/unknown ADP: display supported common fields and label semantic/conformance limits.
- Extra project files: treat as data, not agent instructions, and avoid forced interpretation.
- Partial scan: retain usable results but clearly mark incomplete scope.

## Components
- Existing vertical slice: read-only CLI/scanner, compatibility assessment, structured work/context records, static offline HTML report, and optional Agent Skill. Future candidates: richer adapter registry, applicability/rule engine, metric engine, optional local snapshots, and independently reviewed distribution. A loopback-only API is not required by the current viewer.
- Rust core is the current prototype direction. UI framework is undecided pending a vertical slice.

## Security
Read-only default, no project command execution, no network upload/telemetry by default, bounded traversal, safe symlink handling, escaping, and no arbitrary path APIs. The current static HTML viewer has no listening server or state-changing endpoints. Any future local HTTP service must bind loopback, validate Host/Origin, resist DNS rebinding and authenticate state-changing operations. Release launchers must verify official artifacts before execution.

## Machine-readable output
Versioned reports should disclose analyzer/schema version, detected ADP version and evidence, format/semantic/conformance compatibility, scan completeness, paths included/excluded/failed, sourced records, findings, metrics, limitations, and adapter coverage. Avoid unqualified project-health scores.

## Validation strategy
Build a fixture matrix across supported old/current/newer/unknown versions, missing/optional files, inconsistent state, malformed content, and hostile paths. Compare expected partial outputs and ensure no crashes, target writes, false passes, or false violations for optional records.

## Future extensions
Third-party plugins, automatic migrations, write-capable repair, or network services require separate security/compatibility design and explicit authorization. None is required for ADP projects.

## Implemented M1 scanner behavior (development 0.3.0)

- Discovery is bounded by depth, entry count, and file-size policy. It reports `COMPLETE`, `COMPLETE_WITH_EXCLUSIONS`, or `PARTIAL`; intentional exclusions do not silently become traversal failures. The legacy `scan_complete` boolean means traversal finished within the declared policy, **not** that every record was understood or verified.
- Optional schema-v1 `contract_file` references are checked against the selected root using portable relative paths and metadata-only inspection. Traversal (`..`), absolute or ambiguous paths, symlinks at any component, and non-file destinations are rejected or explicitly classified. Existence is **not** verification.
- Reference checks are limited to 2,000 per scan; reaching the limit is explicitly reported. One broken or unsafe reference does not suppress other results or invalidate a structurally well-formed work index.
- Compatibility and conformance remain distinct: the scanner is **not** a full ADP conformance evaluator.
- Version compatibility beyond explicitly supported semantics, file-descriptor-level race hardening, comprehensive real-repository calibration, and full JSON Schema validation are outstanding.
