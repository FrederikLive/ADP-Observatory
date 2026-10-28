use serde_json::Value;

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
    let mut output = pairs
        .into_iter()
        .map(|(key, value)| keyval(key, &value))
        .collect::<String>();
    if let Some(source) = report.get("source_inspection") {
        let readable = source["documents"]
            .as_array()
            .map(|files| {
                files
                    .iter()
                    .filter(|file| file["state"] == "READABLE")
                    .count()
            })
            .unwrap_or(0);
        output.push_str(&keyval("Markdown snapshots", &readable.to_string()));
        output.push_str(&keyval(
            "Markdown scan state",
            at(source, "/status", "UNKNOWN"),
        ));
        output.push_str(&keyval(
            "Source bytes analyzed",
            &count(&source["bytes_read"]),
        ));
        output.push_str(&keyval("Source text in report", "None (metadata only)"));
    }
    output
}

/// A completely self-contained HTML report. Untrusted project data is escaped
/// as text; no project value is ever interpolated into CSS or executable JS.
pub fn render(report: &Value) -> String {
    let project = at(report, "/target", "Unknown project directory");
    let version = at(report, "/compatibility/protocol/value", "Unknown");
    let work = at(report, "/work_index/status", "UNKNOWN");
    let scope = at(report, "/scan_scope/status", "PARTIAL");
    let semantics = at(report, "/compatibility/semantics", "UNKNOWN");
    let available = report
        .pointer("/work_index/contract_count")
        .map(count)
        .unwrap_or_else(|| "—".into());
    let summary = [
        metric(
            "ADP version",
            version,
            "Based on recognized local version declarations",
        ),
        metric(
            "Work contracts",
            &available,
            "Declared records; statuses are not independently verified",
        ),
        metric(
            "Scan scope",
            scope,
            "Files excluded or unassessed are disclosed",
        ),
        metric(
            "Semantic coverage",
            semantics,
            "Not a complete protocol conformance check",
        ),
    ]
    .join("");
    let (findings, finding_count) = append_findings(report);
    TEMPLATE
        .replace("/*__STYLES__*/", STYLE)
        .replace("/*__SCRIPT__*/", SCRIPT)
        .replace("<!--__PROJECT_PATH__-->", &escape_html(project))
        .replace("<!--__METRICS__-->", &summary)
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
