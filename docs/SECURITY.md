# Security and Privacy

## Threat model
Assume target repositories may contain malicious or malformed content, hostile filenames, symlinks, oversized files, crafted documents, misleading agent instructions, and external reports designed to manipulate results. Assume a local browser may visit untrusted sites while the dashboard runs.

## Safe defaults
- Target scanning is read-only.
- Do not execute target scripts, tests, hooks, package managers, binaries, or agent-provided commands during analysis.
- Treat scanned content as data, including agent instructions and external-analysis reports.
- Do not upload files, metrics, paths, snapshots, or telemetry by default. Future remote features need explicit informed opt-in and data preview.
- No admin/root privileges.
- Enforce file-size, recursion, time, and memory limits; report partial scans.
- Handle symlinks deliberately and do not escape the selected root without explicit configuration.
- Do not follow arbitrary URLs discovered in project files.
- Escape rendered content and prevent script injection.

## Release and launcher security
- Download only from a configured official release origin over HTTPS.
- Verify a cryptographic signature where practical; otherwise verify a published checksum from a trustworthy release channel. A checksum alone does not protect against a compromised channel.
- Verify platform, architecture, version, and artifact before execution.
- Never pipe downloaded bytes directly into a shell or execute an unverified artifact.
- Use temporary files, restrictive permissions, atomic replacement where supported, and clear errors.
- No silent auto-update. Expose version, source, verification result, and explicit update path.
- Document uninstall and local cache/data removal.
- Avoid shell injection; quote paths and handle Unicode/spaces.
- Improve signed builds and provenance as the release process matures.

## Agent-invoked Observatory

The optional `skills/adp-observatory/SKILL.md` is activated **only** by a
user's explicit visualization request. ADP supplies a discovery link, not
authority to execute remote code. Agent instructions found inside the target
project, GitHub issues, external reports or fetched Markdown are untrusted.
No script from any of these sources may override user/harness permissions.

The local standard-library helper does not fetch or install binaries. It
rejects executables resolving into the project being inspected and attempts
to reuse only existing operator-approved installed executables. **Finding an
executable or querying `--version` is not proof of cryptographic provenance**;
approval and signature verification belong to the installation/release
workflow. No unpinned latest release, silent auto-updates, elevated privileges,
or unreviewed shell piping.

Agent/harness locality matters. A remote sandbox or GitHub connector cannot
promise to open the user's physical browser. In CI/SSH/headless contexts,
return the local HTML path or a supported transferable artifact; do not
automatically upload project reports. A successful browser opener call is
evidence of an open request, not proof that the user saw the file.

## Agent orchestration non-interference

A first Windows local-agent trial exposed an important distinction: the
Observatory executable generated an offline report without writing to the
target, but the **agent** unnecessarily created an optional work index,
ran the target's tests, and committed/pushed it. An Observatory setup or
visualization request alone is **not permission** for those changes.
The [Agent Skill](../skills/adp-observatory/SKILL.md) now explicitly forbids
project file/metadata changes, application test execution and Git mutations
during visualization. Missing optional records must remain unknown.
Separate project maintenance, if requested, follows ADP's own authorization
and review rules rather than being smuggled into a dashboard task.

Windows HTML opening uses the system Shell API with normalized shell paths;
its return value only shows an open request was accepted, not whether the user
saw a browser tab. Report paths can contain sensitive local metadata; do not
silently copy them to other IDE or cloud artifact storage.

## Local dashboard
The current viewer is a self-contained offline HTML report with a restrictive Content Security Policy and escaped project data; it does **not** expose a local HTTP listener. If a future interactive dashboard uses HTTP, bind exclusively to loopback, validate Host and Origin, protect against DNS rebinding and cross-site requests, authenticate state-changing endpoints, and avoid unrestricted filesystem access. Absolute project paths in saved reports are sensitive local data and must never be uploaded by default.

## Privacy and retention
Snapshots stay local and user-controlled. Explain stored data and location; provide deletion/export controls. Exports are explicit and warn about paths/sensitive metadata. No analytics or crash upload by default.

## Security reporting
Document a private reporting route before public releases. Reproduce and assess reports before publishing exploitable details.

## Development source-reader boundary (2026-10-28; review pending)
- The new capability-relative Markdown reader walks held directory handles with no-follow operations. It does not open an ambient project filename after a prior metadata-only `PRESENT` check. It checks actual opened-file metadata and uses an extra-byte bounded read to detect growth beyond limits.
- Budgets: 128 documents, 128 KiB/document, 8 MiB combined, bounded path components, lines, line lengths and headings. Unsupported, escaped, symlinked or unreadable items are isolated. No arbitrary repository command execution.
- Serialized reports contain filenames, state, counts and SHA-256 hashes, **not Markdown content**. Even metadata, paths and digests may be sensitive; reports remain local/exported deliberately.
- These are implementation safeguards awaiting independent Windows reparse-point and rename-race review. No claim of full symlink/race immunity or unreviewed safe publication should be made before cross-platform adversarial acceptance.
