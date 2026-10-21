use serde_json::Value;
use std::{fs, path::PathBuf, process::Command};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn scan(name: &str) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_adp-observatory"))
        .arg(fixture(name))
        .arg("--json")
        .output()
        .expect("run Observatory CLI");
    assert!(
        output.status.success(),
        "scanner failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("valid JSON report")
}

#[test]
fn current_version_reads_structured_work_without_claiming_conformance() {
    let report = scan("current");
    assert_eq!(report["schema_version"], "0.3.0");
    assert_eq!(report["work_index"]["status"], "STRUCTURALLY_VALID");
    assert_eq!(report["compatibility"]["protocol"]["value"], "2.7.1");
    assert_eq!(report["compatibility"]["format"], "READABLE");
    assert_eq!(report["compatibility"]["semantics"], "SUPPORTED");
    assert_eq!(report["compatibility"]["conformance"], "NOT_ASSESSED");
    assert!(report["context"]["conflicts"]
        .as_array()
        .expect("conflicts")
        .is_empty());
}

#[test]
fn absent_optional_work_index_does_not_fail_independent_context() {
    let report = scan("partial");
    assert_eq!(report["work_index"]["status"], "UNKNOWN");
    assert_eq!(report["compatibility"]["format"], "NOT_PRESENT");
    assert_eq!(report["compatibility"]["conformance"], "NOT_ASSESSED");
    assert_eq!(report["context"]["records"][0]["state"], "MISSING");
    assert_eq!(report["context"]["records"][1]["state"], "READABLE");
    assert_eq!(
        report["context"]["active_contract_claims"][0]["value"],
        "WORK-PARTIAL"
    );
}

#[test]
fn malformed_json_does_not_hide_valid_continuity_data() {
    let report = scan("malformed");
    assert_eq!(report["work_index"]["status"], "INVALID");
    assert_eq!(report["compatibility"]["format"], "INVALID");
    assert_eq!(report["context"]["records"][1]["state"], "READABLE");
    assert_eq!(
        report["context"]["active_contract_claims"][0]["value"],
        "WORK-SURVIVES"
    );
}

#[test]
fn older_version_is_not_assumed_semantically_compatible() {
    let report = scan("older");
    assert_eq!(report["compatibility"]["protocol"]["value"], "2.6.0");
    assert_eq!(report["compatibility"]["format"], "READABLE");
    assert_eq!(report["compatibility"]["semantics"], "UNSUPPORTED");
    assert_eq!(report["compatibility"]["conformance"], "NOT_ASSESSED");
}

#[test]
fn future_schema_remains_readable_without_unverified_semantics() {
    let report = scan("future");
    assert_eq!(report["compatibility"]["protocol"]["value"], "3.0.0");
    assert_eq!(report["compatibility"]["format"], "READABLE");
    assert_eq!(report["compatibility"]["semantics"], "UNSUPPORTED");
    assert_eq!(report["work_index"]["status"], "UNSUPPORTED_SCHEMA");
    assert_eq!(report["compatibility"]["conformance"], "NOT_ASSESSED");
}

#[test]
fn conflicts_keep_all_source_claims_and_unknown_semantics() {
    let report = scan("conflict");
    assert_eq!(report["compatibility"]["protocol"]["status"], "CONFLICT");
    assert_eq!(report["compatibility"]["semantics"], "CONFLICT");
    let claims = report["context"]["conflicts"][0]["claims"]
        .as_array()
        .expect("claims array");
    assert_eq!(claims.len(), 3);
    assert_eq!(claims[0]["path"], ".project/WORK.json");
    assert_eq!(claims[1]["path"], "docs/STATUS.md");
    assert_eq!(claims[2]["path"], "docs/CONTINUITY.md");
    assert_eq!(report["compatibility"]["conformance"], "NOT_ASSESSED");
}

#[test]
fn scanner_output_is_deterministic_for_same_project_and_version() {
    assert_eq!(scan("current"), scan("current"));
}

#[test]
fn scanner_never_writes_target_records() {
    let target = fixture("current").join(".project/WORK.json");
    let before = fs::read(&target).expect("read before");
    let _ = scan("current");
    let after = fs::read(&target).expect("read after");
    assert_eq!(before, after);
}

#[test]
fn referenced_contract_files_are_checked_without_interpreting_them_as_conformance() {
    let report = scan("references");
    assert_eq!(report["work_index"]["status"], "STRUCTURALLY_VALID");
    assert_eq!(report["work_index"]["reference_inspection_limited"], false);
    let refs = report["work_index"]["contract_files"]
        .as_array()
        .expect("references");
    let statuses: Vec<_> = refs.iter().map(|r| r["status"].as_str().unwrap()).collect();
    assert_eq!(
        statuses,
        [
            "PRESENT",
            "MISSING",
            "OUTSIDE_ROOT",
            "INVALID_PATH",
            "NOT_FILE",
            "NOT_DIRECTORY"
        ]
    );
    assert_eq!(
        refs[0]["source"],
        ".project/WORK.json#/contracts/0/contract_file"
    );
    assert_eq!(report["compatibility"]["conformance"], "NOT_ASSESSED");
    assert_eq!(report["scan_scope"]["status"], "COMPLETE");
    assert_eq!(report["scan_complete"], true);
}

#[test]
fn scan_scope_reports_exclusions_separately_from_failures() {
    let report = scan("current");
    assert_eq!(report["scan_scope"]["status"], "COMPLETE");
    assert_eq!(report["scan_scope"]["depth_limit_hits"], 0);
    assert_eq!(report["scan_scope"]["metadata_errors"], 0);
    assert_eq!(report["scan_complete"], true);
}

#[test]
fn emitted_top_level_report_has_all_schema_required_fields() {
    let schema: Value =
        serde_json::from_str(include_str!("../schema/scan-report.schema.json")).expect("schema");
    let report = scan("current");
    for key in schema["required"].as_array().expect("required fields") {
        let key = key.as_str().expect("field");
        assert!(report.get(key).is_some(), "missing required field: {key}");
    }
}

#[test]
fn observatory_can_inspect_its_own_adp_work_records() {
    let output = Command::new(env!("CARGO_BIN_EXE_adp-observatory"))
        .arg(env!("CARGO_MANIFEST_DIR"))
        .arg("--json")
        .output()
        .expect("run repository self-scan");
    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).expect("JSON output");
    assert_eq!(report["work_index"]["status"], "STRUCTURALLY_VALID");
    let refs = report["work_index"]["contract_files"]
        .as_array()
        .expect("contract file references");
    assert!(!refs.is_empty());
    assert!(refs.iter().all(|r| r["status"] == "PRESENT"));
    assert_eq!(report["compatibility"]["conformance"], "NOT_ASSESSED");
    assert_eq!(report["context"]["conflicts"].as_array().unwrap().len(), 0);
}

#[test]
fn standalone_html_report_is_generated_outside_project() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("observatory-report-test-{nonce}"));
    fs::create_dir_all(&root).expect("report directory");
    let output_path = root.join("dashboard.html");
    let output = Command::new(env!("CARGO_BIN_EXE_adp-observatory"))
        .arg("report")
        .arg(fixture("current"))
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("run report command");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let html = fs::read_to_string(&output_path).expect("offline HTML");
    assert!(html.contains("A clearer view of your project"));
    assert!(html.contains("WORK-A"));
    assert!(html.contains("NOT_ASSESSED"));
    assert!(html.contains("connect-src 'none'"));
    assert!(html.contains("id=\"contract-search\""));
    assert!(!html.contains("<!--__"));
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn no_open_dashboard_mode_writes_report_without_browser() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("observatory-offline-test-{nonce}"));
    fs::create_dir_all(&root).expect("report directory");
    let output_path = root.join("offline.html");
    let output = Command::new(env!("CARGO_BIN_EXE_adp-observatory"))
        .arg("dashboard")
        .arg(fixture("partial"))
        .arg("--no-open")
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("offline dashboard");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_path.is_file());
    assert!(fs::read_to_string(&output_path)
        .unwrap()
        .contains("No explicit contract-file references"));
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn in_project_html_export_is_refused_without_writing() {
    let root = fixture("current");
    let forbidden = root.join("should-not-exist.html");
    let output = Command::new(env!("CARGO_BIN_EXE_adp-observatory"))
        .arg("report")
        .arg(&root)
        .arg("--output")
        .arg(&forbidden)
        .output()
        .expect("run report");
    assert!(!output.status.success());
    assert!(!forbidden.exists());
}

#[test]
fn help_and_version_require_no_project_directory() {
    let executable = env!("CARGO_BIN_EXE_adp-observatory");
    let help = Command::new(executable)
        .arg("--help")
        .output()
        .expect("help");
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("adp-observatory scan"));
    let version = Command::new(executable)
        .arg("--version")
        .output()
        .expect("version");
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).contains("ADP-Observatory"));
}
