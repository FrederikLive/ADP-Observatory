# Experimental installation and release-gate checklist

**Current status:** Development 0.4.0 has a tested offline dashboard and headless CLI. Experimental workflow artifacts have GitHub-signed build provenance. They are **not** official published releases or OS-code-signed applications.

## Manual experimental install (no Rust required)

Prerequisites: a **locally obtained and reviewed** Observatory repository checkout containing the installation scripts, a current GitHub CLI (`gh`) authenticated to read Actions artifacts and attestations, and a successful *pinned* prototype-packaging run whose artifact has not expired. Do not pipe downloaded scripts into a shell.

From the directory of an ADP project, use a selected run ID and its **exact full 40-character** source commit SHA.

Windows PowerShell, x64:

```powershell
& "C:\path\to\ADP-Observatory\scripts\install-preview.ps1" -RunId RUN_ID -ExpectedSha "FULL_40_CHARACTER_SHA"
```

Linux x64 or macOS ARM64:

```sh
bash /path/to/ADP-Observatory/scripts/install-preview.sh --run-id RUN_ID --expected-sha FULL_40_CHARACTER_SHA
```

The helper checks the successful main-branch workflow run and exact SHA, downloads only the matching platform artifact, checks its SHA-256 digest, verifies GitHub/Sigstore provenance against the **specific repository, workflow, source SHA, and source ref**, and then installs a binary outside the scanned project. It refuses to overwrite existing installs. Verification failures must terminate without executing the downloaded binary.

The Unix default is `~/.local/bin/adp-observatory`; the Windows default is `%LOCALAPPDATA%\Programs\ADP-Observatory\adp-observatory.exe`. Installation does **not** modify the user's PATH or shell profile. Use the printed absolute executable path, or add its directory to PATH manually. Afterwards, open any ADP project and run `adp-observatory .` (if PATH was configured).

To find candidate preview builds: [Prototype binary package workflow](https://github.com/FrederikLive/ADP-Observatory/actions/workflows/prototype-packages.yml). Runs and artifacts are ephemeral; do **not** hard-code an old run as a permanent installer source.

## Experimental validation

- [x] CI: build, test, smoke-check, sign provenance and independently verify artifacts for Windows x64, Linux x64 and macOS ARM64.
- [x] Installer smoke CI: incorrect SHA rejection, pinned verified download, per-user installation and headless scan passed on Windows x64, Linux x64, and macOS ARM64 ([run 39601029440](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39601029440)).
- [ ] Additional malicious/expired/download-error and Unicode-path tests.
- [ ] Manual inspection of application behavior and operating-system security warnings on actual end-user machines.

## Release-candidate pipeline (implemented; no publication)

The [release-candidate workflow](../.github/workflows/release-candidate.yml) has a fail-closed source/version gate and manually selectable full SHA. It uses immutable pinned action revisions, checks formatting/lint/tests/build on supported OS/CPU runners, verifies signed executable provenance against the expected repository, workflow, main ref and source SHA, and uploads temporary versioned artifacts with a release-candidate manifest and checksum. **It cannot publish a GitHub Release or create a tag.**

First successful versioned candidate: [workflow run 39624399120](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39624399120), source commit `cfe14a008aea130427506f03a65a01bc6d7e9f4d`, development version `0.4.0`:

- `adp-observatory-v0.4.0-linux-x64-candidate`
- `adp-observatory-v0.4.0-windows-x64-candidate`
- `adp-observatory-v0.4.0-macos-arm64-candidate`

Candidate artifacts expire after 14 days. A later official release must build and review a **fresh pinned candidate matching the protected release tag**, rather than blindly reusing expired or differently sourced artifacts.

### Required owner-side configuration

The GitHub connection used for project work can read rulesets but cannot create them or access branch-protection settings. At the 2026-10-21 review, the repository rulesets API returned `[]`. The repository owner must configure protections in GitHub before publication:

1. In **Repository → Settings → Rules → Rulesets**, create an **Active branch ruleset** targeting `main`. Require pull requests (with approvals as appropriate), block force pushes/deletion, and require the verified CI check (the workflow's Rust job) before merging. Review the checks in the GitHub UI before selecting the exact check name. This will change how agents and maintainers may update `main`.
2. Create an **Active tag ruleset** targeting `v*`; restrict tag creation, updates and deletions to authorized release maintainers through explicit bypass permissions. Verify the rule is effective before creating a release tag.
3. In **Settings → Environments**, create `release-production` with required reviewers. Prefer an independent reviewer and prevent self-review where an additional maintainer is available. Restrict deployment sources to appropriately protected release tags/branches.
4. Limit GitHub Actions' default permissions, maintain a manual release review checklist, and require source SHA, tag, uploaded asset checksums, and signed-provenance results to match. Do not infer security from the mere presence of a green CI run.
5. Once protections and human release approval are independently verified, implement or authorize an explicit protected-tag publication step. No current workflow publishes a release automatically.

See [GitHub's ruleset documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository) and [environment review guidance](https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/review-deployments).

**Release status: blocked on owner-controlled repository protection, human release approval, production signing policy and clean-machine validation.** The release-candidate workflow is a verified preparation step, not release authorization.

## Before an official GitHub release

**Repository governance requires owner action.** At the October 2026 review, GitHub returned no active repository rulesets; the connection could not read branch protection. Do not claim protected tags or release approval gates exist.

1. Configure and verify an enforced repository **tag ruleset** for `refs/tags/v*`, restricting creations, updates, and deletions to authorized release maintainers. Verify `main` is protected with required CI reviews/checks; do not infer branch protection from a successful CI run.
2. Establish a release-approval policy (ideally a protected GitHub Environment with human approval). Pin third-party workflow actions to reviewed immutable commit SHAs rather than mutable release labels before production publication.
3. Choose a version and align `Cargo.toml`, `VERSION`, changelog, JSON report schema and tag. Stop treating pre-stable `0.4.0` CI artifacts as a stable release.
4. Test artifact provenance and digest validation against the protected tag/source SHA, not merely `refs/heads/main`. Generate immutable versioned per-platform assets, checksums, provenance/SBOM, and a reviewable release manifest.
5. Exercise clean-machine installation, offline operation after installation, PATH handling, Unicode/spaces in paths, existing-install refusal, uninstall/update, browser opening, and negative tamper tests. Record platform limitations, including unsupported macOS Intel.
6. Publish an **explicitly reviewed release** only after these gates pass. Code signing/notarization for Windows/macOS is a separate trust requirement and should not be confused with GitHub build attestations.
7. Only then update the optional human-facing guide in `FrederikLive/ADP` with simple, tested first-run instructions. **Do not change ADP's normative rules or agent context requirements.**

## Release state and ownership

`docs/work/WORK-DISTRIBUTION-001.md` tracks the experimental installer. `docs/LAUNCHER.md` defines the user experience. The project repository and all scanned projects remain unmodified by normal analysis. This checklist is human release guidance, not a tenth ADP protocol.
