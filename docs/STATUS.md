# Project Status

## Current milestone / phase
M1 scanner calibration: WORK-SCANNER-001 (in_progress). Parallel M4 trusted distribution: WORK-DISTRIBUTION-001 (in_progress, official release blocked by owner-controlled GitHub governance). Offline viewer WORK-DASHBOARD-001 and on-demand Agent Skill WORK-AGENT-LAUNCH-001 are verified implementation slices. Superseded WORK-UI-001 remains deferred for history.

## Latest ADP compatibility (verified 2026-10-21)
- Canonical upstream ADP `main` revision `9b5ab4694321735e83b93962e1b9975cced130bf` declares **protocol 2.7.1**, despite recent optional ADP Actions/project-update documentation. Upstream validation succeeded: https://github.com/FrederikLive/ADP/actions/runs/39658735485.
- Observatory CI pinned that real ADP revision, scanned it and generated HTML: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39659907217. Verified version KNOWN/2.7.1, optional WORK index NOT_PRESENT and semantic/conformance NOT_ASSESSED, without false errors. Also self-scanned Observatory's own structurally valid work index.
- The offline dashboard now presents **static optional ADP Actions prompts**. They are handled by the coding agent, not the dashboard; no execution, background check or required ADP metadata was added.
- The optional Agent Skill expressly separates “Show Observatory” from “Show ADP Actions” and “Update ADP.” The launcher uses an already approved local executable and generates offline HTML; local browser-opening remains environment-dependent.

## Verified implementation / delivery evidence
- Development analyzer/report schema 0.4.0; bounded read-only discovery, optional WORK schema v1, safe contract references, source-backed version/conflict results, standalone HTML, headless JSON.
- Rust format/lint/tests/build, Python Agent Skill tests and actual compiled CLI launch passed at pinned-ADP run 39659907217. Additional docs/skill/layout edits must be checked against the newest CI.
- Experimental Windows x64, Linux x64 and macOS ARM64 preview installation passed (including signature-verification, wrong-SHA rejection, Unicode/spaced-path projects and offline reports): https://github.com/FrederikLive/ADP-Observatory/actions/runs/39660008091.
- Version-pinned upload-only 0.4.0 release candidate passed: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39624399120. Artifacts are temporary and **not an official OS-signed release**.

## First Windows coding-agent pilot corrections — 2026-10-21

- A real local Windows agent generated the HTML report and scanned the target, but did not visibly open the user's browser. The agent also copied multiple binaries/reports and unnecessarily created, committed and pushed a WORK.json in the target project. The executable itself did not require these writes.
- **Duplicate install diagnostics:** The optional Python launcher reports the chosen executable and any other discovered local installations without executing, updating or deleting them. This is intended to expose version drift from the first user pilot.
- **Code:** Windows HTML opening now uses native ShellExecuteW with shell-compatible paths. CLI prints an explicit browser launch status; Python Agent Skill helper only claims an attempted open after receiving the positive marker.
- **Safety:** Agent Skill/setup guide prohibit target work-index creation, application tests, Git changes/pushes, unnecessary duplicate installations, and repeated reports during visualization.
- **Accuracy:** Dashboard differentiates statuses recorded in WORK.json from independent verification, and optional WORK remains optional.
- **Automated verification:** Rust format/lint/tests and Windows/Linux/macOS native package checks passed after the P0 changes (CI 39668145620; package 39668145354), with additional launcher safety and wording passing CI 39668349597. Final documentation-only commits require latest CI check.
- **Not yet demonstrated:** Visible browser tab in the user's actual Antigravity session and a manual confirmation that the target project's files/Git state were untouched. Instructions and evidence fields: [docs/WINDOWS-PILOT.md](WINDOWS-PILOT.md).

## Future dashboard feature captured (not implemented)

- Deferred **WORK-EXPLORER-001**: [Contract & Evidence Explorer](work/WORK-EXPLORER-001.md), tracked in [issue #3](https://github.com/FrederikLive/ADP-Observatory/issues/3), proposes clickable contract/context records with **precomputed, bounded, source-traceable Markdown sections and a safe offline rendered viewer**.
- Today's scanner checks contract-file presence but does **not** parse the referenced Markdown contents. Proposed deterministic analysis must distinguish recorded claims from independently verified evidence and keep source lines/limits visible.
- This is a **future M3 enhancement only**. No runtime code, JSON schema, mandatory ADP files, project writes or changes to active/next contracts have been introduced.

## Open items / release governance
- Official protected GitHub release, operating-system code signing/notarization, one-prompt unattended first-time installation and actual local-desktop browser smoke tests are **not complete**. GitHub rulesets were absent at the last review; owner-controlled protections/approvals remain necessary. See https://github.com/FrederikLive/ADP-Observatory/issues/1.
- M1 remains open for wider representative user-project calibration, historical version adapters, hostile-input/race hardening, evidence graph and full JSON Schema coverage.
- M2 rules/metrics and M3 history/snapshots are not implemented; full ADP conformance remains NOT_ASSESSED.
- Missing optional ADP Actions/Updater documents are **not** evidence of nonconformance.

## Next recommended phase
Complete M1 real-project scanner calibration and fix only reproducible compatibility gaps. In parallel, the repository owner should configure and verify branch/tag rulesets and release approval before a trusted general-user installer is published. Do not change canonical ADP to accommodate Observatory.

## Git / verification recovery
- Remote GitHub HEAD/CI should be inspected before the next code mutation; API access cannot attest the state of a local checkout.
- The upstream ADP integration baseline is pinned in CI to the exact tested commit. Updating that pin requires reviewing the new upstream diff, not blindly following a moving branch.
- No tag, official release, destructive Git rewrite, target-project mutation or ADP normative change was performed for this compatibility work.
