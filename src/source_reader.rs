//! Safe, capability-relative, read-only Markdown snapshot foundation.
//! Document bytes and headings stay in memory; the serialized report contains
//! metadata only. A later, explicitly opted-in viewer may consume bounded views.
#[cfg(unix)]
use cap_fs_ext::OpenOptionsSyncExt;
use cap_fs_ext::{DirExt, FollowSymlinks, OpenOptionsFollowExt};
use cap_std::{
    ambient_authority,
    fs::{Dir, OpenOptions},
};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    io::{ErrorKind, Read},
    path::Path,
};

const MAX_DOCUMENTS: usize = 128;
const MAX_DOCUMENT_BYTES: usize = 128 * 1024;
const MAX_TOTAL_BYTES: usize = 8 * 1024 * 1024;
const MAX_PATH_BYTES: usize = 4096;
const MAX_COMPONENTS: usize = 64;
const MAX_LINES: usize = 20_000;
const MAX_LINE_BYTES: usize = 4096;
const MAX_HEADINGS: usize = 256;

#[derive(Serialize)]
pub struct SourceInspection {
    pub status: &'static str,
    pub documents: Vec<SourceSummary>,
    pub document_limit: usize,
    pub per_document_byte_limit: usize,
    pub total_byte_limit: usize,
    pub bytes_read: usize,
    pub omitted_documents: usize,
    pub source_text_embedded: bool,
}

#[derive(Serialize)]
pub struct SourceSummary {
    pub path: String,
    pub state: &'static str,
    pub bytes: Option<usize>,
    pub sha256: Option<String>,
    pub line_count: Option<usize>,
    pub heading_count: Option<usize>,
}

#[allow(dead_code)] // Retained for the future opt-in, source-backed Explorer view.
#[derive(Debug)]
struct Heading {
    level: u8,
    title: String,
    start_line: usize,
    end_line: usize,
}

#[derive(Debug)]
struct SourceSnapshot {
    text: String,
    headings: Vec<Heading>,
    digest: String,
    line_count: usize,
}

struct Reader {
    root: Dir,
    bytes_read: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum ReadFailure {
    InvalidPath,
    Unsupported,
    Missing,
    Unsafe,
    NotFile,
    TooLarge,
    BudgetLimited,
    InvalidUtf8,
    LongLine,
    Unreadable,
}

impl ReadFailure {
    fn state(&self) -> &'static str {
        match self {
            Self::InvalidPath => "INVALID_PATH",
            Self::Unsupported => "UNSUPPORTED",
            Self::Missing => "MISSING",
            Self::Unsafe => "UNSAFE",
            Self::NotFile => "NOT_FILE",
            Self::TooLarge => "TOO_LARGE",
            Self::BudgetLimited => "LIMITED",
            Self::InvalidUtf8 => "INVALID_UTF8",
            Self::LongLine => "LONG_LINE",
            Self::Unreadable => "UNREADABLE",
        }
    }
}

impl Reader {
    fn new(root: &Path) -> Result<Self, ReadFailure> {
        let root = Dir::open_ambient_dir(root, ambient_authority())
            .map_err(|_| ReadFailure::Unreadable)?;
        Ok(Self {
            root,
            bytes_read: 0,
        })
    }

    fn read_markdown(&mut self, path: &str) -> Result<SourceSnapshot, ReadFailure> {
        let parts = validate_path(path)?;
        if !path.to_ascii_lowercase().ends_with(".md") {
            return Err(ReadFailure::Unsupported);
        }

        // Each directory is opened relative to a held directory capability.
        // No check-then-fs::read of an ambient path is performed.
        let mut parent = self.root.try_clone().map_err(|_| ReadFailure::Unreadable)?;
        for part in &parts[..parts.len() - 1] {
            let entry = parent.symlink_metadata(part).map_err(map_io)?;
            if entry.file_type().is_symlink() {
                return Err(ReadFailure::Unsafe);
            }
            if !entry.is_dir() {
                return Err(ReadFailure::NotFile);
            }
            parent = parent.open_dir_nofollow(part).map_err(map_open)?;
        }

        let name = parts[parts.len() - 1];
        let entry = parent.symlink_metadata(name).map_err(map_io)?;
        if entry.file_type().is_symlink() {
            return Err(ReadFailure::Unsafe);
        }
        if !entry.is_file() {
            return Err(ReadFailure::NotFile);
        }
        if entry.len() > MAX_DOCUMENT_BYTES as u64 {
            return Err(ReadFailure::TooLarge);
        }

        let mut options = OpenOptions::new();
        options.read(true).follow(FollowSymlinks::No);
        #[cfg(unix)]
        options.nonblock(true); // Avoid a blocking special-file open before fstat.
        let file = parent.open_with(name, &options).map_err(map_open)?;
        let opened = file.metadata().map_err(map_io)?;
        if !opened.is_file() || opened.file_type().is_symlink() {
            return Err(ReadFailure::Unsafe);
        }
        if opened.len() > MAX_DOCUMENT_BYTES as u64 {
            return Err(ReadFailure::TooLarge);
        }
        let remaining = MAX_TOTAL_BYTES.saturating_sub(self.bytes_read);
        if opened.len() > remaining as u64 {
            return Err(ReadFailure::BudgetLimited);
        }

        // Read one extra byte, detecting growth during the read. Never trust
        // metadata alone and never allocate based on untrusted file lengths.
        let allowed = remaining.min(MAX_DOCUMENT_BYTES);
        let mut bytes = Vec::new();
        file.take((allowed + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(map_io)?;
        self.bytes_read += bytes.len().min(remaining);
        if bytes.len() > MAX_DOCUMENT_BYTES {
            return Err(ReadFailure::TooLarge);
        }
        if bytes.len() > remaining {
            return Err(ReadFailure::BudgetLimited);
        }
        let digest = format!("{:x}", Sha256::digest(&bytes));
        let text = String::from_utf8(bytes).map_err(|_| ReadFailure::InvalidUtf8)?;
        let line_count = text.lines().count();
        if line_count > MAX_LINES || text.lines().any(|line| line.len() > MAX_LINE_BYTES) {
            return Err(ReadFailure::LongLine);
        }
        let headings = parse_headings(&text);
        Ok(SourceSnapshot {
            text,
            headings,
            digest,
            line_count,
        })
    }
}

fn map_io(error: std::io::Error) -> ReadFailure {
    if error.kind() == ErrorKind::NotFound {
        ReadFailure::Missing
    } else {
        ReadFailure::Unreadable
    }
}

fn map_open(error: std::io::Error) -> ReadFailure {
    if error.kind() == ErrorKind::NotFound {
        ReadFailure::Missing
    } else if error.kind() == ErrorKind::PermissionDenied {
        ReadFailure::Unreadable
    } else {
        // This includes ELOOP/Windows reparse refusal and unexpected races;
        // don't fall back to a less restrictive open.
        ReadFailure::Unsafe
    }
}

fn validate_path(path: &str) -> Result<Vec<&str>, ReadFailure> {
    if path.is_empty()
        || path.len() > MAX_PATH_BYTES
        || path.contains('\\')
        || path
            .chars()
            .any(|c| c.is_control() || matches!(c, ':' | '*' | '?' | '"' | '<' | '>' | '|'))
        || path.starts_with('/')
    {
        return Err(ReadFailure::InvalidPath);
    }
    let parts: Vec<_> = path.split('/').collect();
    if parts.len() > MAX_COMPONENTS
        || parts.iter().any(|p| {
            p.is_empty() || *p == "." || *p == ".." || (p.ends_with('.') || p.ends_with(' '))
        })
    {
        return Err(ReadFailure::InvalidPath);
    }
    // Windows reserves these names even with a file extension.
    if parts.iter().any(|p| {
        let stem = p.split('.').next().unwrap_or("").to_ascii_uppercase();
        matches!(
            stem.as_str(),
            "CON"
                | "PRN"
                | "AUX"
                | "NUL"
                | "COM1"
                | "COM2"
                | "COM3"
                | "COM4"
                | "COM5"
                | "COM6"
                | "COM7"
                | "COM8"
                | "COM9"
                | "LPT1"
                | "LPT2"
                | "LPT3"
                | "LPT4"
                | "LPT5"
                | "LPT6"
                | "LPT7"
                | "LPT8"
                | "LPT9"
        )
    }) {
        return Err(ReadFailure::InvalidPath);
    }
    Ok(parts)
}

fn parse_headings(text: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut current: Option<Heading> = None;
    for (event, range) in Parser::new(text).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current = Some(Heading {
                    level: match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    },
                    title: String::new(),
                    start_line: line_at(text, range.start),
                    end_line: line_at(text, range.end),
                });
            }
            Event::Text(value) | Event::Code(value) if current.is_some() => {
                if let Some(heading) = &mut current {
                    let room = 256_usize.saturating_sub(heading.title.len());
                    heading
                        .title
                        .push_str(&value.chars().take(room).collect::<String>());
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(mut heading) = current.take() {
                    heading.end_line = line_at(text, range.end.saturating_sub(1));
                    if headings.len() < MAX_HEADINGS {
                        headings.push(heading);
                    }
                }
            }
            _ => {}
        }
    }
    headings
}

fn line_at(text: &str, byte_index: usize) -> usize {
    text.as_bytes()[..byte_index.min(text.len())]
        .iter()
        .filter(|&&byte| byte == b'\n')
        .count()
        + 1
}

/// Read only named, in-root documents (bounded by count and aggregate bytes).
/// Do not serialize document contents: opt-in source export is a later feature.
pub fn inspect(root: &Path, contract_paths: impl Iterator<Item = String>) -> SourceInspection {
    // Respect work ledger order so the document limit cannot starve
    // active/recent contracts; independent context records always come first.
    let mut seen = BTreeSet::new();
    let mut paths = Vec::new();
    for path in ["docs/STATUS.md".to_owned(), "docs/CONTINUITY.md".to_owned()]
        .into_iter()
        .chain(contract_paths)
    {
        if seen.insert(path.clone()) {
            paths.push(path);
        }
    }
    let omitted_documents = paths.len().saturating_sub(MAX_DOCUMENTS);
    let reader = Reader::new(root);
    let mut inspection = SourceInspection {
        status: "COMPLETE",
        documents: Vec::new(),
        document_limit: MAX_DOCUMENTS,
        per_document_byte_limit: MAX_DOCUMENT_BYTES,
        total_byte_limit: MAX_TOTAL_BYTES,
        bytes_read: 0,
        omitted_documents,
        source_text_embedded: false,
    };
    match reader {
        Ok(mut reader) => {
            for path in paths.into_iter().take(MAX_DOCUMENTS) {
                let record = match reader.read_markdown(&path) {
                    Ok(snapshot) => SourceSummary {
                        path,
                        state: "READABLE",
                        bytes: Some(snapshot.text.len()),
                        sha256: Some(snapshot.digest),
                        line_count: Some(snapshot.line_count),
                        heading_count: Some(snapshot.headings.len()),
                    },
                    Err(failure) => SourceSummary {
                        path,
                        state: failure.state(),
                        bytes: None,
                        sha256: None,
                        line_count: None,
                        heading_count: None,
                    },
                };
                inspection.documents.push(record);
            }
            inspection.bytes_read = reader.bytes_read;
        }
        Err(_) => {
            inspection.status = "PARTIAL";
            for path in paths.into_iter().take(MAX_DOCUMENTS) {
                inspection.documents.push(SourceSummary {
                    path,
                    state: "UNREADABLE",
                    bytes: None,
                    sha256: None,
                    line_count: None,
                    heading_count: None,
                });
            }
        }
    }
    if omitted_documents > 0
        || inspection
            .documents
            .iter()
            .any(|d| d.state != "READABLE" && d.state != "MISSING")
    {
        inspection.status = "PARTIAL";
    }
    inspection
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn with_project(test: impl FnOnce(&Path)) {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("obs-source-{}-{id}", std::process::id()));
        fs::create_dir_all(root.join("docs/work")).unwrap();
        test(&root);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reads_headings_with_exact_source_lines_without_serializing_text() {
        with_project(|root| {
            let path = root.join("docs/work/WORK-É.md");
            let text = "# Purpose\n\nExample\n\n```md\n# Not a heading\n```\n\n## Acceptance criteria\n- [ ] Check\n";
            fs::write(&path, text).unwrap();
            let mut reader = Reader::new(root).unwrap();
            let result = reader.read_markdown("docs/work/WORK-É.md").unwrap();
            assert_eq!(result.headings.len(), 2);
            assert_eq!(result.headings[0].level, 1);
            assert_eq!(result.headings[0].title, "Purpose");
            assert_eq!(result.headings[0].start_line, 1);
            assert_eq!(result.headings[1].start_line, 9);
            assert_eq!(result.headings[1].title, "Acceptance criteria");
            assert_eq!(result.line_count, 10);
            let output = serde_json::to_string(&inspect(
                root,
                ["docs/work/WORK-É.md".to_owned()].into_iter(),
            ))
            .unwrap();
            assert!(!output.contains("Acceptance criteria"));
            assert!(!output.contains("Example"));
            assert!(output.contains("READABLE"));
        });
    }

    #[test]
    fn refuses_outside_paths_and_unsupported_content() {
        with_project(|root| {
            let mut reader = Reader::new(root).unwrap();
            for name in [
                "../private.md",
                "/tmp/private.md",
                "C:/private.md",
                "docs/./work.md",
                "docs//a.md",
                "docs/CON.md",
            ] {
                assert_eq!(
                    reader.read_markdown(name).unwrap_err(),
                    ReadFailure::InvalidPath
                );
            }
            assert_eq!(
                reader.read_markdown("docs/file.json").unwrap_err(),
                ReadFailure::Unsupported
            );
            assert_eq!(reader.bytes_read, 0);
        });
    }

    #[test]
    fn enforces_bounded_file_size_and_invalid_utf8() {
        with_project(|root| {
            fs::write(
                root.join("docs/work/huge.md"),
                vec![b'x'; MAX_DOCUMENT_BYTES + 1],
            )
            .unwrap();
            fs::write(root.join("docs/work/bad.md"), [0xff, 0xfe]).unwrap();
            fs::write(
                root.join("docs/work/long.md"),
                "x".repeat(MAX_LINE_BYTES + 1),
            )
            .unwrap();
            let mut reader = Reader::new(root).unwrap();
            assert_eq!(
                reader.read_markdown("docs/work/huge.md").unwrap_err(),
                ReadFailure::TooLarge
            );
            assert_eq!(
                reader.read_markdown("docs/work/bad.md").unwrap_err(),
                ReadFailure::InvalidUtf8
            );
            assert_eq!(
                reader.read_markdown("docs/work/long.md").unwrap_err(),
                ReadFailure::LongLine
            );
        });
    }

    #[test]
    fn absence_does_not_hide_other_records_or_create_missing_index() {
        with_project(|root| {
            fs::write(root.join("docs/STATUS.md"), "# Status").unwrap();
            let a = inspect(root, std::iter::empty());
            assert_eq!(a.documents.len(), 2);
            assert!(a
                .documents
                .iter()
                .any(|d| d.path == "docs/STATUS.md" && d.state == "READABLE"));
            assert!(a
                .documents
                .iter()
                .any(|d| d.path == "docs/CONTINUITY.md" && d.state == "MISSING"));
            assert!(!root.join(".project/WORK.json").exists());
        });
    }

    #[cfg(unix)]
    #[test]
    fn refuses_symlinks_at_each_component_and_special_files() {
        use std::os::unix::fs::symlink;
        with_project(|root| {
            fs::write(root.join("docs/work/original.md"), "# Original").unwrap();
            symlink(
                root.join("docs/work/original.md"),
                root.join("docs/work/file-link.md"),
            )
            .unwrap();
            symlink(root.join("docs/work"), root.join("shortcut")).unwrap();
            let mut reader = Reader::new(root).unwrap();
            assert_eq!(
                reader.read_markdown("docs/work/file-link.md").unwrap_err(),
                ReadFailure::Unsafe
            );
            assert_eq!(
                reader.read_markdown("shortcut/original.md").unwrap_err(),
                ReadFailure::Unsafe
            );
            assert_eq!(
                reader.read_markdown("docs/work").unwrap_err(),
                ReadFailure::Unsupported
            );
        });
    }

    #[cfg(windows)]
    #[test]
    fn windows_junction_to_external_directory_is_refused() {
        use std::process::Command;
        with_project(|root| {
            let external = root.with_extension("external");
            fs::create_dir_all(&external).unwrap();
            fs::write(external.join("outside.md"), "# EXTERNAL_SECRET").unwrap();
            let junction = root.join("junction");
            // PowerShell creates a real NTFS junction without developer mode.
            // Escape PowerShell single-quoted strings even for local fixture paths.
            let link_path = junction.to_string_lossy().replace('\'', "''");
            let target_path = external.to_string_lossy().replace('\'', "''");
            let command = format!(
                "New-Item -ItemType Junction -Path '{link_path}' -Target '{target_path}' -ErrorAction Stop | Out-Null"
            );
            let output = Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &command])
                .output()
                .expect("create junction");
            assert!(
                output.status.success(),
                "junction setup failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let mut reader = Reader::new(root).unwrap();
            assert!(
                reader.read_markdown("junction/outside.md").is_err(),
                "junction cannot open external content"
            );
            assert_eq!(reader.bytes_read, 0);
            fs::remove_dir(&junction).unwrap();
            fs::remove_dir_all(&external).unwrap();
        });
    }

    #[test]
    fn preserves_contract_order_and_context_priority_under_limits() {
        with_project(|root| {
            fs::write(root.join("docs/STATUS.md"), "# Status").unwrap();
            let list = (0..300)
                .rev()
                .map(|id| format!("docs/work/WORK-{id:03}.md"));
            let result = inspect(root, list);
            assert_eq!(result.documents.len(), MAX_DOCUMENTS);
            assert_eq!(result.documents[0].path, "docs/STATUS.md");
            assert_eq!(result.documents[1].path, "docs/CONTINUITY.md");
            assert_eq!(result.documents[2].path, "docs/work/WORK-299.md");
            assert_eq!(result.omitted_documents, 302 - MAX_DOCUMENTS);
        });
    }

    #[test]
    fn document_count_limit_is_observable() {
        with_project(|root| {
            let records = (0..300).map(|i| format!("docs/work/WORK-{i}.md"));
            let result = inspect(root, records);
            assert_eq!(result.documents.len(), MAX_DOCUMENTS);
            assert_eq!(result.omitted_documents, 302 - MAX_DOCUMENTS);
            assert_eq!(result.status, "PARTIAL");
        });
    }
}
