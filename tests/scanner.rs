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
    assert_eq!(report["schema_version"], "0.2.0");
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
fn emitted_top_level_report_has_all_schema_required_fields() {
    let schema: Value =
        serde_json::from_str(include_str!("../schema/scan-report.schema.json")).expect("schema");
    let report = scan("current");
    for key in schema["required"].as_array().expect("required fields") {
        let key = key.as_str().expect("field");
        assert!(report.get(key).is_some(), "missing required field: {key}");
    }
}
