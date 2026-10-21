# WORK-EXPLORER-001 — Contract & Evidence Explorer (future feature proposal)

**State: DEFERRED / PROPOSED — NOT IMPLEMENTED.**
**Target area:** M3 human dashboard, building on M1 deterministic safe discovery.
**Origin:** First real Trumpdup/ADP dashboard pilot, 2026-10-21. The report lists approximately 30 contracts with recorded statuses but offers no way to open a file, understand its content, or inspect the supporting evidence. The user asked to brainstorm this feature and preserve the specification for later, **not** to implement it in the current development slice.

## 1. Intent and product outcome

Turn the existing static Observatory report from a long read-only contract list into a **navigable local project knowledge explorer**:

- Click a contract, context record, or supported linked file to open a **human-readable rendered view** of selected source material.
- Parse the file **once during the local scan**, extracting meaningful, traceable sections instead of forcing the browser or a coding agent to reread it on every click.
- Show useful *document-derived* context—purpose, requirements, acceptance criteria, verification statements, related contracts, decisions, risks, next actions—**only when explicitly supported by the source text**.
- Allow the reader to inspect exactly **where** a statement came from, with source path and line range. Keep the source and recorded project status separate from Observatory's own analysis and any independently verified result.
- Operate completely **offline** in the existing self-contained HTML report and leave the analyzed ADP repository/Git state untouched.

**Core invariant:** ADP serves the coding agent efficiently and without added token/file overhead. Observatory is an optional, local read-only *consumer*, not another protocol. Do not require additional ADP fields, Markdown section names, metadata updates, LLM queries, or project commands just to make this explorer useful.

## 2. Screenshot-driven UX observations

The pilot report successfully displays the ADP version, 30 declared contracts, dependency strings, file-reference states, compatibility and findings. It is a good first slice, but the primary column becomes a long scroll of 30 repeated cards, and a contract cannot be opened. The sidebar contains relatively static information even while the reader is inspecting a specific work item. Long labels such as `COMPLETE_WITH_EXCLUSIONS` wrap awkwardly. Recorded statuses could be mistaken for independently assessed completion if source and assessment are not visibly separated.

### Proposed interaction

1. **Overview remains lightweight:** version, scan scope, *declared* contract totals, open limitations, and a concise workstream/contract navigation summary.
2. **Compact navigable list:** searchable/sortable/groupable by declared workstream, state, ID, title and active work. A selected row is visibly highlighted. Clicking a row opens its detail view without navigating away or rescanning.
3. **Contract detail panel/drawer or dedicated in-report view:** heading + recorded state, declared dependencies and workstream, source locator, analysis freshness, availability/limitations. Show an on-screen **document outline** and a few pre-extracted relevant sections.
4. **Detail tabs or sections:** `Overview`, `Extracted sections`, `Rendered source` and `Evidence & links`. Tabs are a UX proposal, not a requirement for v1. Provide a full-source display only when within explicit bounds and when the user chooses it.
5. **Source navigation:** clicking an extracted section scrolls directly to that section within the *embedded offline report*, highlights its original line range, and shows the source path. Dependencies and other explicit, validated references become internal navigation links only if their targets exist in this report.
6. **Other records:** the same viewer should work for supported `docs/STATUS.md`, `docs/CONTINUITY.md`, decision logs and explicit documentation references, so it is a project explorer rather than a WORK.json-only feature. Content availability remains independent of whether WORK.json exists.
7. **Accessible and responsive:** large screens can use a compact left list plus right detail panel. On smaller screens, use a drawer or stacked layout with return/back, visible focus, keyboard navigation, screen-reader labels and a clearly identified active file.
8. **Honest missing states:** `No linked file`, `Link outside project`, `Symlink rejected`, `Unreadable`, `Unsupported format`, `Too large / excerpt only`, and `Parsed with limitations` must be distinguishable. Do not generate a missing WORK.json to compensate.

### Simple information-architecture sketch

```text
Project overview                        Project summary / limitations
┌────────────────────────────────────────────────────────────────────┐
│ Filter contracts   Workstream ▼   Status ▼                          │
│                                                                    │
│ WORK-001  Setup                 recorded: verified   ← selected    │
│ WORK-002  Security              recorded: verified                 │
│ WORK-003  Story graph           recorded: in_progress              │
│ ...                                                                │
├──────────────────────┬─────────────────────────────────────────────┤
│ Contract outline     │ WORK-001 — Setup                            │
│ • Purpose            │ Recorded state: verified (from WORK.json)   │
│ • Requirements       │ Source: docs/work/WORK-001.md [lines 1–90] │
│ • Acceptance         │                                             │
│ • Verification       │ Purpose / accepted requirements            │
│ • Risks / next steps │ [rendered excerpt linked to original lines] │
│                      │                                             │
│                      │ Evidence: claimed vs assessed vs unknown    │
└──────────────────────┴─────────────────────────────────────────────┘
```

The layout should avoid permanently inflating the full report with every contract's complete Markdown content on screen.

## 3. "Pre-analysed" needs precise semantics

**A. Deterministic pre-analysis (recommended v1, offline, no AI dependency)**

The Rust analyzer reads supported referenced source documents *during the scan*, after enforcing path/type/scope/size rules. It constructs a bounded normalized representation:

- file identity: repository-relative path, file type, byte/line limits, parse status, content digest, originating record reference, and scan/parser version;
- hierarchical Markdown heading tree, section titles and ordered line spans;
- paragraphs, emphasis, lists, checklist/task items, blockquotes and fenced code blocks when safe;
- evidence-bearing *candidates* under recognized headings such as purpose/scope, requirements, acceptance criteria, verification/tests, decisions, dependencies, risks, blockers, next steps, and limitations;
- explicitly written contract IDs, file links and referenced evidence paths **only when unambiguously parseable and within the repository**;
- exact short source quotations or excerpts for each extracted item, with provenance and any extraction limits.

Classify a section by deterministic, version-aware heuristics and explicit heading aliases. A heading match means **section classification**, not proof that the content satisfies its heading. Keep unknown/custom headings visible as normal sections rather than dropping them. Do not silently rewrite the text.

**B. Evidence inspection (later capability, not automatically implied by v1)**

A `Verification` section saying "all tests pass" is a *statement in a file*, not independently verified evidence. Separate:
1. **Recorded claim** — what a document or WORK index says, with source path/line.
2. **Linked evidence** — a referenced log, file, test report or explicit artifact whose existence/content may or may not be understood.
3. **Independent assessment** — an actual supported Observatory rule with inputs, rule/version, limitations and result; `NOT_ASSESSED` by default.

Avoid generic "quality", "confidence", "percent complete", or "verified" badges without a defensible definition. Do not execute project tests or scripts to turn a claim into evidence.

**C. Optional AI/LLM summaries (possible later, NOT part of core)**

The term "pre-analyzed" should **not** imply an AI model has read the documents. V1 should be fully deterministic, reproducible and offline. Later, an explicit user-initiated local model/agent enrichment could be separately evaluated, but it must not upload project material, alter ADP files, or be required for basic exploration. Such suggestions must remain clearly labeled generated interpretations with links to the original text.

## 4. Technical design direction (proposed)

**Pipeline:** `safe reference discovery → bounded document reads → Markdown parsing & section extraction → provenance-aware document model → safe offline HTML serialization → client-side selection/navigation`.

- **Source scope:** begin with `contract_file` references already present in optional `WORK.json`; extend to already-discovered `STATUS`/`CONTINUITY` documents and explicitly linked in-root Markdown. Without WORK.json, show available canonical Markdown records and, only if justified, conservatively identify `docs/work/*.md` as unindexed documents with **unknown** status. Never synthesize contract status.
- **Safe resolution:** reuse the existing references module's policy (reject traversal, symlinks, unsafe types and outside-root paths), then revalidate at actual read time. Watch for symlink/reparse-point and time-of-check/time-of-use changes; do not assume that a prior `PRESENT` result authorizes reading arbitrary bytes. Fail closed where containment cannot be established.
- **Bounded work:** cap total documents, per-document bytes, total bytes embedded, parser recursion/heading depth, section excerpt lengths and runtime. Show clear `TRUNCATED`/`NOT_PARSED` states, counts, and the reason. Exact limits need performance fixtures before freezing.
- **Markdown rendering:** use a maintained Rust Markdown parser rather than ad hoc regex for everything. **Disable or escape raw HTML**, unsafe URLs, remote images and arbitrary filesystem links; treat code fences as escaped text, never executable code. Do not load extensions that permit script execution or remote content. No uncontrolled `file://` navigation.
- **Offline report:** precompute a sanitized, bounded source model, embed it in the HTML or safe local in-report structures, and use small JavaScript only to select/filter/navigate. The browser must **never reread the repository** via network, filesystem APIs or a local HTTP service just because a contract is clicked.
- **Report model:** source material is a *scan-time snapshot*, not live content. Include parser version and source digest/scan identity, display `Report generated from snapshot`, and require an explicit **new** scan to refresh. Avoid silently mixing data from different scan instances.
- **Data consistency:** preserve deterministic output where practical. Add versioned optional `documents` / `document_views` fields to Observatory's own JSON output only when implementation starts, with a planned compatibility migration and schema tests. **No normative ADP schema change.** Consider an HTML-only view model first if full source embedding into headless JSON is undesirable for privacy/size.
- **Caching:** v1 can simply precompute the parsed model per explicit scan; no durable index, worker, database or file watcher is necessary. Future caches/snapshots, if justified, belong outside the project and need clear deletion/privacy rules.
- **Security & privacy:** a report containing rendered document excerpts may reveal substantially more private project content than today's filename-only report. Make embedding boundaries visible and provide explicit `summary-only` / `no-source-content` options before broader rollout. Report export/sharing stays an intentional user action; no upload/telemetry. HTML/CSS/JS remain self-contained under strict CSP and source content is inert text.

## 5. Delivery slices (sequenced, not started)

### Slice 1 — Clickable contract detail with bounded extraction (MVP)
- [ ] Click a work contract and see source title, declared metadata, safe path, parse status, key heading outline, selected bounded excerpts and line numbers.
- [ ] Source is parsed only once per explicit CLI scan and embedded safely into the offline report.
- [ ] A missing or unsupported file never blocks independent contract metadata or other documents.
- [ ] No project writes, code execution, new ADP fields or network calls.
- [ ] Test 30+ contract navigation, search, back/focus, long strings and keyboard controls.

### Slice 2 — Rendered Markdown / source navigation
- [ ] Support semantic Markdown display (headings, paragraphs, lists, checkboxes, safe code blocks, tables if feasible), preserving source line provenance.
- [ ] Switch between extracted highlights and bounded rendered source/raw text.
- [ ] Internal links navigate to pre-embedded known documents/contract sections; no arbitrary filesystem access.
- [ ] Explicit statuses for truncated, missing, denied, symlinked and unsupported sources.

### Slice 3 — Traceable relationships and evidence
- [ ] Add a lightweight source-backed graph/list of declared dependencies, explicit references and relevant record links without inventing causal relationships.
- [ ] Separate recorded acceptance criteria, verification claims, linked proof and independently assessed rules.
- [ ] When no evidence exists, show `NOT_ASSESSED` or `NO_EXPLICIT_EVIDENCE`, not failed/passed.

### Slice 4 — Optional deeper analysis (future decision)
- [ ] Consider meaningful non-LLM cross-document summaries and conflict navigation only when backed by tested rules.
- [ ] Consider user-requested agent/model explanation of **selected** passages with explicit privacy, provenance and opt-in guardrails; not part of the basic local report.
- [ ] Consider document-level comparison with *actual recorded snapshots* only after the existing M3 snapshot feature is implemented.

## 6. Acceptance criteria / quality gates

1. **Exact provenance:** every extracted statement links to its true file path and bounded line range; references survive whitespace, duplicate headings and Unicode filenames. No invented source line numbers.
2. **Correct classification:** purpose, requirements, acceptance and evidence candidates are only inferred from a documented tested rule; ambiguous headings degrade to generic section display.
3. **No conflation:** a `WORK.json.status = verified` declaration remains visibly `recorded: verified`; parsing a file is not proof of completion or ADP conformance.
4. **Safely bounded:** hostile Markdown/HTML, `javascript:` or `file:` links, code fences, oversized documents, bad UTF-8, symlinks/traversal, race conditions and nested/ambiguous paths are isolated. No remote assets, DOM script execution or browser filesystem fetch.
5. **No repository mutation:** byte-for-byte and Git-status before/after fixtures on supported real projects; no cache or exports in the target repository.
6. **Performance:** representative projects (30, 300 and large bounded sets of contracts) have measurable scan and HTML size/memory budgets and explicit incomplete-result reporting.
7. **Offline accessibility:** tested keyboard selection, focus, scroll, screen-reader labels, responsive layout, no network requests and reasonable accessibility for large document lists.
8. **Backward compatibility:** existing headless JSON clients and older/partial ADP projects continue working. Versioned Observatory schema changes are explicit; new documents are *optional* output fields, not ADP requirements.
9. **Snapshot truthfulness:** scan-time content is clearly labeled; no inference that a file is still current when a previously generated HTML report is reopened.
10. **Review against actual source:** test the real Windows/Trumpdup-style project (with permission) plus synthetic hostile/incomplete fixtures before marking implementation verified.

## 7. Explicit non-goals

- **No edit-in-place Markdown editor**, "fix my contract", automatically generated WORK.json, Git changes, execution of repository build/tests, or project-file migration.
- No mandatory ADP agent instructions, new protocol chapters, new agent work contracts for consumer projects or token-expensive per-document agent prompts.
- No default hosted/web service, internet access, background polling, telemetry, external LLM API or repository upload.
- No promise of complete Markdown fidelity, all content fully indexed, full verification, complete conformance or automatically correct summaries on unsupported formats.
- No local browser reading arbitrary files directly; only self-contained preprocessed output under existing offline security policy.

## 8. Open design choices (resolve before coding)

- Detail presentation: right-hand panel vs modal/drawer vs in-report page; choose from real 30-contract usability testing.
- Extraction set: first-pass headings/acceptance/test/decision sections vs full Markdown source; choose conservative v1.
- How to embed readable source without excessive HTML size or accidental sensitive-content exposure: bounded snippets by default and deliberate full-source option.
- Exact resource budgets and file-type matrix, including support for `.txt`, `.json` and image/PDF attachments (defer images/PDF to separately scoped decisions).
- Which link syntaxes and source relationships can be resolved deterministically with reliable line provenance?
- Should parsed document data be part of the versioned machine JSON report, or an isolated HTML report view model with only summaries in JSON?
- What should happen when an AI agent separately offers a source summary? Opt-in and trust boundaries must be explicit.
- How to navigate work-contract documents if WORK.json is absent without inventing a contract index.

## 9. Dependencies, scheduling and state

- **Existing:** initial offline HTML renderer `WORK-DASHBOARD-001` is verified; reference presence and bounded scanning live in `WORK-SCANNER-001` but *referenced Markdown contents are not yet parsed*.
- **Must remain separate:** Windows local browser pilot and trusted distribution (issues #2 and #1) are release/user-experience gates; this exploratory future feature must not conceal those open blockers.
- **When ready:** start after scanner safety/calibration work has a stable API for bounded, securely opened referenced files. Choose an MVP slice, create fixture-driven implementation changes, and keep report-schema and UX acceptance explicit.
- **Today:** planning/specification only; **DEFERRED** in Observatory's own optional WORK index. Not started, not verified and not on the active contract path.

**Implementation direction:** deterministic, source-traceable **precomputed document exploration** first; extra analysis or AI explanation only when justified and independently approved later.
