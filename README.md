# ADP-Observatory

**A local-first analysis and visualization companion for Agent Development Protocol (ADP) projects.**

ADP-Observatory helps people understand the evidence already present in an ADP-enabled repository: project progress, work contracts, verification, risks, continuity, and what can or cannot be assessed. It provides a **read-only view of the project**, not a second source of truth or an alternative development workflow.

**Your project stays untouched.** During a normal scan or dashboard launch, Observatory reads supported project files but does not modify source code, `ADP.md`, `AGENTS.md`, work records, Git history, `.gitignore`, or any other file within the analyzed repository. It does not execute project scripts or automatically install dependencies. Generated reports and application files are kept **outside** the repository, so you do not need to add Observatory entries to your project's `.gitignore`.

**The agent must respect the same boundary.** Asking a coding agent to "set up Observatory" is not permission to create `.project/WORK.json`, run the application's tests, or commit/push changes to make the dashboard look more complete. Missing optional records stay missing; the dashboard explains what it could not assess.

> **Project status:** Early development (M1 scanner with a working offline HTML viewer). Historical metrics, snapshots, and broader analysis remain planned. Missing or unsupported evidence is reported honestly; an Observatory report is not a full ADP conformance assessment.

## Compatibility with current ADP Actions (21 October 2026)

The current [ADP main revision](https://github.com/FrederikLive/ADP/commit/9b5ab4694321735e83b93962e1b9975cced130bf) still declares **protocol 2.7.1**. Its optional [ADP Actions](https://github.com/FrederikLive/ADP/blob/main/docs/COMMANDS.md), [project-update guide](https://github.com/FrederikLive/ADP/blob/main/docs/UPDATING_ADP.md), and agent-router hints introduce **no required scanner inputs**. Observatory works with the same optional `WORK.json`, protocol files and status/continuity records; it never requires projects to copy the Actions catalogue.

**Verified integration:** [CI 39659907217](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39659907217) checked out the exact upstream ADP revision and ran both JSON scanning and HTML reporting, correctly recognizing version 2.7.1 while leaving missing optional WORK records unassessed. The report now offers static examples of ADP Actions as prompts for the **coding agent**, not dashboard-executed commands. The separate [Agent Skill](skills/adp-observatory/SKILL.md) handles an explicit **"Show Observatory"** request; **"Show ADP Actions"** or **"Update ADP"** do not trigger an Observatory launch.

With a previously approved installed binary, a local agent can invoke the skill's [offline launch helper](skills/adp-observatory/scripts/launch.py). Browser opening depends on real local desktop/process access; hosted agents cannot guarantee a window on the user's desktop. First-time automatic installation remains blocked on verified official release governance; no download is executed merely because the project uses ADP.

## Core objectives

1. **Observe, never interfere.** Analyze the existing ADP project **read-only by default**. Do not rewrite project files, change task statuses, update Git, edit `.gitignore`, execute project scripts, or require changes to the project's structure.
2. **Protect ADP's original purpose.** ADP exists to help coding agents work efficiently, conserve tokens, preserve dependable project knowledge, and resume work accurately. Observatory must adapt to the information ADP already provides—**not** create new mandatory records, agent instructions, or routine bookkeeping just to power a dashboard.
3. **Make project state understandable.** Show the available work contracts, progress, evidence, risks, conflicts and continuity in a clear human-facing view. Link findings to real sources where supported, and distinguish reported facts from interpretations.
4. **Be transparent about uncertainty.** A missing optional file is not automatically a defect; a discovered file is not proof that work was verified. Clearly label unknown, incomplete, incompatible and unassessed information. Never invent historical trends or an ADP conformance verdict.
5. **Keep data local and private.** Generate reports locally without a required cloud account, telemetry, or automatic project-data uploads. Reports may reveal internal file paths and project details, so users control their storage and sharing.
6. **Be easy to use without changing the project.** Aim for an install-once application that a user or authorized coding agent can launch from any ADP project. Keep the executable, generated reports, caches and future optional snapshots in user-controlled locations **outside the analyzed repository**.
7. **Remain compatible and independent.** Tolerate older, newer, partial and inconsistent ADP projects where safe; remain an optional companion rather than another ADP protocol or a prerequisite for agent development.

### What Observatory changes — and what it does not

| Area | Behavior during normal scanning and dashboard use |
| --- | --- |
| **Project source code, documentation and ADP records** | Read for analysis; **not edited, replaced, generated or deleted** |
| **`.gitignore`, Git commits and branches** | **Not modified**; no Git commit, checkout, reset or push is performed |
| **Project dependencies, scripts, tests and hooks** | **Not installed or executed** by the analyzer |
| **Observatory executable and application cache** | Stored separately from the analyzed project, in approved user-level locations |
| **HTML dashboards and derived results** | Created **outside** the analyzed repository; default HTML output uses a local application cache |
| **Network access and project data** | Local analysis works offline; **no automatic upload or telemetry** |
| **Optional future snapshots** | Planned as separate, user-controlled Observatory data—not canonical ADP project records |

**No `.gitignore` changes are necessary.** The current report writer rejects output paths inside the analyzed project. Installing or updating Observatory is a separate, authorized operation: the app can create its **own** files outside the project, but that is not permission to modify the project itself. If a user explicitly wants an in-project export or integration in the future, it must be separately designed and authorized; it is not the default behavior.

See [Security and Privacy](docs/SECURITY.md) and [ADP Integration](docs/ADP-INTEGRATION.md) for the technical boundaries.

## How it works

1. **Choose a project.** Open a terminal in an ADP project or ask a local coding agent to **"Show Observatory."** Observatory receives the selected project directory; it does not need to be copied into that project.
2. **Scan without changing it.** The analyzer reads supported records, safely handles absent or inconsistent data, and reports findings and limitations without running the project's code or rewriting its files.
3. **Generate the dashboard elsewhere.** The current development executable creates a self-contained offline HTML report in a user-level application cache **outside the project**. No HTTP server or external page resources are required.
4. **View the result.** On a supported local desktop, the executable attempts to open that report in the default browser. In CI or a remote/headless environment, it returns the report path instead of claiming to open the user's browser.
5. **Close or remove reports independently.** Generated reports belong to Observatory's local output location, not to the project's source of truth or Git history.

**Installation status:** The working development CLI and verified experimental platform packages are available, but a protected official release and no-prerequisite automatic first-time installer have not yet shipped. Approved first-time installation is handled separately and must verify executable provenance. The long-term goal is an install-once experience without requiring a compiler or additional development toolchain.

## Product boundaries

### ADP remains responsible for

- Normative project/agent behavior and conformance expectations.
- Work decomposition and project records.
- Verification, evidence, continuity, escalation, and external-analysis reconciliation rules.
- The source of truth for project state.

### ADP-Observatory is responsible for

- Repository discovery and format adapters.
- Version-aware analysis and conditional applicability.
- Metric definitions and calculation.
- Findings, confidence/limitations, traceability, local history, and visualizations.
- Release packaging, launchers, and optional integrations.

ADP-Observatory must not silently write project records, accept/reject external findings, change task status, commit changes, or push to remotes.

## Initial metric families

- **Delivery:** declared versus verified completion, milestone/workstream status, overdue or blocked work where dates and states are recorded.
- **Evidence:** required evidence coverage, missing/stale/contradictory evidence, requirements-to-implementation-to-test traceability.
- **Conformance:** applicable checks passed/failed/not assessed, severity, protocol version compatibility, missing required structures.
- **Continuity and agent operations:** checkpoint/handoff completeness, delegated work reconciliation, retry/blocking data only when explicitly recorded.
- **External analysis:** intake traceability, dispositions, rationale, accepted findings linked to implementation and verification, unresolved/deferred findings.
- **Change and maintainability:** scope change, rework, regression, file churn, hotspots, test/CI results when trustworthy source data exists.
- **Documentation:** inventory, cross-document contradictions, freshness based on configured policy, orphaned references.

Each metric must document its formula, unit, applicability, source evidence, time window, confidence, and limitations. Unknown is not zero. A repository scan alone cannot establish historical trends.

## Architecture direction

- **Analysis engine:** Rust, for a portable, single-binary core and deterministic repository analysis.
- **Dashboard:** initial responsive, self-contained, offline HTML overview is implemented in Rust; richer UI and history remain planned.
- **Local persistence:** SQLite for scan snapshots and metric history; source project files remain untouched.
- **Interface:** the current offline HTML report has no HTTP server; a future live dashboard API would need loopback binding and origin/host protection.
- **Release delivery:** signed or otherwise verifiable release metadata, checksums, platform-specific artifacts, and minimal launchers.
- **Validation:** unit/integration fixtures, golden test repositories, security checks, and CI.

This is the target architecture, not a claim that all components are implemented yet. Detailed decisions and alternatives are tracked in `docs/ARCHITECTURE.md`.

## Repository map

- `AGENTS.md` — operational instructions for AI coding agents.
- `docs/PRODUCT.md` — product requirements, user journeys, and scope.
- `docs/ARCHITECTURE.md` — architecture and technical decisions.
- `docs/METRICS.md` — metric catalogue and definitions.
- `docs/ADP-INTEGRATION.md` — minimal proposed relationship with ADP.
- `docs/SECURITY.md` — threat model and release/launcher requirements.
- `docs/ROADMAP.md` — milestones, acceptance criteria, and delivery sequence.
- `docs/DECISIONS.md` — durable architecture/product decisions and open questions.
- `CHANGELOG.md` — changes to this project.

## Ask your coding agent: "Show Observatory"

An optional, [on-demand Observatory Agent Skill](skills/adp-observatory/SKILL.md)
lets ADP-aware local coding agents discover how to show the project dashboard
without adding routine instructions or new records to ADP projects. The
[local launch helper](skills/adp-observatory/scripts/launch.py) searches for
previously approved Observatory installations, refuses project-local
executables, generates an offline report, and requests browser opening only
when a local desktop appears available. It reports an HTML path when running
headlessly.

Example from an agent's **trusted local Observatory checkout**:

```sh
python3 /path/to/ADP-Observatory/skills/adp-observatory/scripts/launch.py --project /path/to/ADP-project
```

The helper is standard-library only and performs **no downloads**. A user or
trusted environment must still approve and verify first-time installation;
there is currently no official protected public release or fully unattended
installer. Remote/cloud-only agents cannot guarantee opening a window on the
user's local computer. Agents may use a previously trusted executable
directly even when this helper is not locally available.

The canonical [ADP protocol](https://github.com/FrederikLive/ADP) contains
only a short conditional pointer to the skill. Daily agent work and ADP
conformance do not depend on Observatory.

## First Windows desktop pilot: lessons and fixes

An early Windows coding-agent pilot successfully built and ran the local analyzer but did not visibly open the generated report in the user's browser. The agent also created and pushed an optional project work index during setup—**outside the authorized scope of visualization**. These are genuine UX and orchestration gaps, not evidence that the scanner itself writes project files.

The Windows viewer now requests opening HTML files through the native Shell API using shell-compatible paths instead of passing canonical extended-length paths to Explorer. After generating the HTML, the CLI prints `Browser launch: REQUESTED`, `FAILED`, or `NOT_REQUESTED`; a successful launch request is **not proof** that the user's desktop displayed the tab. The optional [Agent Skill](skills/adp-observatory/SKILL.md) and [launcher](skills/adp-observatory/scripts/launch.py) use that explicit outcome and provide a confirmed report path as a fallback. They are instructed to leave the analyzed project and its Git history untouched.

The [Windows desktop pilot checklist](docs/WINDOWS-PILOT.md) explains how to test the updated executable **from a trusted source checkout or verified CI artifact**, diagnose a default-browser association, and distinguish browser visibility from a successful HTML report. CI validates Windows API compilation and path/status behavior; a real Antigravity desktop retest remains necessary. Do not assume older installed binaries or copied agent skills automatically acquire these fixes.

## One-command launch direction

The intended human experience is to enter an ADP project and run `adp-observatory .` to open its local dashboard. A separate headless `adp-observatory scan . --json` mode is planned for CI and agents. **The offline dashboard and headless scan are implemented in the development CLI**, and experimental binaries can be verified manually; no trusted installed launcher or official release exists yet. Today the application runs from a Rust checkout or experimental CI artifact.

Install-once, verified native releases are the preferred final delivery. Experimental Bash/PowerShell **preview installers** now support pinned CI artifacts and signed-provenance verification (all three OS installation smoke tests passed), but require a reviewed local script plus GitHub CLI. They are not a production installer. We will not use blind remote-script execution or require installation inside every ADP repository. See the [launcher and distribution design](docs/LAUNCHER.md) and [preview installation instructions](docs/RELEASING.md).

## Release readiness

A source/version-pinned, signed-provenance **release candidate** has passed Windows x64, Linux x64 and macOS ARM64 builds: [v0.4.0 candidate run](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39624399120). This is temporary CI output, **not an official GitHub Release or OS-code-signed installer**. The release-candidate workflow cannot publish a release.

Official publication is blocked until GitHub main/tag protection, required release approval and clean-machine acceptance are verified. The exact owner actions are tracked in [release-governance issue #1](https://github.com/FrederikLive/ADP-Observatory/issues/1) and [RELEASING.md](docs/RELEASING.md). The current preview installers require a reviewed local script and an independently installed GitHub CLI.

## Current prototype

The Rust CLI and offline HTML viewer now discover selected ADP markers and safely inspects optional `.project/WORK.json` records using the supported schema-v1 structural rules. It can detect explicit protocol versions, display a limited compatibility report, examine `docs/STATUS.md` and `docs/CONTINUITY.md`, and preserve conflicting active-contract claims with source references. Missing or invalid files do not stop independent analysis.

Semantic support currently means **WORK.json schema v1 for an explicitly identified ADP 2.7.1 project**, not complete understanding of the protocol. Older, newer, and unknown versions remain displayable where format parsing is safe, but are not declared fully supported. **Full ADP conformance is never assessed by this prototype.**

Build from a Rust checkout of this repository, then enter an ADP project directory and run:

```bash
cargo run --manifest-path /absolute/path/to/ADP-Observatory/Cargo.toml -- .
cargo run --manifest-path /absolute/path/to/ADP-Observatory/Cargo.toml -- scan . --json
```

The first command produces a **standalone offline HTML dashboard** in the user's application cache directory and attempts to open it in the default browser. The second produces headless JSON without opening a browser. Use `--no-open` to generate a dashboard without starting a browser, or `report . --output /outside/project/report.html` to choose an existing external output directory. Generated reports contain local project paths and should be treated as potentially sensitive.

The evolving JSON report schema is `0.4.0` in `schema/scan-report.schema.json` (analyzer development version `0.4.0`). It now separates traversal coverage from interpretation and reports bounded contract-file reference status; a referenced file being present does not prove conformance or verification. Tests cover current, older, future, malformed, conflicting, missing-record, and safe/unsafe contract-reference fixtures. CI also runs Observatory against its own ADP work records. The browser view is offline, has no listening HTTP server and uses no external assets. Experimental Windows/macOS/Linux GitHub Actions binaries have passed cross-platform tests and carry GitHub/Sigstore-signed build provenance, but are not operating-system code-signed or published as official releases. Manual verification instructions are in docs/LAUNCHER.md. Persistent metrics and historical snapshots are not implemented.

See [the roadmap](docs/ROADMAP.md) for milestones and acceptance criteria. Features are only considered implemented when the code, tests, documentation, and validation agree.

## Contributing

Contributions should be evidence-based, narrowly scoped, and compatible with the project's ADP workflow. Read `AGENTS.md` before making changes. External analysis is treated as untrusted input: independently assess each finding, preserve provenance, record disposition, and verify accepted changes.

## License

MIT. See [LICENSE](LICENSE).
