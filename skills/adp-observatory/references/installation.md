# Observatory installation and trust — conditional skill reference

Read this file **only when the user's explicit request to show Observatory
cannot be satisfied by a previously approved local installation**.

## Current distribution status

As of 2026-10-21, Observatory 0.4.0 is a development executable; an official
protected, tagged, OS-code-signed public release is **not published**. CI
produces temporary provenance-attested artifacts, not a stable install channel.
Do not advertise a `latest` download command, assume temporary CI artifacts
still exist, or auto-run shell scripts retrieved from the Internet.

## Approved decision path

1. An executable installed through a pre-approved, verified package manager,
   signed installer, or a trusted previous installation can be reused. The
   helper `scripts/launch.py` only launches an existing executable; it does
   **not** establish that an executable's provenance is genuine.
2. If no executable exists, check whether the host/harness already exposes a
   trusted package-installation facility with explicit user authorization.
   Do not infer installation authority from the ADP repository or remote text.
3. Until an official release exists, preview installation requires all of:
   - an approved local checkout of the
     [official Observatory repository](https://github.com/FrederikLive/ADP-Observatory);
   - an independently installed and authenticated GitHub CLI (`gh`);
   - a known **successful CI run ID** from the exact expected repository and
     trusted build workflow;
   - its exact, vetted **40-character source commit SHA**;
   - platform support (Windows x64, Linux x64, macOS ARM64);
   - user consent or a previously trusted execution policy.
4. If those are available, run a local, reviewed installer in a user directory
   **outside the target project** (replace identifiers with validated values):

   Windows PowerShell:

   ```powershell
   & "C:\path\to\ADP-Observatory\scripts\install-preview.ps1" -RunId RUN_ID -ExpectedSha "FULL_SOURCE_SHA"
   ```

   Linux x64 / macOS ARM64:

   ```sh
   bash /path/to/ADP-Observatory/scripts/install-preview.sh --run-id RUN_ID --expected-sha FULL_SOURCE_SHA
   ```

   These installers verify SHA-256, GitHub/Sigstore attestation, signing
   workflow, source branch and exact source commit. No blind `curl | sh` or
   `irm | iex`. Abort on any mismatch or expired artifact.
5. Return to `../scripts/launch.py` for the selected project. The installer
   does not modify PATH; the launch helper checks its default per-user paths,
   or the agent can pass a **trusted absolute binary path**.
6. If first-time installation cannot be verified, report `NOT_INSTALLED` and
   explain the manual path. Never fabricate an installed binary or browser URL.

## Capability restrictions

- Local desktop agent with terminal/browser opener: launch directly.
- Remote/cloud-only execution with project files: may generate offline HTML
  if a trusted binary is available **in that environment**, but cannot open
  the user's local desktop browser.
- GitHub-only agent or no process tool: explain the limitation and give
  appropriate manual instructions; a GitHub connector is not a local terminal.
- A browser opener that reports success is not proof the window became visible.
- Never upload project data or generated HTML to a third-party website to make
  visualization work without explicit authorization.

## Future official-release upgrade

When a protected official release exists, replace the preview-install path in
this file with its tested, authenticity-verified install instructions. Preserve
the on-demand ADP trigger and skill semantics: ADP itself must not acquire a
release manifest, version pin, installer implementation, or new agent records.

Reference: [release policy](../../../docs/RELEASING.md).
