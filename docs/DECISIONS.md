# Decisions and Open Questions

## Accepted baseline decisions
- **D-001 Optional companion:** Observatory is not required for ADP conformance and cannot override canonical ADP records.
- **D-002 Local-first:** no mandatory account, hosted service, or telemetry.
- **D-003 Read-only:** scans do not modify target projects by default.
- **D-004 Honest uncertainty:** unknown/partial/not-assessed are not zero or pass.
- **D-005 Traceability:** metrics/findings reference evidence and disclose limitations.
- **D-006 Derived history:** snapshots are optional local data, not project truth; no trend without a baseline.
- **D-007 Verify before execute:** launchers verify official artifacts; no blind shell piping.
- **D-008 Separate ADP changes:** normative amendments are proposed/reviewed in the canonical ADP repository.
- **D-009 ADP-first, zero mandatory overhead:** no new required ADP files, metadata, event logging, or agent reading for Observatory. Protect agent speed, low token use, and continuity.
- **D-010 Version-aware adapters:** format parsing, semantic understanding, and conformance coverage are independent. Unknown/newer ADP versions degrade safely.
- **D-011 Fault isolation:** malformed/missing/conflicting records must not crash the whole scan or silently override other claims.
- **D-012 No protocol amendment now:** test against real projects before proposing any change to canonical ADP 2.7.1.
- **D-013 Optional means optional:** missing WORK.json alone is not a conformance failure.
- **D-014 Conflict provenance:** preserve contradictory claims with source references; no automatic winner by file modification time.
- **D-015 User-invoked visualization:** the eventual default UX is `adp-observatory .` from a project directory; headless scans remain available separately. No repository installation, agent prompt, or mandatory ADP file is added.
- **D-016 Trusted first-run distribution:** prefer install-once, verified native releases; consider zero-install launchers only after authenticated artifact verification is implemented and tested. No blind script piping or unverified auto-updates.
- **D-017 Experimental build provenance:** GitHub/Sigstore-signed CI attestations can verify the provenance of experimental binaries, but do not imply an OS-code-signed, protected, stable release or justify a blind first-run download-and-execute command.
- **D-018 Pinned preview installers:** reviewed local Bash/PowerShell helpers require an explicit successful CI run plus exact 40-character source SHA and validate checksum, source signature and signing workflow before per-user installation. Installed executable runs `adp-observatory .` from a project; no automatic path changes or official installer claim. All three platform installer smoke checks passed (run 39601029440).

## Open decisions
- **Q-001 Core language:** validate Rust against packaging and maintenance needs.
- **Q-002 Dashboard framework:** choose the smallest maintainable UI after a vertical slice.
- **Q-003 Version adapter support matrix:** which historical ADP versions have independently tested semantics?
- **Q-004 Platform matrix:** decide OS/architectures based on CI and release feasibility.
- **Q-005 Metric configuration:** project-specific policy without forcing a new ADP manifest.
- **Q-006 Release signing:** choose a maintainable signing/provenance process.
- **Q-007 Data location/retention:** specify paths, migration, export, and deletion.
- **Q-008 Launcher packaging:** decide per-OS signed installer/package manager routes and whether authenticated one-shot helpers are worth the extra attack surface. See docs/LAUNCHER.md.
- **Q-009 Initial rules:** prioritize correctness, applicability, and low false-positive risk.
- **Q-010 Markdown linking:** determine reliable explicit-reference extraction versus speculative inference.
