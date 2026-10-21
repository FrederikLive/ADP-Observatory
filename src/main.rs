mod work;

use serde::Serialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

#[derive(Serialize)]
struct Report {
    schema_version: &'static str,
    analyzer_version: &'static str,
    target: String,
    scan_complete: bool,
    files_examined: usize,
    files_skipped: usize,
    adp_markers: Vec<String>,
    work_index: work::WorkInspection,
    limitations: Vec<String>,
}

fn main() {
    let mut args = env::args_os().skip(1);
    let root = match args.next() {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!("Usage: adp-observatory <project-directory> [--json]");
            process::exit(2);
        }
    };
    let json = args.any(|arg| arg == "--json");
    let root = match fs::canonicalize(&root) {
        Ok(path) if path.is_dir() => path,
        _ => {
            eprintln!("Error: project path must exist and be a directory.");
            process::exit(2);
        }
    };

    let mut markers = Vec::new();
    let mut examined = 0usize;
    let mut skipped = 0usize;
    let mut complete = true;
    scan(
        &root,
        &root,
        0,
        &mut examined,
        &mut skipped,
        &mut markers,
        &mut complete,
    );

    let report = Report {
        schema_version: "0.1.0",
        analyzer_version: env!("CARGO_PKG_VERSION"),
        target: root.display().to_string(),
        scan_complete: complete,
        files_examined: examined,
        files_skipped: skipped,
        adp_markers: markers,
        work_index: work::inspect(&root),
        limitations: vec![
            "Initial structural work-index inspection only; this is not a full ADP conformance assessment.".into(),
            "No project scripts or commands are executed.".into(),
            "Dashboard, metric calculations, and historical snapshots are not implemented.".into(),
        ],
    };

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".into())
        );
    } else {
        println!("ADP-Observatory {}", report.analyzer_version);
        println!("Target: {}", report.target);
        println!("Scan complete: {}", report.scan_complete);
        println!(
            "Files examined: {}; skipped: {}",
            report.files_examined, report.files_skipped
        );
        println!("ADP markers:");
        for marker in &report.adp_markers {
            println!("  - {marker}");
        }
        println!("Work index: {}", report.work_index.status);
        for finding in &report.work_index.findings {
            println!("  [{}] {}: {}", finding.severity, finding.code, finding.message);
        }
        println!("No full ADP conformance verdict is produced.");
        for item in &report.limitations {
            println!("- {item}");
        }
    }
}

fn scan(
    root: &Path,
    dir: &Path,
    depth: usize,
    examined: &mut usize,
    skipped: &mut usize,
    markers: &mut Vec<String>,
    complete: &mut bool,
) {
    if depth > 8 || *examined + *skipped >= 20_000 {
        *complete = false;
        *skipped += 1;
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => {
            *complete = false;
            *skipped += 1;
            return;
        }
    };

    for item in entries {
        if *examined + *skipped >= 20_000 {
            *complete = false;
            *skipped += 1;
            break;
        }

        let entry = match item {
            Ok(entry) => entry,
            Err(_) => {
                *complete = false;
                *skipped += 1;
                continue;
            }
        };
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name == ".git" || name == "target" || name == "node_modules" || name == "vendor" {
            *skipped += 1;
            continue;
        }

        let meta = match fs::symlink_metadata(&path) {
            Ok(meta) => meta,
            Err(_) => {
                *complete = false;
                *skipped += 1;
                continue;
            }
        };

        if meta.file_type().is_symlink() {
            *complete = false;
            *skipped += 1;
            continue;
        }
        if meta.is_dir() {
            scan(root, &path, depth + 1, examined, skipped, markers, complete);
            continue;
        }
        if !meta.is_file() || meta.len() > 2_097_152 {
            *complete = false;
            *skipped += 1;
            continue;
        }

        *examined += 1;
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
                | "docs/CONFORMANCE.md"
        ) {
            markers.push(relative);
        }
    }
}
