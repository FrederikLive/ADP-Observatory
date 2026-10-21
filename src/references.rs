use serde::Serialize;
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

const MAX_REFERENCE_LENGTH: usize = 4096;
const MAX_COMPONENTS: usize = 64;

/// The source is an index within the original WORK.json, not an inferred file owner.
/// A resolved path means an existing regular file, not verified evidence or ADP compliance.
#[derive(Serialize)]
pub struct ContractFileReference {
    pub contract_id: String,
    pub source: String,
    pub path: String,
    pub status: &'static str,
    pub detail: Option<String>,
}

fn outcome(
    id: &str,
    index: usize,
    path: &str,
    status: &'static str,
    detail: Option<String>,
) -> ContractFileReference {
    ContractFileReference {
        contract_id: id.to_owned(),
        source: format!(".project/WORK.json#/contracts/{index}/contract_file"),
        path: path.to_owned(),
        status,
        detail,
    }
}

/// Inspect only explicit, optional schema-v1 contract_file references. Never open
/// referenced file contents, execute scripts, or follow symlinks at any path level.
pub fn inspect(root: &Path, id: &str, index: usize, raw: &str) -> ContractFileReference {
    if raw.is_empty()
        || raw.len() > MAX_REFERENCE_LENGTH
        || raw.contains('\\')
        || raw.contains('\0')
        || raw.contains(':')
    {
        return outcome(
            id,
            index,
            raw,
            "INVALID_PATH",
            Some("Unsupported or ambiguous portable contract-file path.".into()),
        );
    }

    if Path::new(raw).is_absolute() || raw.starts_with('/') {
        return outcome(
            id,
            index,
            raw,
            "OUTSIDE_ROOT",
            Some("Absolute paths are never resolved from a project record.".into()),
        );
    }

    let components: Vec<_> = raw
        .split('/')
        .filter(|c| !c.is_empty() && *c != ".")
        .collect();
    if components.iter().any(|c| *c == "..") {
        return outcome(
            id,
            index,
            raw,
            "OUTSIDE_ROOT",
            Some("Parent-directory traversal is not allowed.".into()),
        );
    }
    if components.is_empty() || components.len() > MAX_COMPONENTS {
        return outcome(
            id,
            index,
            raw,
            "INVALID_PATH",
            Some("No usable path components, or too many components.".into()),
        );
    }

    let mut current = PathBuf::from(root);
    for (position, component) in components.iter().enumerate() {
        current.push(component);
        let meta = match fs::symlink_metadata(&current) {
            Ok(meta) => meta,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return outcome(
                    id,
                    index,
                    raw,
                    "MISSING",
                    Some("Referenced contract file or parent directory does not exist.".into()),
                );
            }
            Err(error) => {
                return outcome(
                    id,
                    index,
                    raw,
                    "UNREADABLE",
                    Some(format!("Cannot inspect referenced path metadata: {error}")),
                );
            }
        };
        if meta.file_type().is_symlink() {
            return outcome(
                id,
                index,
                raw,
                "SYMLINK",
                Some("Symbolic links are not followed during contract-file inspection.".into()),
            );
        }
        let last = position + 1 == components.len();
        if last && !meta.is_file() {
            return outcome(
                id,
                index,
                raw,
                "NOT_FILE",
                Some("Referenced contract path is not a regular file.".into()),
            );
        }
        if !last && !meta.is_dir() {
            return outcome(
                id,
                index,
                raw,
                "NOT_DIRECTORY",
                Some("A parent component of the contract file is not a directory.".into()),
            );
        }
    }

    outcome(id, index, raw, "PRESENT", None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn with_root(test: impl FnOnce(&Path)) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "adp-observatory-reference-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("docs/work")).expect("directory");
        fs::write(root.join("docs/work/WORK-A.md"), "contract").expect("contract");
        test(&root);
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn existing_reference_is_only_present_not_verified() {
        with_root(|root| {
            let item = inspect(root, "WORK-A", 0, "docs/work/WORK-A.md");
            assert_eq!(item.status, "PRESENT");
            assert_eq!(item.source, ".project/WORK.json#/contracts/0/contract_file");
            assert_eq!(
                fs::read_to_string(root.join("docs/work/WORK-A.md")).unwrap(),
                "contract"
            );
        });
    }

    #[test]
    fn missing_reference_does_not_fail_other_items() {
        with_root(|root| {
            assert_eq!(
                inspect(root, "WORK-B", 1, "docs/work/nope.md").status,
                "MISSING"
            );
            assert_eq!(
                inspect(root, "WORK-A", 0, "docs/work/WORK-A.md").status,
                "PRESENT"
            );
        });
    }

    #[test]
    fn rejects_absolute_and_parent_traversal() {
        with_root(|root| {
            for path in [
                "/etc/passwd",
                "../escape.md",
                "docs/../../escape",
                "C:/escape.md",
            ] {
                let status = inspect(root, "WORK-X", 0, path).status;
                assert!(matches!(status, "OUTSIDE_ROOT" | "INVALID_PATH"));
            }
        });
    }

    #[test]
    fn rejects_ambiguous_cross_platform_paths() {
        with_root(|root| {
            for path in [
                r"docs\work\WORK-A.md",
                "",
                "docs/work/a\0b.md",
                "docs/work/a:b",
            ] {
                assert_eq!(inspect(root, "WORK-X", 0, path).status, "INVALID_PATH");
            }
        });
    }

    #[test]
    fn does_not_follow_non_directory_parents() {
        with_root(|root| {
            assert_eq!(
                inspect(root, "WORK-A", 0, "docs/work/WORK-A.md/child").status,
                "NOT_DIRECTORY"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn refuses_symlink_file_and_parent() {
        use std::os::unix::fs::symlink;
        with_root(|root| {
            symlink(
                root.join("docs/work/WORK-A.md"),
                root.join("docs/work/linked.md"),
            )
            .expect("file symlink");
            symlink(root.join("docs/work"), root.join("linked-directory"))
                .expect("directory symlink");
            assert_eq!(
                inspect(root, "WORK-X", 0, "docs/work/linked.md").status,
                "SYMLINK"
            );
            assert_eq!(
                inspect(root, "WORK-X", 0, "linked-directory/WORK-A.md").status,
                "SYMLINK"
            );
        });
    }
}
