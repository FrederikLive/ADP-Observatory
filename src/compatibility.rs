use crate::work::WorkInspection;
use serde::Serialize;
use std::{collections::BTreeSet, fs, path::Path};

const MAX_PROTOCOL_BYTES: u64 = 256 * 1024;

#[derive(Serialize)]
pub struct Compatibility {
    pub format: &'static str,
    pub semantics: &'static str,
    pub semantic_scope: &'static str,
    pub conformance: &'static str,
    pub protocol: ProtocolVersion,
}

#[derive(Serialize)]
pub struct ProtocolVersion {
    pub status: &'static str,
    pub value: Option<String>,
    pub sources: Vec<VersionSource>,
    pub limitations: Vec<String>,
}

#[derive(Serialize)]
pub struct VersionSource {
    pub path: &'static str,
    pub version: String,
}

pub fn inspect(root: &Path, work: &WorkInspection) -> Compatibility {
    let protocol = detect_version(root);
    let format = match work.status {
        "STRUCTURALLY_VALID" | "UNSUPPORTED_SCHEMA" => "READABLE",
        "INVALID" => "INVALID",
        _ if work.findings.iter().any(|f| f.code == "work.missing") => "NOT_PRESENT",
        _ => "UNREADABLE",
    };
    let semantics = if work.status == "UNSUPPORTED_SCHEMA" {
        "UNSUPPORTED"
    } else if format != "READABLE" {
        "NOT_ASSESSED"
    } else {
        match (protocol.status, protocol.value.as_deref()) {
            ("KNOWN", Some("2.7.1" | "2.8.0")) => "SUPPORTED",
            ("KNOWN", _) => "UNSUPPORTED",
            ("CONFLICT", _) => "CONFLICT",
            _ => "UNKNOWN",
        }
    };
    Compatibility {
        format,
        semantics,
        semantic_scope: "WORK_JSON_SCHEMA_V1_ONLY",
        conformance: "NOT_ASSESSED",
        protocol,
    }
}

fn detect_version(root: &Path) -> ProtocolVersion {
    let candidates: &[(&str, bool)] = &[
        ("ADP.md", false),
        ("ADP.min.md", true),
        (".adp/protocol/ADP.md", false),
        (".adp/protocol/ADP.min.md", true),
    ];
    let mut sources = Vec::new();
    let mut limitations = Vec::new();
    for &(relative, compact) in candidates {
        match read_protocol_file(root, relative) {
            Ok(None) => {}
            Ok(Some(text)) => match extract_version(&text, compact) {
                Some(version) => sources.push(VersionSource {
                    path: relative,
                    version,
                }),
                None => limitations.push(format!(
                    "No recognizable ADP protocol version header in {relative}."
                )),
            },
            Err(reason) => limitations.push(format!("{relative}: {reason}")),
        }
    }
    let versions: BTreeSet<_> = sources.iter().map(|s| s.version.clone()).collect();
    let status = match versions.len() {
        0 => "UNKNOWN",
        1 => "KNOWN",
        _ => "CONFLICT",
    };
    let value = if status == "KNOWN" {
        versions.into_iter().next()
    } else {
        None
    };
    ProtocolVersion {
        status,
        value,
        sources,
        limitations,
    }
}

fn read_protocol_file(root: &Path, relative: &str) -> Result<Option<String>, String> {
    let path = root.join(relative);
    let mut current = root.to_path_buf();
    let components: Vec<_> = Path::new(relative).components().collect();
    for (index, component) in components.iter().enumerate() {
        current.push(component.as_os_str());
        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("cannot read metadata: {e}")),
        };
        if metadata.file_type().is_symlink() {
            return Err("symbolic-link path is not followed".into());
        }
        if index + 1 < components.len() && !metadata.is_dir() {
            return Err("non-directory parent in protocol path".into());
        }
        if index + 1 == components.len() {
            if !metadata.is_file() {
                return Err("protocol path is not a regular file".into());
            }
            if metadata.len() > MAX_PROTOCOL_BYTES {
                return Err("protocol file exceeds the safe read limit".into());
            }
        }
    }
    let bytes = fs::read(path).map_err(|e| format!("cannot read protocol file: {e}"))?;
    if bytes.len() as u64 > MAX_PROTOCOL_BYTES {
        return Err("protocol file exceeds the safe read limit".into());
    }
    String::from_utf8(bytes)
        .map(Some)
        .map_err(|_| "protocol file is not UTF-8".into())
}

fn extract_version(text: &str, compact: bool) -> Option<String> {
    for line in text.lines().take(16) {
        let remainder = if compact {
            line.split_once("Protocol version:").map(|(_, rest)| rest)
        } else if line.contains("Agent Development Protocol") {
            line.split_once("Version ").map(|(_, rest)| rest)
        } else {
            None
        };
        if let Some(rest) = remainder {
            let version: String = rest
                .trim_start_matches(|c: char| c.is_whitespace() || c == '*')
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if version.split('.').count() == 3
                && version
                    .split('.')
                    .all(|part| !part.is_empty() && part.parse::<u32>().is_ok())
            {
                return Some(version);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::work;
    use std::{
        sync::atomic::{AtomicU64, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    static NEXT_TEST_ROOT: AtomicU64 = AtomicU64::new(0);

    fn with_project(test: impl FnOnce(&Path)) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "adp-observatory-compat-{}-{nonce}-{}",
            std::process::id(),
            NEXT_TEST_ROOT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).expect("create root");
        test(&root);
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn missing_optional_records_are_not_conformance_failures() {
        with_project(|root| {
            let report = inspect(root, &work::inspect(root));
            assert_eq!(report.protocol.status, "UNKNOWN");
            assert_eq!(report.format, "NOT_PRESENT");
            assert_eq!(report.semantics, "NOT_ASSESSED");
            assert_eq!(report.conformance, "NOT_ASSESSED");
        });
    }

    #[test]
    fn detects_explicit_current_version() {
        with_project(|root| {
            fs::write(
                root.join("ADP.md"),
                "# ADP.md\n> **Agent Development Protocol — Version 2.7.1 (2026-10-18)**",
            )
            .expect("write");
            let report = inspect(root, &work::inspect(root));
            assert_eq!(report.protocol.status, "KNOWN");
            assert_eq!(report.protocol.value.as_deref(), Some("2.7.1"));
            assert_eq!(report.format, "NOT_PRESENT");
        });
    }

    #[test]
    fn adp_280_work_schema_v1_is_supported_but_not_certified() {
        with_project(|root| {
            fs::write(
                root.join("ADP.md"),
                "# ADP.md\n> **Agent Development Protocol — Version 2.8.0 (2026-10-23)**",
            )
            .expect("write protocol");
            fs::create_dir_all(root.join(".project")).expect("create directory");
            fs::write(
                root.join(".project/WORK.json"),
                r#"{"schema_version":1,"project":"P","contracts":[{"id":"WORK-A","title":"A","status":"ready","depends_on":[],"execution":{"assignment":{"role":"reviewer","selection_basis":"capability_based"}}}]}"#,
            )
            .expect("write work index");
            let work = work::inspect(root);
            assert_eq!(work.status, "STRUCTURALLY_VALID");
            let report = inspect(root, &work);
            assert_eq!(report.protocol.value.as_deref(), Some("2.8.0"));
            assert_eq!(report.format, "READABLE");
            assert_eq!(report.semantics, "SUPPORTED");
            assert_eq!(report.semantic_scope, "WORK_JSON_SCHEMA_V1_ONLY");
            assert_eq!(report.conformance, "NOT_ASSESSED");
        });
    }

    #[test]
    fn unknown_newer_versions_never_claim_semantic_support() {
        with_project(|root| {
            fs::write(
                root.join("ADP.md"),
                "# ADP.md\n> **Agent Development Protocol — Version 3.0.0**",
            )
            .expect("write");
            fs::create_dir_all(root.join(".project")).expect("mkdir");
            fs::write(
                root.join(".project/WORK.json"),
                r#"{"schema_version":1,"project":"P","contracts":[]}"#,
            )
            .expect("write");
            let report = inspect(root, &work::inspect(root));
            assert_eq!(report.format, "READABLE");
            assert_eq!(report.semantics, "UNSUPPORTED");
            assert_eq!(report.conformance, "NOT_ASSESSED");
        });
    }

    #[test]
    fn newer_work_schema_is_displayable_but_not_semantically_supported() {
        with_project(|root| {
            fs::write(
                root.join("ADP.md"),
                "# ADP.md\n> **Agent Development Protocol — Version 2.7.1**",
            )
            .expect("write");
            fs::create_dir_all(root.join(".project")).expect("mkdir");
            fs::write(
                root.join(".project/WORK.json"),
                r#"{"schema_version":2,"project":"P","contracts":[]}"#,
            )
            .expect("write");
            let report = inspect(root, &work::inspect(root));
            assert_eq!(report.format, "READABLE");
            assert_eq!(report.semantics, "UNSUPPORTED");
            assert_eq!(report.conformance, "NOT_ASSESSED");
        });
    }

    #[test]
    fn conflicting_versions_keep_provenance() {
        with_project(|root| {
            fs::write(
                root.join("ADP.md"),
                "# ADP.md\n> **Agent Development Protocol — Version 2.7.1**",
            )
            .expect("write");
            fs::write(root.join("ADP.min.md"), "**Protocol version:** 3.0.0").expect("write");
            let report = detect_version(root);
            assert_eq!(report.status, "CONFLICT");
            assert_eq!(report.sources.len(), 2);
            assert!(report.value.is_none());
        });
    }
}
