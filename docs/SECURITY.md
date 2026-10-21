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

## Local dashboard
The current viewer is a self-contained offline HTML report with a restrictive Content Security Policy and escaped project data; it does **not** expose a local HTTP listener. If a future interactive dashboard uses HTTP, bind exclusively to loopback, validate Host and Origin, protect against DNS rebinding and cross-site requests, authenticate state-changing endpoints, and avoid unrestricted filesystem access. Absolute project paths in saved reports are sensitive local data and must never be uploaded by default.

## Privacy and retention
Snapshots stay local and user-controlled. Explain stored data and location; provide deletion/export controls. Exports are explicit and warn about paths/sensitive metadata. No analytics or crash upload by default.

## Security reporting
Document a private reporting route before public releases. Reproduce and assess reports before publishing exploitable details.
