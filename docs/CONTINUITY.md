# Continuity

## Active objective
Deliver an official safe optional Observatory release while protecting ADP's agent-first efficiency and support for incomplete/older project records.

## Active work and ownership
M1 scanner calibration remains active: WORK-SCANNER-001, in_progress. M4 distribution: WORK-DISTRIBUTION-001, in_progress, now blocked on repository-owner release governance. Offline viewer WORK-DASHBOARD-001 verified. Original WORK-UI-001 deferred/superseded with history retained.

## Last known good state
- Development 0.4.0 supports local offline dashboard `adp-observatory .`, explicit HTML export, and `scan . --json`. It does not offer full ADP conformance or historical snapshots.
- Preview installers passed install and scan tests for Windows x64, Linux x64 and macOS ARM64 (run 39601029440), requiring gh CLI, full SHA and a pinned CI run.
- Release candidate workflow `.github/workflows/release-candidate.yml` added with `scripts/check_release_candidate.py` and `tests/test_release_validation.py`. It rejects wrong SHA/ref/versions and produces signed-provenance temporary candidate assets, **without any GitHub release publishing permissions or steps**.
- Release-candidate run 39624399120 succeeded for all three platforms at exact source commit cfe14a008aea130427506f03a65a01bc6d7e9f4d.
- docs/RELEASING.md contains owner-side GitHub branch/tag ruleset and release-environment setup steps; release-governance issue #1 tracks manual owner actions.
- Added Unicode/spaced-path/offline installer checks to the existing installer smoke workflow. Their new follow-on CI has not completed and must remain NOT VERIFIED.

## Blocker / investigation
- No repository rulesets existed at last check. The connector cannot create these admin controls or read branch protection (403). Owner must configure and verify active main/v* tag rulesets and release approval environment before publication.
- Candidate binaries have signed build provenance, **not** operating-system signatures. Clean-machine acceptance and platform signing policy remain open.
- Release candidate artifacts are temporary, not tagged production artifacts; no official release has been published.

## Exact next action
1. Inspect repo HEAD, rulesets, branch protections if accessible, newest CI (especially pending Unicode/offline smoke test) and active contracts.
2. Ask/check that repository owner completed [release-governance issue #1](https://github.com/FrederikLive/ADP-Observatory/issues/1); validate enforced main/v* rulesets and deployment environment with actual evidence.
3. Finish end-user install tests and approve version/signing/release caveats.
4. Produce a fresh source-pinned candidate matching a protected release tag, compare assets/attestations, and only then publish through a reviewed gated process. No automatic or unapproved publication.
5. Update STATUS/CONTINUITY with exact verification. Return to M1 scanner calibration after the distribution release milestone is unblocked.

## Decisions maintained
- Canonical ADP remains normative agent-first; Observatory is a non-authoritative optional analysis companion, requiring no added project metadata or agent context.
- Unknown/unsupported/incomplete evidence must never become a zero, false conformance failure or passing verdict.
- Signed GitHub build provenance is distinct from OS code-signing; release approval requires owner-controlled protection.
- No release/tag creation, destructive Git operation or deployment was performed in this checkpoint.

## Last checkpoint
2026-10-21: v0.4.0 upload-only release candidate green (39624399120), tag/main rulesets absent, governance issue #1 created, new installer regression tests pending.

## Git authorization and recovery
Github main updated through connector under the user's continue directive. Local working-tree cleanliness is unknowable from remote state. No durable authorization to bypass review or release-governance policy exists.
