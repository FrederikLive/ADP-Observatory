use crate::references::{self, ContractFileReference};
use serde::Serialize;
use std::{collections::HashSet, fs, path::Path};

const MAX_WORK_BYTES: u64 = 2 * 1024 * 1024;
const MAX_REFERENCE_CHECKS: usize = 2000;

#[derive(Serialize)]
pub struct WorkInspection {
    pub status: &'static str,
    pub project: Option<String>,
    pub active_contract: Option<String>,
    pub contract_count: Option<usize>,
    pub contracts: Vec<WorkSummary>,
    pub contract_files: Vec<ContractFileReference>,
    pub reference_inspection_limited: bool,
    pub findings: Vec<WorkFinding>,
}

#[derive(Serialize)]
pub struct WorkSummary {
    pub id: String,
    pub title: String,
    pub status: String,
    pub workstream: Option<String>,
    pub depends_on: Vec<String>,
    pub contract_file: Option<String>,
}

#[derive(Serialize)]
pub struct WorkFinding {
    pub code: &'static str,
    pub severity: &'static str,
    pub message: String,
    pub source: &'static str,
}

fn finding(code: &'static str, severity: &'static str, message: impl Into<String>) -> WorkFinding {
    WorkFinding {
        code,
        severity,
        message: message.into(),
        source: ".project/WORK.json",
    }
}

fn nullable_string(value: &serde_json::Value) -> bool {
    value.is_null() || value.is_string()
}

pub fn inspect(root: &Path) -> WorkInspection {
    let mut result = WorkInspection {
        status: "UNKNOWN",
        project: None,
        active_contract: None,
        contract_count: None,
        contracts: Vec::new(),
        contract_files: Vec::new(),
        reference_inspection_limited: false,
        findings: Vec::new(),
    };
    let parent = root.join(".project");
    match fs::symlink_metadata(&parent) {
        Ok(meta) if meta.file_type().is_symlink() || !meta.is_dir() => {
            result.findings.push(finding(
                "work.unsafe_parent",
                "warning",
                "Work-index parent is not a regular directory; refusing to follow it.",
            ));
            return result;
        }
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            result.findings.push(finding(
                "work.missing",
                "info",
                "Optional WORK.json not found; work-index checks not assessed.",
            ));
            return result;
        }
        Err(e) => {
            result.findings.push(finding(
                "work.metadata_error",
                "warning",
                format!("Cannot inspect work-index directory: {e}"),
            ));
            return result;
        }
    }
    let path = parent.join("WORK.json");
    let meta = match fs::symlink_metadata(&path) {
        Ok(meta) => meta,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            result.findings.push(finding(
                "work.missing",
                "info",
                "Optional WORK.json not found; work-index checks not assessed.",
            ));
            return result;
        }
        Err(e) => {
            result.findings.push(finding(
                "work.metadata_error",
                "warning",
                format!("Cannot inspect work index: {e}"),
            ));
            return result;
        }
    };
    if !meta.is_file() || meta.file_type().is_symlink() || meta.len() > MAX_WORK_BYTES {
        result.findings.push(finding(
            "work.unsafe_file",
            "warning",
            "Work index is not a supported regular file within the size limit.",
        ));
        return result;
    }
    let bytes = match fs::read(&path) {
        Ok(bytes) if bytes.len() as u64 <= MAX_WORK_BYTES => bytes,
        _ => {
            result.findings.push(finding(
                "work.read_error",
                "warning",
                "Cannot read work index within size limit.",
            ));
            return result;
        }
    };
    let value: serde_json::Value = match serde_json::from_slice(&bytes) {
        Ok(value) => value,
        Err(e) => {
            result.status = "INVALID";
            result.findings.push(finding(
                "work.invalid_json",
                "error",
                format!("Invalid JSON: {e}"),
            ));
            return result;
        }
    };
    let Some(object) = value.as_object() else {
        result.status = "INVALID";
        result.findings.push(finding(
            "work.invalid_root",
            "error",
            "Expected a JSON object.",
        ));
        return result;
    };
    let schema_version = object.get("schema_version").and_then(|v| v.as_u64());
    if schema_version.is_some_and(|version| version > 1) {
        result.status = "UNSUPPORTED_SCHEMA";
        result.project = object
            .get("project")
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        result.active_contract = object
            .get("active_contract")
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        result.contract_count = object
            .get("contracts")
            .and_then(|v| v.as_array())
            .map(Vec::len);
        result.findings.push(finding(
            "work.unsupported_schema",
            "info",
            format!(
                "WORK.json schema version {} is not supported; no schema-v1 verdict is issued.",
                schema_version.unwrap_or_default()
            ),
        ));
        return result;
    }
    if schema_version != Some(1) {
        result.findings.push(finding(
            "work.schema_version",
            "error",
            "Missing or invalid schema_version; expected schema version 1.",
        ));
    }
    for key in [
        "active_milestone",
        "active_contract",
        "next_milestone",
        "next_contract",
    ] {
        if object.get(key).is_some_and(|v| !nullable_string(v)) {
            result.findings.push(finding(
                "work.optional_field_type",
                "error",
                format!("{key} must be a string or null."),
            ));
        }
    }
    result.project = object
        .get("project")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_owned);
    if result.project.is_none() {
        result.findings.push(finding(
            "work.project",
            "error",
            "Missing nonempty project name.",
        ));
    }
    result.active_contract = object
        .get("active_contract")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let Some(contracts) = object.get("contracts").and_then(|v| v.as_array()) else {
        result.findings.push(finding(
            "work.contracts",
            "error",
            "Expected contracts array.",
        ));
        result.status = "INVALID";
        return result;
    };
    result.contract_count = Some(contracts.len());
    let mut ids = HashSet::new();
    let mut dependencies = Vec::new();
    const STATUSES: &[&str] = &[
        "candidate",
        "ready",
        "in_progress",
        "blocked",
        "implemented",
        "verifying",
        "verified",
        "deferred",
        "cancelled",
    ];
    for (index, contract) in contracts.iter().enumerate() {
        let Some(entry) = contract.as_object() else {
            result.findings.push(finding(
                "work.contract_shape",
                "error",
                format!("Contract {index} is not an object."),
            ));
            continue;
        };
        let id = entry.get("id").and_then(|v| v.as_str()).unwrap_or("");
        if !id.starts_with("WORK-")
            || id.len() <= 5
            || !id[5..]
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
        {
            result.findings.push(finding(
                "work.contract_id",
                "error",
                format!("Invalid contract ID at index {index}."),
            ));
        }
        if !ids.insert(id.to_owned()) {
            result.findings.push(finding(
                "work.duplicate_id",
                "error",
                format!("Duplicate contract ID: {id}"),
            ));
        }
        if entry
            .get("title")
            .and_then(|v| v.as_str())
            .is_none_or(|s| s.trim().is_empty())
        {
            result.findings.push(finding(
                "work.contract_title",
                "error",
                format!("Missing title for {id}."),
            ));
        }
        if !entry
            .get("status")
            .and_then(|v| v.as_str())
            .is_some_and(|s| STATUSES.contains(&s))
        {
            result.findings.push(finding(
                "work.contract_status",
                "error",
                format!("Invalid status for {id}."),
            ));
        }
        result.contracts.push(WorkSummary {
            id: id.to_owned(),
            title: entry
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned(),
            status: entry
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("UNKNOWN")
                .to_owned(),
            workstream: entry
                .get("workstream")
                .and_then(|v| v.as_str())
                .map(str::to_owned),
            depends_on: entry
                .get("depends_on")
                .and_then(|v| v.as_array())
                .map(|deps| {
                    deps.iter()
                        .filter_map(|v| v.as_str().map(str::to_owned))
                        .collect()
                })
                .unwrap_or_default(),
            contract_file: entry
                .get("contract_file")
                .and_then(|v| v.as_str())
                .map(str::to_owned),
        });
        if let Some(file) = entry.get("contract_file").and_then(|v| v.as_str()) {
            if result.contract_files.len() < MAX_REFERENCE_CHECKS {
                result
                    .contract_files
                    .push(references::inspect(root, id, index, file));
            } else if !result.reference_inspection_limited {
                result.reference_inspection_limited = true;
                result.findings.push(finding(
                    "work.reference_check_limit",
                    "warning",
                    "Contract-file reference check limit reached; remaining references were not assessed.",
                ));
            }
        }
        for key in ["workstream", "contract_file"] {
            if entry.get(key).is_some_and(|v| !nullable_string(v)) {
                result.findings.push(finding(
                    "work.contract_optional_type",
                    "error",
                    format!("{key} in {id} must be a string or null."),
                ));
            }
        }
        if let Some(execution) = entry.get("execution") {
            if !execution.is_null() {
                if let Some(fields) = execution.as_object() {
                    for (key, value) in fields {
                        let valid = match key.as_str() {
                            "mode" => matches!(value.as_str(), Some("direct" | "delegated")),
                            "mutation" => {
                                matches!(value.as_str(), Some("read_only" | "mutating"))
                            }
                            "branch" => nullable_string(value),
                            "assignment" => {
                                value.is_null()
                                    || value.as_object().is_some_and(|assignment| {
                                        assignment.iter().all(|(name, field)| {
                                            matches!(
                                                name.as_str(),
                                                "role" | "provider" | "model" | "selection_basis"
                                            ) && field.as_str().is_some_and(|text| !text.is_empty())
                                        })
                                    })
                            }
                            _ => false,
                        };
                        if !valid {
                            result.findings.push(finding(
                                "work.execution_field",
                                "error",
                                format!("Unsupported or invalid execution field {key} in {id}."),
                            ));
                        }
                    }
                } else {
                    result.findings.push(finding(
                        "work.execution_type",
                        "error",
                        format!("Execution metadata in {id} must be an object or null."),
                    ));
                }
            }
        }
        match entry.get("depends_on").and_then(|v| v.as_array()) {
            Some(deps) => {
                let mut seen = HashSet::new();
                for dep in deps {
                    match dep.as_str() {
                        Some(dep) => {
                            if !seen.insert(dep) {
                                result.findings.push(finding(
                                    "work.duplicate_dependency",
                                    "error",
                                    format!("Duplicate dependency in {id}: {dep}"),
                                ));
                            }
                            if dep == id {
                                result.findings.push(finding(
                                    "work.self_dependency",
                                    "error",
                                    format!("Self dependency in {id}."),
                                ));
                            }
                            dependencies.push((id.to_owned(), dep.to_owned()));
                        }
                        None => result.findings.push(finding(
                            "work.dependency_type",
                            "error",
                            format!("Non-string dependency in {id}."),
                        )),
                    }
                }
            }
            None => result.findings.push(finding(
                "work.dependencies",
                "error",
                format!("Missing depends_on array in {id}."),
            )),
        }
    }
    for (id, dep) in dependencies {
        if !ids.contains(&dep) {
            result.findings.push(finding(
                "work.missing_dependency",
                "error",
                format!("{id} depends on missing {dep}."),
            ));
        }
    }
    if let Some(active) = &result.active_contract {
        if !ids.contains(active) {
            result.findings.push(finding(
                "work.active_contract",
                "error",
                format!("Active contract not found: {active}"),
            ));
        }
    }
    result.status = if result.findings.iter().any(|f| f.severity == "error") {
        "INVALID"
    } else {
        "STRUCTURALLY_VALID"
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn with_work(content: &str, test: impl FnOnce(&Path)) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir =
            std::env::temp_dir().join(format!("adp-observatory-{}-{nonce}", std::process::id()));
        fs::create_dir_all(dir.join(".project")).expect("create");
        fs::write(dir.join(".project/WORK.json"), content).expect("write");
        test(&dir);
        fs::remove_dir_all(dir).expect("cleanup");
    }

    #[test]
    fn valid_work_index() {
        with_work(
            r#"{"schema_version":1,"project":"Example","active_contract":"WORK-1","contracts":[{"id":"WORK-1","title":"First","status":"in_progress","depends_on":[]}]}"#,
            |root| {
                let r = inspect(root);
                assert_eq!(r.status, "STRUCTURALLY_VALID");
                assert_eq!(r.contract_count, Some(1));
                assert_eq!(r.contracts[0].id, "WORK-1");
                assert_eq!(r.contracts[0].status, "in_progress");
            },
        );
    }

    #[test]
    fn invalid_json() {
        with_work("{", |root| {
            assert_eq!(inspect(root).status, "INVALID");
        });
    }

    #[test]
    fn unsupported_newer_schema_is_not_invalid() {
        with_work(
            r#"{"schema_version":2,"project":"Future","active_contract":"WORK-X","contracts":[]}"#,
            |root| {
                let r = inspect(root);
                assert_eq!(r.status, "UNSUPPORTED_SCHEMA");
                assert_eq!(r.project.as_deref(), Some("Future"));
                assert!(r
                    .findings
                    .iter()
                    .any(|f| f.code == "work.unsupported_schema"));
            },
        );
    }

    #[test]
    fn invalid_optional_fields_are_not_called_structurally_valid() {
        with_work(
            r#"{"schema_version":1,"project":"P","active_milestone":42,"contracts":[{"id":"WORK-A","title":"A","status":"ready","depends_on":[],"workstream":true,"execution":{"mode":"unknown","branch":9,"extra":true}}]}"#,
            |root| {
                let r = inspect(root);
                assert_eq!(r.status, "INVALID");
                assert!(r
                    .findings
                    .iter()
                    .any(|f| f.code == "work.optional_field_type"));
                assert!(r
                    .findings
                    .iter()
                    .any(|f| f.code == "work.contract_optional_type"));
                assert!(r.findings.iter().any(|f| f.code == "work.execution_field"));
            },
        );
    }

    #[test]
    fn adp_280_optional_assignment_is_structurally_valid() {
        with_work(
            r#"{"schema_version":1,"project":"P","contracts":[{"id":"WORK-A","title":"A","status":"in_progress","depends_on":[],"execution":{"mode":"delegated","mutation":"mutating","branch":"feature/a","assignment":{"role":"lead_programmer","provider":"openai","model":"example-model","selection_basis":"capability_based"}}}]}"#,
            |root| {
                let result = inspect(root);
                assert_eq!(result.status, "STRUCTURALLY_VALID");
                assert!(result.findings.is_empty());
            },
        );
    }

    #[test]
    fn null_assignment_is_permitted_by_optional_adp_280_schema() {
        with_work(
            r#"{"schema_version":1,"project":"P","contracts":[{"id":"WORK-A","title":"A","status":"ready","depends_on":[],"execution":{"assignment":null}}]}"#,
            |root| assert_eq!(inspect(root).status, "STRUCTURALLY_VALID"),
        );
    }

    #[test]
    fn invalid_assignment_fields_are_rejected_without_running_agents() {
        for assignment in [
            r#"{"role":""}"#,
            r#"{"provider":42}"#,
            r#"{"unrecognized":"not allowed"}"#,
            r#""not an object""#,
        ] {
            let payload = format!(
                r#"{{"schema_version":1,"project":"P","contracts":[{{"id":"WORK-A","title":"A","status":"ready","depends_on":[],"execution":{{"assignment":{assignment}}}}}]}}"#
            );
            with_work(&payload, |root| {
                let result = inspect(root);
                assert_eq!(result.status, "INVALID");
                assert!(result
                    .findings
                    .iter()
                    .any(|f| f.code == "work.execution_field"));
            });
        }
    }

    #[test]
    fn duplicate_contract_id_is_invalid() {
        with_work(
            r#"{"schema_version":1,"project":"P","contracts":[{"id":"WORK-A","title":"A","status":"ready","depends_on":[]},{"id":"WORK-A","title":"B","status":"ready","depends_on":[]}]}"#,
            |root| {
                let r = inspect(root);
                assert_eq!(r.status, "INVALID");
                assert!(r.findings.iter().any(|f| f.code == "work.duplicate_id"));
            },
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_work_directory_is_not_followed() {
        use std::os::unix::fs::symlink;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "adp-observatory-work-symlink-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("external")).expect("create");
        fs::write(
            root.join("external/WORK.json"),
            r#"{"schema_version":1,"project":"P","contracts":[]}"#,
        )
        .expect("write");
        symlink(root.join("external"), root.join(".project")).expect("symlink");
        let r = inspect(&root);
        assert_eq!(r.status, "UNKNOWN");
        assert!(r.findings.iter().any(|f| f.code == "work.unsafe_parent"));
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn missing_dependency_is_invalid() {
        with_work(
            r#"{"schema_version":1,"project":"Example","contracts":[{"id":"WORK-1","title":"First","status":"ready","depends_on":["WORK-2"]}]}"#,
            |root| {
                let r = inspect(root);
                assert_eq!(r.status, "INVALID");
                assert!(r
                    .findings
                    .iter()
                    .any(|f| f.code == "work.missing_dependency"));
            },
        );
    }
}
