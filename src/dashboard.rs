use serde_json::Value;
use std::collections::BTreeMap;

const TEMPLATE: &str = include_str!("../web/report.html");
const STYLE: &str = include_str!("../web/report.css");
const SCRIPT: &str = include_str!("../web/report.js");

fn escape_html(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for character in raw.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(character),
        }
    }
    out
}

fn at<'a>(value: &'a Value, pointer: &str, fallback: &'a str) -> &'a str {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .unwrap_or(fallback)
}

fn count(value: &Value) -> String {
    match value.as_u64() {
        Some(number) => number.to_string(),
        None => "—".into(),
    }
}

fn safe_tag(status: &str) -> &'static str {
    match status {
        "PRESENT" | "READABLE" | "STRUCTURALLY_VALID" | "COMPLETE" => "ok",
        "INVALID" | "INVALID_PATH" | "OUTSIDE_ROOT" | "CONFLICT" | "error" => "error",
        "MISSING" | "UNREADABLE" | "UNSUPPORTED" | "UNSUPPORTED_SCHEMA" | "PARTIAL" | "SYMLINK"
        | "NOT_FILE" | "NOT_DIRECTORY" | "warning" | "blocked" => "warn",
        _ => "neutral",
    }
}

fn tag(status: &str) -> String {
    format!(
        "<span class=\"tag {}\">{}</span>",
        safe_tag(status),
        escape_html(status)
    )
}

fn metric(label: &str, value: &str, note: &str) -> String {
    format!(
        "<article class=\"metric\"><span class=\"metric-label\">{}</span><span class=\"metric-value\">{}</span><p class=\"metric-foot\">{}</p></article>",
        escape_html(label),
        escape_html(value),
        escape_html(note)
    )
}

fn keyval(label: &str, value: &str) -> String {
    format!(
        "<div class=\"keyval\"><span class=\"keyval-label\">{}</span><span class=\"keyval-value\">{}</span></div>",
        escape_html(label),
        escape_html(value)
    )
}

fn row(title: &str, detail: &str, status: &str, filterable: bool) -> String {
    let class = if filterable {
        "list-row filterable"
    } else {
        "list-row"
    };
    format!(
        "<article class=\"{class}\"><div class=\"row-header\"><span class=\"row-title\">{}</span>{}</div><p class=\"row-desc\">{}</p></article>",
        escape_html(title),
        tag(status),
        escape_html(detail)
    )
}

/// Dashboard-only, explicitly *declared* progress. This does not assess
/// evidence, source quality, independently verified work, or overall project
/// readiness. Any invalid ledger suppresses all derived percentages.
struct DeclaredProgress {
    counts: [usize; 5],
    total: usize,
    streams: BTreeMap<String, [usize; 5]>,
    applicable: usize,
    percent: Option<u64>,
}
impl DeclaredProgress {
    fn from_metrics(metrics: &crate::metrics::Metrics) -> Option<Self> {
        if !matches!(metrics.delivery.state.as_str(), "KNOWN" | "NOT_APPLICABLE") {
            return None;
        }
        let counts = &metrics.delivery.statuses;
        let values = [
            counts.verified,
            counts.underway,
            counts.blocked,
            counts.planned,
            counts.cancelled,
        ];
        let streams = metrics
            .delivery
            .workstreams
            .iter()
            .map(|s| {
                let c = &s.statuses;
                (
                    s.name.clone(),
                    [c.verified, c.underway, c.blocked, c.planned, c.cancelled],
                )
            })
            .collect();
        Some(Self {
            counts: values,
            total: counts.total(),
            streams,
            applicable: metrics.delivery.applicable,
            percent: metrics.delivery.percent,
        })
    }
}

fn actual_metrics(report: &Value) -> crate::metrics::Metrics {
    // CLI supplies the serialized metrics; legacy render-only test fixtures
    // may omit them, in which case the *same* pure metric rule runs locally.
    report
        .get("metrics")
        .and_then(|data| serde_json::from_value(data.clone()).ok())
        .unwrap_or_else(|| crate::metrics::analyze(&report["work_index"]))
}

fn show_live_metrics(m: &crate::metrics::Metrics, progress: Option<&DeclaredProgress>) -> String {
    let mut results = Vec::new();
    match progress {
        Some(work) => {
            let value = work
                .percent
                .map(|amount| format!("{amount}%"))
                .unwrap_or_else(|| "UNKNOWN".into());
            results.push(metric(
                "Declared completion",
                &value,
                &format!(
                    "{}/{} declared verified / non-cancelled contracts; {} cancelled excluded. Declared records; statuses are not independently verified",
                    work.counts[0], work.applicable, work.counts[4]
                ),
            ));
            results.push(metric(
                "Active contracts",
                &work.counts[1].to_string(),
                "Recorded in_progress, implemented or verifying",
            ));
            results.push(metric(
                "Recorded blockers",
                &work.counts[2].to_string(),
                "Explicit blocked status only; other findings listed separately",
            ));
        }
        None => {
            results.push(metric("Declared completion", "UNKNOWN",
                "Invalid, missing or unsupported work ledger. Declared records; statuses are not independently verified"));
            results.push(metric(
                "Active contracts",
                "UNKNOWN",
                "No trustworthy contract-state breakdown",
            ));
            results.push(metric(
                "Recorded blockers",
                "UNKNOWN",
                "No trustworthy contract-state breakdown",
            ));
        }
    }
    let references = &m.references;
    if let Some(pct) = references.percent {
        results.push(metric(
            "Reference presence", &format!("{pct}%"),
            &format!("{}/{} contract-file paths present; {} missing · {} other states. This is not evidence verification",
                references.present, references.expected,
                references.missing, references.denied_or_unsupported),
        ));
    } else {
        results.push(metric(
            "Reference presence",
            &references.state,
            &format!(
                "{}/{} observed paths; {} examined; coverage is not independently verified",
                references.present, references.expected, references.inspected
            ),
        ));
    }
    results.join("")
}

fn show_dependency_inspection(m: &crate::metrics::Metrics) -> String {
    if m.dependencies.state != "KNOWN" {
        return "<div class=\"empty\">Dependency coverage UNKNOWN or PARTIAL: check the work index findings.</div>".into();
    }
    let mut html = format!(
        "<div class=\"dependency-summary\"><strong>{}</strong> explicitly declared dependency links · <strong>{}</strong> target records not marked verified</div>",
        m.dependencies.declared_edges, m.dependencies.pending_by_recorded_status
    );
    if m.dependencies.details.is_empty() {
        html.push_str("<p class=\"section-help\">No pending-by-recorded-status dependencies are declared. This does not verify completion or prove absence of blockers.</p>");
    }
    for item in m.dependencies.details.iter().take(12) {
        html.push_str(&format!(
            "<div class=\"dependency-row\"><span>{}</span><span aria-hidden=\"true\">→</span><span>{}</span><small>recorded: {}</small></div>",
            escape_html(&item.contract_id),
            escape_html(&item.depends_on),
            escape_html(&item.target_recorded_status)
        ));
    }
    let not_displayed =
        m.dependencies.details.len().saturating_sub(12) + m.dependencies.omitted_details;
    if not_displayed > 0 {
        html.push_str(&format!("<p class=\"section-help\">{not_displayed} additional recorded dependency relationships not shown in this summary.</p>"));
    }
    html
}

fn show_state_chart(progress: Option<&DeclaredProgress>) -> String {
    let Some(work) = progress else {
        return "<div class=\"empty\">UNKNOWN — a structurally valid work index is required before drawing contract-state charts.</div>".into();
    };
    if work.total == 0 {
        return "<div class=\"empty\">No declared contracts available for a status distribution.</div>".into();
    }
    let names = [
        "Declared verified",
        "Underway",
        "Blocked",
        "Planned / deferred",
        "Cancelled",
    ];
    let colors = ["#476af3", "#16ad9d", "#d66f79", "#e6ac51", "#a8b5c7"];
    let mut legend = String::new();
    let mut bars = String::new();
    for (index, ((name, color), value)) in names
        .iter()
        .zip(colors.iter())
        .zip(work.counts.iter())
        .enumerate()
    {
        let width = (*value * 100) / work.total;
        bars.push_str(&format!(
            "<span class=\"state-segment state-{index}\" style=\"width:{width}%;background:{color}\" title=\"{}: {}\"></span>",
            name, value
        ));
        legend.push_str(&format!(
            "<div class=\"state-legend-row\"><span class=\"legend-dot\" style=\"background:{color}\"></span><span>{}</span><strong>{value}</strong></div>",
            escape_html(name)
        ));
    }
    format!(
        "<div class=\"state-chart\" role=\"img\" aria-label=\"Declared work-state distribution, {} contracts\"><div class=\"state-number\">{} <small>recorded contracts</small></div><div class=\"state-bar\">{bars}</div><div class=\"state-legend\">{legend}</div></div>",
        work.total, work.total
    )
}

fn show_workstreams(progress: Option<&DeclaredProgress>) -> String {
    let Some(work) = progress else {
        return "<div class=\"empty\">Workstream progress is UNKNOWN without a valid contract index.</div>".into();
    };
    if work.streams.is_empty() {
        return "<div class=\"empty\">No declared workstreams to compare.</div>".into();
    }
    let mut rows = String::new();
    for (stream, counts) in &work.streams {
        let total: usize = counts.iter().sum();
        let applicable = total - counts[4];
        let (value, pct) = match (counts[0] * 100 + applicable / 2).checked_div(applicable) {
            Some(pct) => (format!("{} / {} · {pct}%", counts[0], applicable), pct),
            None => ("NOT_APPLICABLE".into(), 0),
        };
        rows.push_str(&format!(
            "<div class=\"stream-row\"><div class=\"stream-head\"><span class=\"stream-name\">{}</span><span class=\"stream-value\">{}</span></div><div class=\"stream-track\"><span style=\"width:{pct}%\"></span></div><p class=\"stream-note\">{} underway · {} blocked · {} planned/deferred · {} cancelled</p></div>",
            escape_html(stream),
            escape_html(&value),
            counts[1],
            counts[2],
            counts[3],
            counts[4]
        ));
    }
    rows
}

fn show_contracts(report: &Value) -> String {
    let mut html = String::new();
    let references = report
        .pointer("/work_index/contract_files")
        .and_then(Value::as_array);
    if let Some(items) = report
        .pointer("/work_index/contracts")
        .and_then(Value::as_array)
    {
        for item in items {
            let id = at(item, "/id", "Unnamed contract");
            let title = at(item, "/title", "Title unavailable");
            let status = at(item, "/status", "UNKNOWN");
            let workstream = at(item, "/workstream", "No declared workstream");
            let dependencies = item
                .get("depends_on")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "None declared".into());
            let mut description =
                format!("Workstream: {workstream} · Dependencies: {dependencies}");
            if let Some(path) = item.get("contract_file").and_then(Value::as_str) {
                let state = references
                    .and_then(|links| {
                        links.iter().find(|reference| {
                            at(reference, "/contract_id", "") == id
                                && at(reference, "/path", "") == path
                        })
                    })
                    .map(|reference| at(reference, "/status", "UNKNOWN"))
                    .unwrap_or("NOT_ASSESSED");
                description.push_str(&format!(" · File: {path} ({state})"));
            }
            // WORK.json statuses are claims supplied by the project, not
            // independent verification performed by Observatory.
            html.push_str(&row(
                &format!("{id} · {title}"),
                &description,
                &format!("recorded: {status}"),
                true,
            ));
        }
    }
    if html.is_empty() {
        "<div class=\"empty\">No structured work contracts are available. This may be normal when WORK.json is absent or unsupported.</div>".into()
    } else {
        html
    }
}

fn append_findings(report: &Value) -> (String, usize) {
    let mut lines = String::new();
    let mut total = 0;
    if let Some(findings) = report
        .pointer("/work_index/findings")
        .and_then(Value::as_array)
    {
        for finding in findings {
            let code = at(finding, "/code", "Finding");
            let description = at(finding, "/message", "No description");
            let source = at(finding, "/source", "");
            let severity = at(finding, "/severity", "info");
            lines.push_str(&row(
                code,
                &format!("{source} · {description}"),
                severity,
                false,
            ));
            total += 1;
        }
    }
    if let Some(refs) = report
        .pointer("/work_index/contract_files")
        .and_then(Value::as_array)
    {
        for reference in refs {
            let status = at(reference, "/status", "UNKNOWN");
            if status != "PRESENT" {
                let id = at(reference, "/contract_id", "Unnamed contract");
                let path = at(reference, "/path", "");
                let source = at(reference, "/source", "");
                let detail = reference
                    .get("detail")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                lines.push_str(&row(
                    &format!("{id}: contract reference {status}"),
                    &format!("{path} · {source} · {detail}"),
                    status,
                    false,
                ));
                total += 1;
            }
        }
    }
    if let Some(conflicts) = report
        .pointer("/context/conflicts")
        .and_then(Value::as_array)
    {
        for conflict in conflicts {
            let field = at(conflict, "/field", "Project record");
            let descriptions = conflict
                .get("claims")
                .and_then(Value::as_array)
                .map(|claims| {
                    claims
                        .iter()
                        .map(|claim| {
                            let path = at(claim, "/path", "Unknown path");
                            let value = at(claim, "/value", "Unknown value");
                            format!("{path}: {value}")
                        })
                        .collect::<Vec<_>>()
                        .join(" · ")
                })
                .unwrap_or_default();
            lines.push_str(&row(
                &format!("Conflicting {field} claims"),
                &descriptions,
                "CONFLICT",
                false,
            ));
            total += 1;
        }
    }
    if let Some(limits) = report
        .pointer("/compatibility/protocol/limitations")
        .and_then(Value::as_array)
    {
        for limitation in limits {
            if let Some(detail) = limitation.as_str() {
                lines.push_str(&row(
                    "Protocol version limitation",
                    detail,
                    "UNKNOWN",
                    false,
                ));
                total += 1;
            }
        }
    }
    if at(report, "/scan_scope/status", "PARTIAL") == "PARTIAL" {
        lines.push_str(&row(
            "Some files were not examined",
            "Traversal reached a limit or encountered a read error. Other results are still useful but incomplete.",
            "PARTIAL",
            false,
        ));
        total += 1;
    }
    if report
        .pointer("/work_index/reference_inspection_limited")
        .and_then(Value::as_bool)
        == Some(true)
    {
        lines.push_str(&row(
            "Contract-reference budget reached",
            "Additional contract references were not checked.",
            "PARTIAL",
            false,
        ));
        total += 1;
    }
    if let Some(limits) = report.get("limitations").and_then(Value::as_array) {
        for limitation in limits {
            if let Some(detail) = limitation.as_str() {
                lines.push_str(&row("Analysis limitation", detail, "NOT_ASSESSED", false));
                total += 1;
            }
        }
    }
    if lines.is_empty() {
        lines.push_str("<div class=\"empty\">No findings were reported by these supported checks. This is not proof of project correctness.</div>");
    }
    (lines, total)
}

fn show_records(report: &Value) -> String {
    let mut html = String::new();
    if let Some(records) = report.pointer("/context/records").and_then(Value::as_array) {
        for record in records {
            html.push_str(&row(
                at(record, "/path", "Unnamed record"),
                record
                    .get("limitation")
                    .and_then(Value::as_str)
                    .unwrap_or("Available for limited analysis"),
                at(record, "/state", "UNKNOWN"),
                false,
            ));
        }
    }
    if html.is_empty() {
        "<div class=\"empty\">No supported context records were discovered.</div>".into()
    } else {
        html
    }
}

fn show_compatibility(report: &Value) -> String {
    let pairs = [
        (
            "Detected ADP",
            at(report, "/compatibility/protocol/value", "Unknown"),
        ),
        (
            "Version evidence",
            at(report, "/compatibility/protocol/status", "UNKNOWN"),
        ),
        (
            "WORK.json format",
            at(report, "/compatibility/format", "UNKNOWN"),
        ),
        (
            "WORK schema interpretation",
            at(report, "/compatibility/semantics", "UNKNOWN"),
        ),
        (
            "Semantic scope",
            at(report, "/compatibility/semantic_scope", "UNKNOWN"),
        ),
        (
            "Conformance",
            at(report, "/compatibility/conformance", "NOT_ASSESSED"),
        ),
    ];
    pairs
        .into_iter()
        .map(|(key, value)| keyval(key, value))
        .collect::<String>()
}

fn show_scope(report: &Value) -> String {
    let scope = &report["scan_scope"];
    let pairs = [
        ("Files inspected", count(&report["files_examined"])),
        ("Entries skipped", count(&report["files_skipped"])),
        ("Ignored directories", count(&scope["ignored_entries"])),
        ("Symlinks excluded", count(&scope["symlinks_excluded"])),
        ("Oversized files", count(&scope["oversized_files_excluded"])),
        ("Metadata errors", count(&scope["metadata_errors"])),
        ("Unreadable directories", count(&scope["directory_errors"])),
        ("Depth limits", count(&scope["depth_limit_hits"])),
        ("Entry limits", count(&scope["entry_limit_hits"])),
    ];
    pairs
        .into_iter()
        .map(|(key, value)| keyval(key, &value))
        .collect::<String>()
}

/// A completely self-contained HTML report. Untrusted project data is escaped
/// as text; no project value is ever interpolated into CSS or executable JS.
pub fn render(report: &Value) -> String {
    let project = at(report, "/target", "Unknown project directory");
    let version = at(report, "/compatibility/protocol/value", "Unknown");
    let work = at(report, "/work_index/status", "UNKNOWN");
    let scope = at(report, "/scan_scope/status", "PARTIAL");
    let semantics = at(report, "/compatibility/semantics", "UNKNOWN");
    let metrics = actual_metrics(report);
    let progress = DeclaredProgress::from_metrics(&metrics);
    let summary = show_live_metrics(&metrics, progress.as_ref());
    let (findings, finding_count) = append_findings(report);
    TEMPLATE
        .replace("/*__STYLES__*/", STYLE)
        .replace("/*__SCRIPT__*/", SCRIPT)
        .replace("<!--__PROJECT_PATH__-->", &escape_html(project))
        .replace("<!--__METRICS__-->", &summary)
        .replace("<!--__STATES__-->", &show_state_chart(progress.as_ref()))
        .replace(
            "<!--__DEPENDENCIES__-->",
            &show_dependency_inspection(&metrics),
        )
        .replace(
            "<!--__WORKSTREAMS__-->",
            &show_workstreams(progress.as_ref()),
        )
        .replace("<!--__ADP_VERSION__-->", &escape_html(version))
        .replace("<!--__SCAN_STATUS__-->", &escape_html(scope))
        .replace("<!--__SEMANTIC_STATUS__-->", &escape_html(semantics))
        .replace("<!--__WORK_STATUS__-->", &escape_html(work))
        .replace("<!--__CONTRACTS__-->", &show_contracts(report))
        .replace("<!--__FINDING_COUNT__-->", &finding_count.to_string())
        .replace("<!--__FINDINGS__-->", &findings)
        .replace("<!--__COMPATIBILITY__-->", &show_compatibility(report))
        .replace("<!--__RECORDS__-->", &show_records(report))
        .replace("<!--__SCAN_SCOPE__-->", &show_scope(report))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn html_escapes_malicious_project_data_in_all_report_regions() {
        let report = json!({
            "target": "<script>alert('test')</script>",
            "work_index": {
                "status": "INVALID",
                "contract_count": 1,
                "contract_files": [{
                    "contract_id": "<img src=x onerror=alert(1)>",
                    "path": "<iframe src='file://x'></iframe>",
                    "status": "MISSING",
                    "source": "<svg onload=alert(1)>",
                    "detail": "a&b"
                }],
                "findings": [{
                    "code": "<script>alert(2)</script>",
                    "message": "bad <b>source</b>",
                    "source": "source"
                }]
            },
            "compatibility": {
                "protocol": {"status": "UNKNOWN", "value": null, "limitations": []},
                "semantics": "UNKNOWN",
                "format": "INVALID",
                "conformance": "NOT_ASSESSED",
                "semantic_scope": "WORK_JSON_SCHEMA_V1_ONLY"
            },
            "context": {"records": [], "conflicts": []},
            "scan_scope": {"status": "COMPLETE"},
            "files_examined": 0,
            "files_skipped": 0,
            "limitations": []
        });
        let page = render(&report);
        assert!(!page.contains("<script>alert('test')</script>"));
        assert!(!page.contains("<img src=x onerror=alert(1)>"));
        assert!(!page.contains("<iframe src='file://x'></iframe>"));
        assert!(!page.contains("<svg onload=alert(1)>"));
        assert!(page.contains("&lt;script&gt;alert(&#39;test&#39;)&lt;/script&gt;"));
        assert!(page.contains("a&amp;b"));
        assert!(page.contains("NOT_ASSESSED"));
    }

    #[test]
    fn recorded_verified_contract_does_not_claim_independent_verification() {
        let report = json!({
            "work_index": {
                "status": "STRUCTURALLY_VALID", "contract_count": 1,
                "contracts": [{
                    "id": "W-001", "title": "Example",
                    "status": "verified", "workstream": "delivery",
                    "depends_on": []
                }],
                "contract_files": [], "findings": []
            },
            "compatibility": {"protocol": {"limitations": []}, "conformance": "NOT_ASSESSED"},
            "context": {"records": [], "conflicts": []},
            "scan_scope": {"status": "COMPLETE"},
            "limitations": []
        });
        let page = render(&report);
        assert!(page.contains("recorded: verified"));
        assert!(page.contains("Declared records; statuses are not independently verified"));
        assert!(!page.contains(r#"<span class="tag ok">verified</span>"#));
        assert!(page.contains("Conformance is not assessed"));
    }

    #[test]
    fn declared_completion_uses_valid_recorded_statuses_only() {
        let report = json!({
            "work_index": {
                "status": "STRUCTURALLY_VALID",
                "contract_count": 5,
                "contracts": [
                    {"id":"WORK-A","status":"verified","workstream":"core"},
                    {"id":"WORK-B","status":"verified","workstream":"core"},
                    {"id":"WORK-C","status":"in_progress","workstream":"core"},
                    {"id":"WORK-D","status":"blocked","workstream":"ux"},
                    {"id":"WORK-E","status":"cancelled","workstream":"ux"}
                ],
                "contract_files": [],
                "findings": []
            },
            "compatibility": {"protocol": {"limitations": []}, "conformance": "NOT_ASSESSED"},
            "context": {"records": [], "conflicts": []},
            "scan_scope": {"status": "COMPLETE"},
            "limitations": []
        });
        let progress =
            DeclaredProgress::from_metrics(&actual_metrics(&report)).expect("valid ledger");
        assert_eq!(progress.total, 5);
        assert_eq!(progress.applicable, 4);
        assert_eq!(progress.percent, Some(50));
        assert_eq!(progress.counts, [2, 1, 1, 0, 1]);
        let html = render(&report);
        assert!(html.contains("2/4 declared verified"));
        assert!(html.contains("50%"));
        assert!(html.contains("Cancelled"));
        assert!(html.contains("core"));
        assert!(html.contains("No compatible saved snapshots"));
        assert!(html.contains("NOT_ASSESSED"));
    }

    #[test]
    fn invalid_ledger_never_shows_pseudoprecise_completion_or_charts() {
        let report = json!({
            "work_index": {
                "status":"INVALID",
                "contract_count":2,
                "contracts":[
                    {"id":"WORK-A","status":"verified"},
                    {"id":"WORK-B","status":"in_progress"}
                ],
                "contract_files":[],
                "findings":[]
            },
            "compatibility":{"protocol":{"limitations":[]},"conformance":"NOT_ASSESSED"},
            "context":{"records":[],"conflicts":[]},
            "scan_scope":{"status":"COMPLETE"},
            "limitations":[]
        });
        assert!(DeclaredProgress::from_metrics(&actual_metrics(&report)).is_none());
        let html = render(&report);
        assert!(html.contains("UNKNOWN"));
        assert!(html.contains("a structurally valid work index is required"));
        assert!(html.contains("<span class=\"metric-value\">UNKNOWN</span>"));
        assert!(!html.contains("<span class=\"metric-value\">50%</span>"));
    }

    #[test]
    fn hostile_workstream_name_is_escaped_in_visual_summary() {
        let report = json!({
            "work_index": {
                "status":"STRUCTURALLY_VALID",
                "contract_count":1,
                "contracts":[{
                    "id":"WORK-A", "title":"Test", "status":"verified",
                    "workstream":"<img src=x onerror=alert(1)>"
                }],
                "contract_files":[], "findings":[]
            },
            "compatibility":{"protocol":{"limitations":[]},"conformance":"NOT_ASSESSED"},
            "context":{"records":[],"conflicts":[]},
            "scan_scope":{"status":"COMPLETE"},
            "limitations":[]
        });
        let html = render(&report);
        assert!(html.contains("&lt;img src=x onerror=alert(1)&gt;"));
        assert!(!html.contains("<img src=x onerror=alert(1)>"));
        assert!(!html.contains("https://"));
    }

    #[test]
    fn no_applicable_contracts_does_not_divide_by_zero() {
        let report = json!({
            "work_index": {
                "status":"STRUCTURALLY_VALID",
                "contract_count":1,
                "contracts":[{"id":"WORK-CANCELLED","status":"cancelled","workstream":"archived"}]
            }
        });
        let progress = DeclaredProgress::from_metrics(&actual_metrics(&report)).unwrap();
        assert_eq!(progress.applicable, 0);
        assert_eq!(progress.percent, None);
        let html = render(&report);
        assert!(html.contains("NOT_APPLICABLE"));
        assert!(html.contains("UNKNOWN"));
    }

    #[test]
    fn forged_ledger_count_or_status_suppresses_derived_progress() {
        let mut report = json!({
            "work_index": {
                "status":"STRUCTURALLY_VALID",
                "contract_count":2,
                "contracts":[{"id":"WORK-A","status":"verified"}]
            }
        });
        assert!(DeclaredProgress::from_metrics(&actual_metrics(&report)).is_none());
        report["work_index"]["contract_count"] = json!(1);
        report["work_index"]["contracts"][0]["status"] = json!("unrecognized");
        assert!(DeclaredProgress::from_metrics(&actual_metrics(&report)).is_none());
    }

    #[test]
    fn offline_report_has_no_external_urls_or_dependencies() {
        let report = json!({
            "work_index": {"contract_files": [], "findings": []},
            "compatibility": {"protocol": {"limitations": []}},
            "context": {"records": [], "conflicts": []},
            "limitations": [],
            "scan_scope": {"status": "COMPLETE"}
        });
        let page = render(&report);
        assert!(page.contains("Content-Security-Policy"));
        assert!(page.contains("connect-src 'none'"));
        assert!(!page.contains("https://"));
        assert!(!page.contains("<!--__"));
        assert!(page.contains("id=\"contract-search\""));
        assert!(page.contains("No structured work contracts are available"));
    }
}
