# Launching and distributing ADP-Observatory

**Status:** Development 0.4.0 provides the offline dashboard and headless CLI. CI builds signed-provenance experimental Windows x64, Linux x64, and macOS ARM64 binaries. Preview installers now pass pinned-download, attestation-verification, install and scan smoke tests on all three platforms (run 39601029440). **No official protected release, OS code-signed installer, or zero-prerequisite first-run downloader exists yet.**

## User experience

The desired experience is to open a terminal **inside any ADP project** and run:

```text
adp-observatory .
```

The 0.4.0 development executable accepts this command, treats the current working directory as the project, generates a single offline HTML report outside the project, and attempts to open it in the browser. No local web server is required for this first interface. The user should not copy any Observatory files into their project.

For scripted, noninteractive, and agent usage, the desired command is:

```text
adp-observatory scan . --json
```

The development executable now supports this headless command. The legacy `adp-observatory PROJECT --json` shortcut remains supported. Both report-format compatibility and release-distribution guarantees are **pre-stable**.

## Experimental builds: inspect and verify before executing

A successful experimental matrix run is available at [GitHub Actions #39597842680](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39597842680). It also re-verifies each executable's signed attestation against this repository and the expected signing workflow before uploading the binary. Each platform job tests, builds, smoke-tests, records SHA-256, creates a GitHub/Sigstore-signed build provenance attestation for the binary, and uploads a workflow artifact.

**Manual testing only; this is not a public release.** To check an experimental binary:

1. Open the successful workflow run and download the artifact for the correct OS and architecture.
2. Extract the binary and `SHA256SUMS.txt`. Check the SHA-256 file digest against the supplied checksum to catch corruption, but do not treat that alone as publisher authentication.
3. Using an independently installed GitHub CLI that supports artifact verification, validate provenance **before execution**, from the extracted directory:

   ```sh
   gh attestation verify ./adp-observatory --repo FrederikLive/ADP-Observatory
   ```

   On Windows, replace the binary name with `adp-observatory.exe`. A successful verification should identify GitHub's signed attestation associated with this repository. **Do not execute if verification fails or provenance unexpectedly names another repository/workflow/commit.**
4. Invoke that verified binary by its explicit filesystem path from within the desired ADP project. For example on Linux/macOS, `/path/to/adp-observatory .`; in PowerShell, `& "C:\\path\\to\\adp-observatory.exe" .`. The application then generates the offline report and attempts to open it in the browser.

Do not bypass operating-system security warnings to run an unsigned executable. Artifact provenance is not equivalent to OS code signing or a trusted installer. The release source/branch may change during development: pin a known workflow run for repeatable testing instead of assuming the newest artifact is vetted.

## Install once using the experimentally verified helpers

See [RELEASING.md](RELEASING.md) for prerequisites and the complete preview instructions. From an ADP project directory, invoke the **locally reviewed** helper with an explicit successful packaging run ID and full source SHA:

Linux x64 / macOS ARM64:

```sh
bash /path/to/ADP-Observatory/scripts/install-preview.sh --run-id RUN_ID --expected-sha FULL_40_CHARACTER_SHA
```

Windows x64 PowerShell:

```powershell
& "C:\path\to\ADP-Observatory\scripts\install-preview.ps1" -RunId RUN_ID -ExpectedSha "FULL_40_CHARACTER_SHA"
```

The helpers require an independently installed, authenticated GitHub CLI (`gh`). They reject a mismatched run/SHA, verify the binary checksum and GitHub signature plus source workflow/commit, and install outside the ADP project. They do not change PATH; use the printed executable path or configure PATH yourself. CI successfully exercised both successful installation and deliberate wrong-SHA rejection on Windows, Linux and macOS: [installer smoke run 39601029440](https://github.com/FrederikLive/ADP-Observatory/actions/runs/39601029440).

These are **developer preview installers**, not a general-audience one-command downloader or official release. Unsupported architectures, expired artifacts, or failed verification must fail closed.

## First-run trade-off: one command versus trusted execution

A bare `adp-observatory .` command requires an executable to be installed or a launcher registered in the user's shell. A truly zero-install one-liner can fetch a binary on first use, but its bootstrap code must establish a trustworthy verification path. Merely downloading a checksum from the same compromised origin is not sufficient to establish trust.

**Recommended delivery model:**

1. **Default:** trusted one-time installation using official signed platform packages or a reputable OS package manager. Thereafter, `adp-observatory .` is one command on every project.
2. **Optional first-run helper:** a small published, platform-specific, user-invoked launcher (Windows: PowerShell/installer; Linux/macOS: shell or signed native stub) that chooses a supported binary, verifies the publisher's signature/release attestation or pinned trusted metadata, caches it per user, then executes it with the current project directory.
3. **Advanced/offline:** download and verify a release manually, or run the binary from a specified path.
4. **Never:** promise a working installer before publishing one, use `curl | sh` / `irm | iex`, run unverified downloads, silently update, request elevation for ordinary usage, or place binaries and caches in the scanned repository.

A single OS-agnostic install-and-run line is not a portable promise. Publish explicit instructions for Windows PowerShell, macOS, and Linux only for platforms actually supported by tested artifacts.

## Trust and update model

- Release artifacts are built by protected CI on pinned sources and published under explicit immutable version tags.
- Support explicit platform/CPU combinations; fail clearly on unsupported systems.
- Prefer independently verifiable signing/provenance with a pinned trust anchor, supported package-manager verification, or a signed native installer.
- SHA-256 checksums help detect accidental corruption but are not a substitute for verified publisher identity when distributed alongside an executable.
- Download to temporary storage; verify before atomically placing in a user-specific cache/install directory.
- No administrator/root requirement, silent startup network upload, or telemetry.
- Stable-release channel by default; version pinning for reproducible usage; updates explicit and understandable.
- After installation, scanning works offline unless the user requests an external integration/update.
- Provide `--version`, `--help`, uninstall, cache deletion, and a documented version/compatibility matrix.
- For local dashboards, bind only to loopback; validate Host/Origin, defend against DNS rebinding and cross-site requests, and avoid unrestricted filesystem APIs.

## Project selection and privacy

- `.` explicitly means the current working directory; display the resolved path before processing when practical.
- Never modify the target project, run its scripts, execute checked-in agent instructions, or require ADP-specific visualization metadata.
- Scanning unknown or incomplete ADP versions must be useful where safe while withholding unsupported conformance claims.
- Report excluded files, incomplete traversal, parser errors, and metric limitations.
- Use per-user application directories for binary/cache/snapshots; optional snapshot history never becomes canonical project evidence.
- No account, mandatory network service, automatic upload, or telemetry.

## Proposed CLI contract

| Usage | Intended behavior |
| --- | --- |
| `adp-observatory .` | Generate offline local HTML and attempt to open it (implemented in development executable; not installed globally) |
| `adp-observatory /path/to/project` | Analyze selected project and generate offline HTML (implemented in development executable) |
| `adp-observatory scan . --json` | Headless JSON report without browser (implemented) |
| `adp-observatory --version` | Display executable and analyzer version (implemented) |
| `adp-observatory --help` | Command help (implemented) |

**Developer use:** run `cargo run --manifest-path /path/to/ADP-Observatory/Cargo.toml -- .` from a project directory to open the offline report, or replace `.` with `scan . --json` for headless JSON. No signed or published installation experience is available yet.

## Delivery sequence and acceptance

1. Finalize the scanner's supported versions, safe file handling, and JSON schema (currently a tested limited subset).
2. Provide a basic usable local dashboard and explicit CLI mode separation (implemented in the development executable).
3. Create a GitHub Actions OS/CPU matrix for experimental artifacts and signed provenance (Windows x64, Linux x64, macOS runner implemented and verified; no official release yet).
4. Publish a signed or otherwise authentically verifiable tagged release and checksums/provenance; exercise install from a clean machine.
5. Add verified platform-specific installer/launcher, cache/update/uninstall support, and clear offline/error behavior.
6. Publish tested copy/paste commands and update the human-facing ADP integration guide **only after working releases exist**.
7. Run acceptance tests with a project whose ADP files are missing, malformed, conflicting, or from a newer version.

This design is **not** an ADP protocol requirement; only the Observatory repository defines launcher behavior.
