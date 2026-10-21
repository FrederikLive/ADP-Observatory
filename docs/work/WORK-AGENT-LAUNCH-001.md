# WORK-AGENT-LAUNCH-001 — Optional on-demand Observatory Agent Skill

## Outcome

A user can request "show Observatory" (or an equivalent project visualization)
from an ADP-aware local coding agent without adding new mandatory ADP records,
routine agent context, or unverified automatic downloads.

## Implementation

- `skills/adp-observatory/SKILL.md` — progressively disclosed, user-triggered
  workflow and agent execution/trust boundaries.
- `skills/adp-observatory/references/installation.md` — loaded only when
  first-time installation is necessary. No official public release exists yet.
- `skills/adp-observatory/scripts/launch.py` — Python-standard-library
  helper, existing trusted local executable discovery, project-local executable
  refusal, desktop/headless detection, structured JSON outcomes and offline
  report path; **no network calls or installer**.
- `tests/test_agent_launch.py` — negative path, missing binary, headless,
  browser-attempt, and Unicode report-path cases.
- `.github/workflows/ci.yml` — runs Python skill tests and the compiled
  Observatory binary through the helper, verifying the HTML report is saved
  outside the project.
- A tiny non-normative pointer in `FrederikLive/ADP` (portable/compact
  guidance and agent-router template) links to this canonical skill.

## Acceptance

- [x] The skill is not loaded for ordinary development, bootstrap or handoff.
- [x] Official skill URL is discoverable from new ADP agent-routing guidance.
- [x] The helper does not download/execute remote scripts or use project files
  as executable input.
- [x] Existing approved executable launches the offline dashboard with
  explicit project root; headless mode returns generated report path.
- [x] Missing executable is reported instead of falsely claiming success.
- [x] Python regression tests and end-to-end Rust executable integration run
  through CI.
- [x] No new required files, schema fields, or normative changes in ADP.
- [ ] Trusted first-time auto-install without GitHub CLI or a source checkout.
  **Explicitly outside the current skill slice:** depends on separately
  authorized official release work WORK-DISTRIBUTION-001.
- [ ] Cross-harness smoke tests on actual end-user desktops. Requires local
  agent/browser capabilities and independent test environments.

## Verified evidence

- [Agent skill E2E CI run 39652171472](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39652171472): passed format, lint,
  Rust tests, Python launcher tests, compiled binary, offline HTML report.
- ADP validation runs after the non-normative references: passed; inspect
  current Actions history for the exact document revision.
- Browser visibility from a cloud-only agent and official installation remain
  **NOT AVAILABLE**, not success.

## State

Initial on-demand launch slice **VERIFIED** for the tested local headless
environment; broader end-user one-prompt installation is **not implemented**.

## Next step

Continue WORK-DISTRIBUTION-001 to establish protected official releases and
harness-native approved installation. Do not duplicate release/trust metadata
in ADP or move analyzer implementation into the protocol.

## ADP Actions routing separation — 2026-10-21

The latest ADP introduces **ADP Actions** as non-normative natural-language
requests. The Observatory skill now expressly distinguishes its own
visualization trigger ("Show Observatory") from "Show ADP Actions",
"Check ADP for updates", and "Update ADP". Those latter requests must be
handled by the ADP Actions catalogue and project-update guidance without
launching a browser, installing Observatory or changing work records.

The offline Observatory report displays a small, static set of relevant
agent prompts. No HTML element executes these prompts or modifies the
project. The pinned upstream compatibility test in [CI 39659907217](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39659907217)
confirmed the new ADP guidance and report are coherent. The complete
one-prompt first-time installer remains gated by official release
protections and local host authorization.
