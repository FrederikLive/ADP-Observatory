# Project Status

## Current milestone / phase
M1 — scanner calibration, active contract WORK-SCANNER-001 (in_progress). Parallel M4 — authenticated release/distribution, contract WORK-DISTRIBUTION-001 (in_progress). WORK-DASHBOARD-001 verified. WORK-UI-001 is superseded and preserved for history.

## Verified outcomes
- Offline HTML dashboard and headless scan, development version 0.4.0: formatting/lint/unit/CLI/build CI passed (including 39624399120).
- Three-platform preview installer pipeline: full pinned download, signature/checksum verification, install and headless scan passed earlier on Windows x64, Linux x64 and macOS ARM64 ([39601029440](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39601029440)).
- **Version-pinned release candidate v0.4.0**: source/version gate and Windows/Linux/macOS release-candidate builds passed; signed executable provenance verified against repository/workflow/ref/full SHA. [Candidate 39624399120](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39624399120), source cfe14a008aea130427506f03a65a01bc6d7e9f4d.
- Upload-only release-candidate workflow uses pinned action SHAs, creates temporary manifest/checksum artifacts, and has **no release publication step**.
- Only optional, human-facing ADP integration guidance was changed. No normative ADP protocol modification, agent token overhead, telemetry or target-project writes.

## Optional on-demand Agent Skill (verified slice)

- `skills/adp-observatory/SKILL.md` handles an explicit user request such as "show Observatory"; user-triggered discovery was added as a tiny, non-normative pointer in ADP's full/compact guidance and generated agent-router template.
- Python standard-library `skills/adp-observatory/scripts/launch.py` detects existing approved binaries, refuses executables within the project, generates HTML through the offline dashboard and returns a structured browser-attempt or headless report state.
- Unit tests and an actual compiled CLI/report integration test passed: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39652171472. WORK-AGENT-LAUNCH-001 tracks the verified subset.
- First-time automatic installation and actual user-desktop browser visibility are **not verified**; no official protected release exists and remote execution cannot guarantee a local browser window.
- No additional ADP-mandatory files, normal-agent readings, or workflow steps introduced.

## Current limitations and release blocker
- **No official GitHub release or trusted zero-prerequisite installer.** Experimental and candidate assets expire after 14 days and are not Windows/macOS OS-code-signed/notarized.
- GitHub repository rulesets API returned an empty array; branch protection could not be inspected with current connector permissions. Release requires enforceable protected tags/main, human approval and independent end-user testing.
- Owner action tracked in [release-governance issue #1](https://github.com/FrederikLive/ADP-Observatory/issues/1). See docs/RELEASING.md for exact GitHub UI steps.
- Unicode/spaced-path and offline installer tests were added to workflow and are still **NOT VERIFIED** while the follow-on workflow is queued. Do not mark these passed until specific CI confirms.
- Full conformance engine, historical snapshots, version adapter matrix and broad real-project scanner calibration remain outstanding.

## Next action
1. Repo owner configures active main and v* tag rulesets and a protected release-production approval environment; verify effectiveness.
2. Inspect the queued Unicode/offline installer follow-on CI and repair failures if needed.
3. Perform real end-user platform/OS security testing and signing policy review, then generate a fresh candidate pinned to the intended protected release tag SHA.
4. Publish only after owner release approval, with accurate pre-release scope and verification metadata; update ADP's optional human guide after that.

## Verification / Git
- Last fully verified candidate: https://github.com/FrederikLive/ADP-Observatory/actions/runs/39624399120.
- Latest documentation edits are committed via GitHub connector on main; inspect HEAD and Actions for exact latest SHA.
- Remote API cannot establish local staged/unstaged state. No tag, release, or destructive rewrite created.
