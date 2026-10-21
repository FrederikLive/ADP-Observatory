# Architecture

**Status:** Proposed baseline. Validate through a vertical slice before treating decisions as irreversible.

## Principles
Local-first; project repository remains source of truth; read-only scanning; deterministic and explainable analysis; separate parsing, rules, metrics, persistence, API, and UI; stable machine contracts independent of presentation.

## Proposed components
1. CLI/launcher: arguments, target path, config, version, scan, export, optional dashboard.
2. Discovery: safely identify supported layouts, canonical files, recorded protocol version, exclusions, and scan boundaries.
3. Format adapters: parse supported Markdown, JSON, YAML, and other formats only when semantics and errors are documented; preserve source path and line/pointer.
4. Normalized evidence model: typed, versioned project items and relationships with provenance.
5. Rule engine: explicit invariants and applicability; each rule has stable ID/version, predicate, severity, rationale, and fixtures.
6. Metric engine: calculate only from known inputs; represent unknown and unavailable states distinctly.
7. Snapshot store: optional local SQLite history; derived and disposable, never authoritative.
8. API/dashboard: loopback-only API; CLI JSON works without the dashboard.
9. Packaging/update: platform artifacts, verifiable release metadata, launchers, and documented update/uninstall.

## Language and UI direction
Investigate Rust for a portable core and a lightweight web UI (TypeScript/React or a smaller alternative if it reduces maintenance). Do not lock in a framework before the first vertical slice validates packaging, local API security, and actual UI needs.

## Data flow
Target project → bounded discovery → safe parsing → normalized evidence with provenance → applicable rules → metrics/findings → optional local snapshot → JSON/report/dashboard.

Never execute arbitrary target-project content. Treat text as data. The scanner must not run project commands.

## Local server security
- Bind only to 127.0.0.1 and/or ::1, never all interfaces.
- Reject unexpected Host and Origin headers; use an unpredictable per-process token for state-changing operations.
- Use restrictive CORS, content security policy, and safe output encoding.
- Avoid arbitrary path read APIs; resolve source references within the selected root and reject traversal.
- Avoid shell interpolation; invoke no shell for normal scans.
- Make server lifecycle visible and provide a clear stop/exit path.

## Storage
SQLite stores derived snapshots only. Record schema/analyzer/protocol versions, timestamp, scan scope, completeness, metrics, and finding fingerprints. Avoid copying whole repository contents. Provide local data location and delete/export controls.

## Machine-readable contract
Versioned JSON should include schema/analyzer version, detected ADP version/state, privacy-conscious target identity, timestamp, completeness, scanned/skipped path summary, rule results, findings, metrics, errors, and limitations. Stable IDs; consumers must not parse human prose to determine state.

## Extensions
Start with built-in rules/adapters. Add third-party plugins only when a real use case establishes a security and compatibility model. Do not dynamically execute downloaded plugins.

## Validation
The first vertical slice scans a tiny fixture, emits deterministic JSON, displays a local result, and demonstrates release integrity verification. Revisit stack decisions if costs are disproportionate.
