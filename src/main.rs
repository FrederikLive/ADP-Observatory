mod cli;
mod compatibility;
mod dashboard;
mod discovery;
mod output;
mod records;
mod references;
mod work;

use serde::Serialize;
use std::{fs, process};

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
    if let Err(message) = run() {
        eprintln!("ADP-Observatory: {message}\n\n{}", cli::USAGE);
        process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let request = cli::parse()?;
    if request.mode == cli::Mode::Help {
        print!("{}", cli::USAGE);
        return Ok(());
    }
    if request.mode == cli::Mode::Version {
        println!("ADP-Observatory {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let root = match fs::canonicalize(&request.target) {
        Ok(path) if path.is_dir() => path,
        _ => return Err("Project path must exist and be a directory".into()),
    };
    let discovery = discovery::Discovery::inspect(&root);
    let work_index = work::inspect(&root);
    let compatibility = compatibility::inspect(&root, &work_index);
    let context = records::inspect(&root, &work_index);
    let report = Report {
        schema_version: "0.4.0",
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
            "Historical snapshots and complete verification metrics are not implemented.".into(),
        ],
    };
    let auto_open = matches!(&request.mode, cli::Mode::Dashboard { open: true });
    match request.mode {
        cli::Mode::Json => {
            let json = serde_json::to_string_pretty(&report)
                .map_err(|error| format!("Could not serialize scan report: {error}"))?;
            println!("{json}");
        }
        cli::Mode::Text => print_text(&report),
        cli::Mode::Dashboard { .. } | cli::Mode::HtmlOnly => {
            let data = serde_json::to_value(&report)
                .map_err(|error| format!("Could not serialize dashboard inputs: {error}"))?;
            let html = dashboard::render(&data);
            let destination = output::save(&root, request.output.as_deref(), &html)?;
            println!("Local ADP report: {}", destination.display());
            println!("Analysis is read-only. ADP conformance has not been assessed.");
            if auto_open {
                if let Err(error) = output::open_browser(&destination) {
                    eprintln!("Browser could not be opened automatically: {error}");
                    eprintln!(
                        "Open the report file above manually. The report was saved successfully."
                    );
                }
            }
        }
        cli::Mode::Help | cli::Mode::Version => unreachable!("Handled before analyzing project"),
    }
    Ok(())
}

fn print_text(report: &Report) {
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
