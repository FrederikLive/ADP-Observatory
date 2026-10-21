use std::{env, ffi::OsString, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Dashboard { open: bool },
    HtmlOnly,
    Json,
    Text,
    Help,
    Version,
}

#[derive(Debug)]
pub struct Request {
    pub mode: Mode,
    pub target: PathBuf,
    pub output: Option<PathBuf>,
}

pub const USAGE: &str = r#"ADP-Observatory — local, read-only ADP project viewer

USAGE:
  adp-observatory [PROJECT] [--no-open] [--output REPORT.html]
  adp-observatory dashboard [PROJECT] [--no-open] [--output REPORT.html]
  adp-observatory report [PROJECT] --output REPORT.html
  adp-observatory scan [PROJECT] [--json]
  adp-observatory [PROJECT] --json          Legacy JSON shortcut
  adp-observatory --version
  adp-observatory --help

PROJECT defaults to "." (the current directory).
Dashboard mode generates a standalone local HTML report outside the project
and opens it in your browser when possible. --no-open only writes it.
A report's --output directory must exist, and the file must not already exist.
No target-project scripts or agent instructions are executed.
"#;

pub fn parse() -> Result<Request, String> {
    parse_args(env::args_os().skip(1).collect())
}

fn parse_args(args: Vec<OsString>) -> Result<Request, String> {
    if args.len() == 1 {
        if args[0] == "--help" || args[0] == "-h" {
            return Ok(Request {
                mode: Mode::Help,
                target: PathBuf::from("."),
                output: None,
            });
        }
        if args[0] == "--version" || args[0] == "-V" {
            return Ok(Request {
                mode: Mode::Version,
                target: PathBuf::from("."),
                output: None,
            });
        }
    }
    let mut items = args.into_iter().peekable();
    let mut verb = "dashboard";
    if let Some(head) = items.peek() {
        if head == "dashboard" {
            verb = "dashboard";
            items.next();
        } else if head == "scan" {
            verb = "scan";
            items.next();
        } else if head == "report" {
            verb = "report";
            items.next();
        }
    }

    let mut project = None;
    let mut output = None;
    let mut json = false;
    let mut no_open = false;
    while let Some(arg) = items.next() {
        if arg == "--json" {
            json = true;
        } else if arg == "--no-open" {
            no_open = true;
        } else if arg == "--output" {
            let next = items.next().ok_or("--output requires a path")?;
            if output.replace(PathBuf::from(next)).is_some() {
                return Err("--output specified more than once".into());
            }
        } else if arg.to_string_lossy().starts_with('-') {
            return Err(format!("Unknown option: {}", arg.to_string_lossy()));
        } else if project.replace(PathBuf::from(arg)).is_some() {
            return Err("Specify only one project directory".into());
        }
    }

    if json && (verb == "report" || no_open || output.is_some()) {
        return Err("--json is only available in scan mode or legacy PROJECT --json".into());
    }
    if no_open && verb == "scan" {
        return Err("--no-open only applies to dashboard mode".into());
    }
    if output.is_some() && verb == "scan" {
        return Err("--output only applies to dashboard or report mode".into());
    }
    if verb == "report" && output.is_none() {
        return Err("report mode requires --output REPORT.html".into());
    }
    let mode = match verb {
        "scan" => {
            if json {
                Mode::Json
            } else {
                Mode::Text
            }
        }
        "report" => Mode::HtmlOnly,
        _ => {
            if json {
                Mode::Json
            } else {
                Mode::Dashboard { open: !no_open }
            }
        }
    };
    Ok(Request {
        mode,
        target: project.unwrap_or_else(|| PathBuf::from(".")),
        output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from(args: &[&str]) -> Request {
        parse_args(args.iter().map(OsString::from).collect()).expect("valid CLI")
    }

    #[test]
    fn project_path_opens_dashboard() {
        assert_eq!(from(&["."]).mode, Mode::Dashboard { open: true });
        assert_eq!(from(&[]).target, PathBuf::from("."));
    }

    #[test]
    fn explicit_scan_is_headless() {
        assert_eq!(from(&["scan", ".", "--json"]).mode, Mode::Json);
        assert_eq!(from(&["scan", "."]).mode, Mode::Text);
    }

    #[test]
    fn backwards_compatible_json_shortcut() {
        assert_eq!(from(&[".", "--json"]).mode, Mode::Json);
    }

    #[test]
    fn html_mode_can_skip_browser() {
        let request = from(&["dashboard", ".", "--no-open"]);
        assert_eq!(request.mode, Mode::Dashboard { open: false });
    }

    #[test]
    fn reject_invalid_option_combinations() {
        for args in [
            vec!["report", "."],
            vec!["scan", ".", "--no-open"],
            vec!["scan", ".", "--output", "file.html"],
            vec![".", "--json", "--output", "file.html"],
            vec![".", "--bad"],
            vec![".", "another-project"],
        ] {
            assert!(parse_args(args.into_iter().map(OsString::from).collect()).is_err());
        }
    }
}
