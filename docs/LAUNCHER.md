# Launching and distributing ADP-Observatory

**Status:** Design baseline; **not implemented or released**. Observatory currently requires a Rust build/checkout and provides CLI scanning, not a dashboard or an installer.

## User experience

The desired experience is to open a terminal **inside any ADP project** and run:

```text
adp-observatory .
```

When the graphical application is implemented, this should select the current working directory as the project root, launch a loopback-only dashboard, and open it in the browser. The user should not copy any Observatory files into their project.

For scripted, noninteractive, and agent usage, the desired command is:

```text
adp-observatory scan . --json
```

This future CLI shape is **not implemented**. The current Rust prototype accepts `adp-observatory <project-directory> [--json]` and has no graphical dashboard. Preserve this positional form as a backwards-compatible alias when introducing subcommands, or document the breaking change before a stable release.

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
| `adp-observatory .` | Analyze current project and open local dashboard (future) |
| `adp-observatory /path/to/project` | Analyze an explicitly selected project (future dashboard mode) |
| `adp-observatory scan . --json` | Headless, versioned JSON report without browser (future) |
| `adp-observatory --version` | Display executable and analyzer version (future) |
| `adp-observatory --help` | Command and compatibility information (future) |

**Existing prototype:** run `cargo run --manifest-path /path/to/ADP-Observatory/Cargo.toml -- . --json` from a project directory with Rust installed and Observatory checked out separately.

## Delivery sequence and acceptance

1. Finalize the scanner's supported versions, safe file handling, and JSON schema.
2. Provide a basic usable local dashboard and explicit CLI mode separation.
3. Create a GitHub Actions release matrix for selected supported OS/CPU architectures.
4. Publish a signed or otherwise authentically verifiable tagged release and checksums/provenance; exercise install from a clean machine.
5. Add verified platform-specific installer/launcher, cache/update/uninstall support, and clear offline/error behavior.
6. Publish tested copy/paste commands and update the human-facing ADP integration guide **only after working releases exist**.
7. Run acceptance tests with a project whose ADP files are missing, malformed, conflicting, or from a newer version.

This design is **not** an ADP protocol requirement; only the Observatory repository defines launcher behavior.
