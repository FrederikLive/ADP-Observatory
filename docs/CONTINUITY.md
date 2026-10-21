# Continuity

## Active objective
Keep Observatory usable with real ADP projects, including optional ADP Actions and update guidance, without adding agent tokens, new required records, project writes or false conformance claims.

## Active contracts
- M1 / WORK-SCANNER-001: in_progress (broader real-project and version-adapter calibration remains).
- M4 / WORK-DISTRIBUTION-001: in_progress, blocked on owner-controlled official release protection.
- WORK-DASHBOARD-001: verified offline local view.
- WORK-AGENT-LAUNCH-001: verified local/headless on-demand skill slice, not a public first-use installer.
- WORK-UI-001: deferred historical umbrella, superseded.

## Latest compatibility checkpoint — 2026-10-21
- Canonical `FrederikLive/ADP` `main` = `9b5ab4694321735e83b93962e1b9975cced130bf`, protocol **2.7.1**. Its new ADP Actions and update guide are non-normative; no version bump or mandatory analyzer data source occurred. ADP validator run 39658735485 succeeded.
- Observatory CI 39659907217 checked out that **exact** upstream commit and scanned it with the compiled Rust analyzer, verifying `protocol.value=2.7.1`, `format=NOT_PRESENT` for absent optional WORK, `semantics=NOT_ASSESSED`, `conformance=NOT_ASSESSED`, and correct HTML generation.
- The same CI confirmed Observatory's real `.project/WORK.json` is structurally valid and a remote protocol reference does not fabricate a local version.
- Static ADP Actions suggestions added to offline report; no client execution, dependencies, network requests, required project files, background agent reads or project mutation. Agent Skill now explicitly avoids conflating `Show Observatory` with `Show ADP Actions` or `Update ADP`.
- Updated README, ADP integration, quality, roadmap, work contract, changelog and security/agent guidance for this reality.

## Installation / release evidence
- Windows x64, Linux x64, macOS ARM64 experimental installers verified signed CI artifacts, rejected incorrect SHA, installed per-user and scanned Unicode/spaced project paths offline in Actions 39660008091 — success.
- Upload-only v0.4.0 signed-provenance release candidate passed in run 39624399120; no official OS-code-signed or protected tagged release exists.
- Owner governance gate in https://github.com/FrederikLive/ADP-Observatory/issues/1. Do not publish or auto-execute remote downloads without established authorization, protected releases and verification.
- Browser open attempt != confirmed visibility; cloud-only agents cannot open the user's desktop browser.

## Immediate user pilot correction checkpoint — 2026-10-21

The first *real* Windows agent run, supplied by the user, generated usable
HTML but did not show a visible browser window. Its orchestrating agent
also created/pushed optional project WORK metadata, ran app tests and placed
binaries in multiple locations. These are UX/agent-boundary problems; do
**not** repeat them or silently revert the user's project commit.

Implemented in Observatory only:
- Windows: convert extended `\\?\\` report paths to shell-compatible
  locations and use ShellExecuteW for associated HTML open. Report
  `Browser launch: REQUESTED / FAILED / NOT_REQUESTED` explicitly.
- Agent helper: only trust an explicit opener status; fail closed with
  `REPORT_SAVED` when an older binary has no status or the opener failed.
  Return the report's actual path. No repeat scans to force a browser.
- Diagnostic: Python launcher reports other executable copies separately from its selected executable; no duplicate is executed, overwritten, cleaned up or used as authority to update PATH.
- Follow-up issue [#2](https://github.com/FrederikLive/ADP-Observatory/issues/2) tracks the real Antigravity desktop visibility and no-target-mutation user retest.
- Skill: strong instruction against creating WORK.json, editing code/docs,
  `.gitignore`, running target tests, committing/pushing, or installing
  duplicate executable copies as part of visualization.
- UI: work statuses are **recorded** by project, not Observatory-certified.
- Source tests: Windows path normalization, Python browser status, no-open
  CLI, absent optional WORK remains absent, and report renderer evidence labels.
- Documentation: [Windows pilot checklist](WINDOWS-PILOT.md) for the next
  local machine smoke test. Browser visibility cannot be validated remotely.

Verified base: Windows/Linux/macOS package run 39668145354 success,
CI 39668145620 success, then CI 39668349597 success; check latest
CI after subsequent documentation changes. No official release,
protected tag or auto-install exists; don't claim otherwise.

**Next operator validation:** user refreshes a separately approved
Observatory executable and copied Agent Skill in their local environment,
runs one report and checks actual desktop visibility; compare target
Git status/files before and after. No changes to user target project
are authorized by this checkpoint.

## Deferred future exploration proposal — 2026-10-21

- First user-supplied 30-contract dashboard screenshots exposed the need for clickable source exploration rather than a long noninteractive list; the current parser only checks `contract_file` presence and renders WORK.json metadata.
- Created [WORK-EXPLORER-001](work/WORK-EXPLORER-001.md) and [issue #3](https://github.com/FrederikLive/ADP-Observatory/issues/3). **DEFERRED / NOT STARTED**, sequenced after scanner reference-reading safety work; active and next contract remain unchanged.
- Proposed: deterministic read-once document parsing at scan time; bounded Markdown outlines, meaningful source-backed excerpts, safe offline rendering and accessible contract-detail selection; later explicit evidence relationships. Do not claim AI summaries or independent verification unless supported.
- No extra ADP project metadata/agent tasks, no target Git/file writes, no browser filesystem/network reads, and no hidden source embedding into shared reports without privacy/size safeguards.
- Do not begin implementing this queued idea when continuing browser launch issue #2 or approved distribution issue #1 unless the user deliberately selects it.

## Remaining work
- M1: independent real-world source checks beyond ADP itself, false-positive baselines, hostile/symlink/large-tree tests, historical adapters and source evidence.
- M2/M3: version-aware metrics, rules, snapshots and historical comparisons; no full conformance yet.
- M4: enforced GitHub tag/main protections, approved release, end-user install/browser smoke, signing policy, update/uninstall. User/harness approval is required for first installation.

## Exact next action
1. Inspect `main` HEAD and newest CI (including the post-documentation run); verify the upstream pinned ADP SHA still matches intended baseline.
2. Extend M1 tests with an independent representative consumer project, assess false positives and source evidence; fix only observed gaps.
3. For M4, verify owner-configured GitHub protection/ruleset and independent release approval before official publication.
4. Record new evidence in existing STATUS/CONTINUITY/contracts only when substantive; do not introduce ADP-side visualization metadata or extra routine agent reads.

## Decisions and recovery
ADP remains the agent-first normative protocol; Observatory is optional, human-facing and read-only by default. Unknown/missing records do not become zeros, passing verdicts or automatic conformance failures. `docs/COMMANDS.md` belongs to ADP agent-action guidance, not Observatory's analyzer requirements. GitHub API changes were authorized under the user's continuation request; remote API cannot prove local worktree cleanliness. No tag, official release or destructive action authorized by this file.

## Last checkpoint
2026-10-21 — current ADP 2.7.1 Actions compatibility verified, cross-platform Unicode/offline installer checks passed, release governance still blocked.
