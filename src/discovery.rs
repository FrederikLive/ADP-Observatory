use serde::Serialize;
use std::{fs, path::Path};

const MAX_DEPTH: usize = 8;
const MAX_ENTRIES: usize = 20_000;
const MAX_FILE_BYTES: u64 = 2 * 1024 * 1024;

#[derive(Serialize)]
pub struct ScanScope {
    pub status: &'static str,
    pub ignored_entries: usize,
    pub symlinks_excluded: usize,
    pub oversized_files_excluded: usize,
    pub nonregular_excluded: usize,
    pub metadata_errors: usize,
    pub directory_errors: usize,
    pub depth_limit_hits: usize,
    pub entry_limit_hits: usize,
}

impl ScanScope {
    fn new() -> Self {
        Self {
            status: "COMPLETE",
            ignored_entries: 0,
            symlinks_excluded: 0,
            oversized_files_excluded: 0,
            nonregular_excluded: 0,
            metadata_errors: 0,
            directory_errors: 0,
            depth_limit_hits: 0,
            entry_limit_hits: 0,
        }
    }

    fn finish(&mut self) {
        let partial = self.metadata_errors
            + self.directory_errors
            + self.depth_limit_hits
            + self.entry_limit_hits
            > 0;
        let exclusions = self.ignored_entries
            + self.symlinks_excluded
            + self.oversized_files_excluded
            + self.nonregular_excluded
            > 0;
        self.status = if partial {
            "PARTIAL"
        } else if exclusions {
            "COMPLETE_WITH_EXCLUSIONS"
        } else {
            "COMPLETE"
        };
    }
}

#[derive(Serialize)]
pub struct Discovery {
    pub files_examined: usize,
    pub files_skipped: usize,
    pub markers: Vec<String>,
    pub scope: ScanScope,
}

impl Discovery {
    pub fn inspect(root: &Path) -> Self {
        let mut result = Self {
            files_examined: 0,
            files_skipped: 0,
            markers: Vec::new(),
            scope: ScanScope::new(),
        };
        scan_directory(
            root,
            root,
            0,
            &mut result,
            MAX_DEPTH,
            MAX_ENTRIES,
            MAX_FILE_BYTES,
        );
        result.markers.sort();
        result.scope.finish();
        result
    }
}

fn scan_directory(
    root: &Path,
    dir: &Path,
    depth: usize,
    report: &mut Discovery,
    max_depth: usize,
    max_entries: usize,
    max_file_bytes: u64,
) {
    if depth > max_depth {
        report.scope.depth_limit_hits += 1;
        report.files_skipped += 1;
        return;
    }
    if report.files_examined + report.files_skipped >= max_entries {
        report.scope.entry_limit_hits += 1;
        return;
    }
    let iterator = match fs::read_dir(dir) {
        Ok(iterator) => iterator,
        Err(_) => {
            report.scope.directory_errors += 1;
            report.files_skipped += 1;
            return;
        }
    };

    // Bound per-directory collection before sorting for reproducibility. A
    // directory exceeding the limit remains PARTIAL rather than appearing green.
    let remaining = max_entries - (report.files_examined + report.files_skipped);
    let mut entries: Vec<_> = iterator.take(remaining + 1).collect();
    if entries.len() > remaining {
        report.scope.entry_limit_hits += 1;
        entries.pop();
    }
    entries.sort_by(|a, b| match (a.as_ref(), b.as_ref()) {
        (Ok(x), Ok(y)) => x.file_name().cmp(&y.file_name()),
        (Ok(_), Err(_)) => std::cmp::Ordering::Less,
        (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
        (Err(_), Err(_)) => std::cmp::Ordering::Equal,
    });

    for entry in entries {
        if report.files_examined + report.files_skipped >= max_entries {
            report.scope.entry_limit_hits += 1;
            break;
        }
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                report.scope.metadata_errors += 1;
                report.files_skipped += 1;
                continue;
            }
        };
        let path = entry.path();
        let name = entry.file_name();
        if [".git", "target", "node_modules", "vendor"]
            .iter()
            .any(|ignored| name == *ignored)
        {
            report.scope.ignored_entries += 1;
            report.files_skipped += 1;
            continue;
        }
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(_) => {
                report.scope.metadata_errors += 1;
                report.files_skipped += 1;
                continue;
            }
        };
        if metadata.file_type().is_symlink() {
            report.scope.symlinks_excluded += 1;
            report.files_skipped += 1;
            continue;
        }
        if metadata.is_dir() {
            scan_directory(
                root,
                &path,
                depth + 1,
                report,
                max_depth,
                max_entries,
                max_file_bytes,
            );
            continue;
        }
        if !metadata.is_file() {
            report.scope.nonregular_excluded += 1;
            report.files_skipped += 1;
            continue;
        }
        if metadata.len() > max_file_bytes {
            report.scope.oversized_files_excluded += 1;
            report.files_skipped += 1;
            continue;
        }

        report.files_examined += 1;
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if matches!(
            relative.as_str(),
            "ADP.md"
                | "ADP.min.md"
                | "AGENTS.md"
                | "STATUS.md"
                | "CONTINUITY.md"
                | ".project/WORK.json"
                | "docs/STATUS.md"
                | "docs/CONTINUITY.md"
                | "docs/CONFORMANCE.md"
        ) {
            report.markers.push(relative);
        }
    }
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
            "adp-observatory-scope-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("root");
        test(&root);
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn ignored_generated_directories_are_exclusions_not_failures() {
        with_root(|root| {
            fs::create_dir_all(root.join(".git")).expect(".git");
            fs::write(root.join("AGENTS.md"), "# Agent").expect("agent");
            let report = Discovery::inspect(root);
            assert_eq!(report.scope.status, "COMPLETE_WITH_EXCLUSIONS");
            assert_eq!(report.scope.ignored_entries, 1);
            assert_eq!(report.markers, vec!["AGENTS.md"]);
        });
    }

    #[test]
    fn depth_limit_marks_scan_partial() {
        with_root(|root| {
            fs::create_dir_all(root.join("a/b")).expect("nested");
            let mut report = Discovery {
                files_examined: 0,
                files_skipped: 0,
                markers: Vec::new(),
                scope: ScanScope::new(),
            };
            scan_directory(root, root, 0, &mut report, 0, 100, 1024);
            report.scope.finish();
            assert_eq!(report.scope.status, "PARTIAL");
            assert!(report.scope.depth_limit_hits > 0);
        });
    }

    #[test]
    fn file_limit_marks_scan_partial() {
        with_root(|root| {
            for index in 0..4 {
                fs::write(root.join(format!("file-{index}.txt")), "content").expect("file");
            }
            let mut report = Discovery {
                files_examined: 0,
                files_skipped: 0,
                markers: Vec::new(),
                scope: ScanScope::new(),
            };
            scan_directory(root, root, 0, &mut report, 8, 2, 1024);
            report.scope.finish();
            assert_eq!(report.scope.status, "PARTIAL");
            assert!(report.scope.entry_limit_hits > 0);
        });
    }

    #[test]
    fn oversized_file_is_an_explicit_exclusion() {
        with_root(|root| {
            fs::write(root.join("ADP.md"), "long content").expect("file");
            let mut report = Discovery {
                files_examined: 0,
                files_skipped: 0,
                markers: Vec::new(),
                scope: ScanScope::new(),
            };
            scan_directory(root, root, 0, &mut report, 8, 100, 2);
            report.scope.finish();
            assert_eq!(report.scope.status, "COMPLETE_WITH_EXCLUSIONS");
            assert_eq!(report.scope.oversized_files_excluded, 1);
            assert!(report.markers.is_empty());
        });
    }

    #[cfg(unix)]
    #[test]
    fn symlink_is_excluded_and_never_traversed() {
        use std::os::unix::fs::symlink;
        with_root(|root| {
            fs::create_dir_all(root.join("real")).expect("directory");
            symlink(root.join("real"), root.join("linked")).expect("link");
            let report = Discovery::inspect(root);
            assert_eq!(report.scope.status, "COMPLETE_WITH_EXCLUSIONS");
            assert_eq!(report.scope.symlinks_excluded, 1);
        });
    }
}
