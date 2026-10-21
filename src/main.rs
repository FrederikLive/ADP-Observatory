mod compatibility;
mod discovery;
mod records;
mod references;
mod work;

use serde::Serialize;
use std::{env, fs, path::PathBuf, process};

#[derive(Serialize)]
struct Report {
    schema_version: &'static str,
    analyzer_version: &'static str,
    target: String,
    scan_complete: bool,
    files_examined: usize,
    files_skipped: usize,
    scan_scope: discovery::ScanScope,
    adp_markers: Vec<String>,
    work_index: work::WorkInspection,
    compatibility: compatibility::Compatibility,
    context: records::ContextInspection,
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

    let discovery = discovery::Discovery::inspect(&root);
    let work_index = work::inspect(&root);
    let compatibility = compatibility::inspect(&root, &work_index);
    let context = records::inspect(&root, &work_index);

    let report = Report {
        schema_version: "0.3.0",
        analyzer_version: env!("CARGO_PKG_VERSION"),
        target: root.display().to_string(),
        scan_complete: discovery.scope.status != "PARTIAL",
        files_examined: discovery.files_examined,
        files_skipped: discovery.files_skipped,
        scan_scope: discovery.scope,
        adp_markers: discovery.markers,
        work_index,
        compatibility,
        context,
        limitations: vec![
            "Structural and contextual inspection only; this is not a full ADP conformance assessment.".into(),
            "scan_complete means traversal completed within policy; intentionally excluded files and independent parse errors are reported separately.".into(),
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
        println!("Scan scope: {}", report.scan_scope.status);
        println!(
            "Files examined: {}; skipped: {}",
            report.files_examined, report.files_skipped
        );
        println!("ADP markers:");
        for marker in &report.adp_markers {
            println!("  - {marker}");
        }
        println!("Work index: {}", report.work_index.status);
        println!("Semantic scope: {}", report.compatibility.semantic_scope);
        for reference in &report.work_index.contract_files {
            println!(
                "Contract file [{}]: {} -> {}",
                reference.contract_id, reference.path, reference.status
            );
        }
        println!(
            "Compatibility: format={}, semantics={}, conformance={}",
            report.compatibility.format,
            report.compatibility.semantics,
            report.compatibility.conformance
        );
        println!(
            "Protocol version: {}",
            report
                .compatibility
                .protocol
                .value
                .as_deref()
                .unwrap_or(report.compatibility.protocol.status)
        );
        for conflict in &report.context.conflicts {
            println!("Conflict in {}:", conflict.field);
            for claim in &conflict.claims {
                println!("  {}: {}", claim.path, claim.value);
            }
        }
        for finding in &report.work_index.findings {
            println!(
                "  [{}] {}: {}",
                finding.severity, finding.code, finding.message
            );
        }
        println!("No full ADP conformance verdict is produced.");
        for item in &report.limitations {
            println!("- {item}");
        }
    }
}
