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

## Local dashboard
Bind exclusively to loopback; validate Host and Origin; protect endpoints against DNS rebinding and cross-site requests; use a random per-run token for state-changing endpoints; apply restrictive CORS, Content Security Policy, and output escaping. Do not expose unrestricted filesystem endpoints. Make address/port and shutdown visible. Avoid absolute-path leakage in reports unless user opts in.

## Privacy and retention
Snapshots stay local and user-controlled. Explain stored data and location; provide deletion/export controls. Exports are explicit and warn about paths/sensitive metadata. No analytics or crash upload by default.

## Security reporting
Document a private reporting route before public releases. Reproduce and assess reports before publishing exploitable details.
