use crate::work::WorkInspection;
use serde::Serialize;
use std::{collections::BTreeSet, fs, path::Path};

const MAX_CONTEXT_BYTES: u64 = 512 * 1024;

#[derive(Serialize)]
pub struct ContextInspection {
    pub records: Vec<ContextRecord>,
    pub active_contract_claims: Vec<Claim>,
    pub conflicts: Vec<Conflict>,
}

#[derive(Serialize)]
pub struct ContextRecord {
    pub path: &'static str,
    pub state: &'static str,
    pub limitation: Option<String>,
}

#[derive(Serialize)]
pub struct Claim {
    pub path: &'static str,
    pub line: Option<usize>,
    pub value: String,
}

#[derive(Serialize)]
pub struct Conflict {
    pub field: &'static str,
    pub claims: Vec<Claim>,
}

pub fn inspect(root: &Path, work: &WorkInspection) -> ContextInspection {
    let paths = ["docs/STATUS.md", "docs/CONTINUITY.md"];
    let mut records = Vec::new();
    let mut claims = Vec::new();

    if let Some(value) = &work.active_contract {
        if work.status == "STRUCTURALLY_VALID" || work.status == "INVALID" {
            claims.push(Claim {
                path: ".project/WORK.json",
                line: None,
                value: value.clone(),
            });
        }
    }

    for path in paths {
        match read_context(root, path) {
            Ok(None) => records.push(ContextRecord {
                path,
                state: "MISSING",
                limitation: Some("Record absent; independent checks continue.".into()),
            }),
            Ok(Some(text)) => {
                for (line, value) in parse_explicit_active_contract(&text) {
                    claims.push(Claim {
                        path,
                        line: Some(line),
                        value,
                    });
                }
                records.push(ContextRecord {
                    path,
                    state: "READABLE",
                    limitation: None,
                });
            }
            Err(reason) => records.push(ContextRecord {
                path,
                state: "UNREADABLE",
                limitation: Some(reason),
            }),
        }
    }

    let distinct: BTreeSet<_> = claims.iter().map(|c| c.value.as_str()).collect();
    let conflicts = if distinct.len() > 1 {
        vec![Conflict {
            field: "active_contract",
            claims: claims.clone(),
        }]
    } else {
        Vec::new()
    };
    ContextInspection {
        records,
        active_contract_claims: claims,
        conflicts,
    }
}

fn read_context(root: &Path, relative: &str) -> Result<Option<String>, String> {
    let mut current = root.to_path_buf();
    let components: Vec<_> = Path::new(relative).components().collect();
    for (index, component) in components.iter().enumerate() {
        current.push(component.as_os_str());
        let metadata = match fs::symlink_metadata(&current) {
            Ok(meta) => meta,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("metadata unavailable: {e}")),
        };
        if metadata.file_type().is_symlink() {
            return Err("symlink paths are not followed".into());
        }
        if index + 1 < components.len() && !metadata.is_dir() {
            return Err("non-directory parent".into());
        }
        if index + 1 == components.len() {
            if !metadata.is_file() {
                return Err("not a regular file".into());
            }
            if metadata.len() > MAX_CONTEXT_BYTES {
                return Err("file exceeds context read limit".into());
            }
        }
    }
    let bytes = fs::read(current).map_err(|e| format!("cannot read record: {e}"))?;
    if bytes.len() as u64 > MAX_CONTEXT_BYTES {
        return Err("file exceeds context read limit".into());
    }
    String::from_utf8(bytes)
        .map(Some)
        .map_err(|_| "record is not UTF-8".into())
}

fn parse_explicit_active_contract(text: &str) -> Vec<(usize, String)> {
    let mut claims = Vec::new();
    let mut in_section = false;
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            in_section = trimmed
                .trim_start_matches('#')
                .trim()
                .eq_ignore_ascii_case("active contract");
            continue;
        }
        let remainder = if in_section {
            Some(trimmed)
        } else {
            trimmed.split_once(':').and_then(|(label, value)| {
                if label
                    .trim()
                    .trim_start_matches('-')
                    .trim()
                    .eq_ignore_ascii_case("active contract")
                {
                    Some(value)
                } else {
                    None
                }
            })
        };
        if let Some(value) = remainder {
            if let Some(id) = extract_work_id(value) {
                claims.push((index + 1, id));
            }
        }
    }
    claims
}

fn extract_work_id(text: &str) -> Option<String> {
    for (start, _) in text.match_indices("WORK-") {
        if start > 0 {
            let previous = text[..start].chars().next_back()?;
            if previous.is_ascii_alphanumeric() || previous == '_' || previous == '-' {
                continue;
            }
        }
        let id: String = text[start..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
            .collect();
        if id.len() > 5 {
            return Some(id);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::work;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn with_project(test: impl FnOnce(&Path)) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "adp-observatory-context-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("create");
        test(&root);
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn missing_context_does_not_erase_known_work_data() {
        with_project(|root| {
            fs::create_dir_all(root.join(".project")).expect("mkdir");
            fs::write(
                root.join(".project/WORK.json"),
                r#"{"schema_version":1,"project":"P","active_contract":"WORK-1","contracts":[{"id":"WORK-1","title":"First","status":"ready","depends_on":[]}]}"#,
            )
            .expect("write");
            let report = inspect(root, &work::inspect(root));
            assert_eq!(report.records.len(), 2);
            assert_eq!(report.active_contract_claims.len(), 1);
            assert!(report.conflicts.is_empty());
        });
    }

    #[test]
    fn conflicting_records_preserve_both_sources() {
        with_project(|root| {
            fs::create_dir_all(root.join("docs")).expect("mkdir");
            fs::create_dir_all(root.join(".project")).expect("mkdir");
            fs::write(
                root.join(".project/WORK.json"),
                r#"{"schema_version":1,"project":"P","active_contract":"WORK-1","contracts":[{"id":"WORK-1","title":"First","status":"ready","depends_on":[]}]}"#,
            )
            .expect("write");
            fs::write(
                root.join("docs/CONTINUITY.md"),
                "# Continuity\n## Active contract\nWORK-2\n",
            )
            .expect("write");
            let report = inspect(root, &work::inspect(root));
            assert_eq!(report.conflicts.len(), 1);
            assert_eq!(report.conflicts[0].claims.len(), 2);
            assert_eq!(report.conflicts[0].claims[1].line, Some(3));
        });
    }

    #[test]
    fn prose_without_explicit_field_is_not_invented_as_a_claim() {
        let claims =
            parse_explicit_active_contract("# Status\n## Background\nWORK-99 was once active.\n");
        assert!(claims.is_empty());
    }

    #[test]
    fn invalid_utf8_isolated_from_other_records() {
        with_project(|root| {
            fs::create_dir_all(root.join("docs")).expect("mkdir");
            fs::write(root.join("docs/STATUS.md"), [0xff, 0xfe]).expect("write");
            fs::write(
                root.join("docs/CONTINUITY.md"),
                "# Context\n## Active contract\nWORK-3\n",
            )
            .expect("write");
            let report = inspect(root, &work::inspect(root));
            assert_eq!(report.records[0].state, "UNREADABLE");
            assert_eq!(report.records[1].state, "READABLE");
            assert_eq!(report.active_contract_claims.len(), 1);
        });
    }

    #[test]
    fn analysis_never_modifies_source_records() {
        with_project(|root| {
            fs::create_dir_all(root.join("docs")).expect("mkdir");
            let path = root.join("docs/CONTINUITY.md");
            let original = b"# Continuity\n## Active contract\nWORK-8\n";
            fs::write(&path, original).expect("write");
            let _ = inspect(root, &work::inspect(root));
            assert_eq!(fs::read(&path).expect("read"), original);
        });
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_context_file_is_rejected() {
        use std::os::unix::fs::symlink;
        with_project(|root| {
            fs::create_dir_all(root.join("docs")).expect("mkdir");
            fs::write(root.join("outside.md"), "## Active contract\nWORK-7").expect("write");
            symlink(root.join("outside.md"), root.join("docs/CONTINUITY.md")).expect("symlink");
            let report = inspect(root, &work::inspect(root));
            assert_eq!(report.records[1].state, "UNREADABLE");
        });
    }
}
