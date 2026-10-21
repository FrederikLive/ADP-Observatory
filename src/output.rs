use std::{
    env, fs,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn cache_directory() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(folder) = env::var_os("LOCALAPPDATA") {
            return PathBuf::from(folder)
                .join("ADP-Observatory")
                .join("reports");
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(folder) = env::var_os("HOME") {
            return PathBuf::from(folder).join("Library/Caches/ADP-Observatory");
        }
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Some(folder) = env::var_os("XDG_CACHE_HOME") {
            return PathBuf::from(folder).join("adp-observatory");
        }
        if let Some(folder) = env::var_os("HOME") {
            return PathBuf::from(folder).join(".cache/adp-observatory");
        }
    }
    env::temp_dir().join("adp-observatory")
}

fn ensure_directory(directory: &Path, project: &Path, create: bool) -> Result<PathBuf, String> {
    // Compare canonical paths on every platform. /var on macOS and Windows
    // short-name aliases otherwise make a simple starts_with() unreliable.
    let project = fs::canonicalize(project)
        .map_err(|e| format!("Cannot resolve analyzed project directory: {e}"))?;
    if create {
        // Refuse an in-project cache root before creating any directories.
        let mut ancestor = directory;
        while !ancestor.exists() {
            ancestor = ancestor
                .parent()
                .ok_or("No existing report-directory ancestor")?;
        }
        let existing = fs::canonicalize(ancestor)
            .map_err(|e| format!("Cannot resolve report-directory ancestor: {e}"))?;
        if existing.starts_with(project) {
            return Err("Reports must be stored outside the analyzed project".into());
        }
        fs::create_dir_all(directory)
            .map_err(|e| format!("Cannot create report directory: {e}"))?;
    }
    let metadata = fs::symlink_metadata(directory)
        .map_err(|e| format!("Cannot inspect report directory: {e}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err("Report directory must be an actual directory, not a symbolic link".into());
    }
    let canonical =
        fs::canonicalize(directory).map_err(|e| format!("Cannot resolve report directory: {e}"))?;
    if canonical.starts_with(project) {
        return Err("Reports must be saved outside the analyzed project".into());
    }
    Ok(canonical)
}

fn write_new(path: &Path, html: &str) -> Result<(), String> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|e| format!("Cannot create report file: {e}"))?;
    if let Err(error) = file.write_all(html.as_bytes()) {
        drop(file);
        let _ = fs::remove_file(path);
        return Err(format!("Cannot write report: {error}"));
    }
    Ok(())
}

/// Saves only a derived report, never writing into the target project.
pub fn save(project: &Path, requested: Option<&Path>, html: &str) -> Result<PathBuf, String> {
    if let Some(path) = requested {
        let file_name = path.file_name().ok_or("Output must be a file path")?;
        if !path.to_string_lossy().to_lowercase().ends_with(".html") {
            return Err("HTML report output must use a .html extension".into());
        }
        let directory = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let parent = ensure_directory(directory, project, false)?;
        let destination = parent.join(file_name);
        write_new(&destination, html)?;
        return Ok(destination);
    }

    let directory = ensure_directory(&cache_directory(), project, true)?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Cannot get time for report filename: {e}"))?
        .as_nanos();
    for attempt in 0..12 {
        let file_name = format!("report-{}-{now}-{attempt}.html", std::process::id());
        let destination = directory.join(file_name);
        match write_new(&destination, html) {
            Ok(()) => return Ok(destination),
            Err(e) => {
                let exists = fs::symlink_metadata(&destination).is_ok();
                if !exists {
                    return Err(e);
                }
            }
        }
    }
    Err("Could not allocate a unique local report filename".into())
}

pub fn open_browser(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let mut command = Command::new("explorer.exe");
    #[cfg(target_os = "macos")]
    let mut command = Command::new("open");
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let mut command = Command::new("xdg-open");

    let status = command
        .arg(path)
        .status()
        .map_err(|e| format!("Could not start the system browser opener: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("System browser opener returned status {status}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn roots() -> (PathBuf, PathBuf) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let base = env::temp_dir().join(format!(
            "adp-observatory-output-{}-{nonce}",
            std::process::id()
        ));
        let project = base.join("project");
        let destination = base.join("report");
        fs::create_dir_all(&project).expect("project");
        fs::create_dir_all(&destination).expect("report");
        (project, destination)
    }

    #[test]
    fn rejects_writing_inside_analyzed_project() {
        let (project, outside) = roots();
        let report = project.join("scan.html");
        assert!(save(&project, Some(&report), "content").is_err());
        assert!(!report.exists());
        fs::remove_dir_all(project.parent().unwrap()).expect("cleanup");
        let _ = outside;
    }

    #[test]
    fn saves_report_only_to_explicit_outside_destination() {
        let (project, outside) = roots();
        let destination = outside.join("scan.html");
        let saved = save(&project, Some(&destination), "offline html").expect("save");
        assert_eq!(
            saved,
            fs::canonicalize(&destination).expect("canonical destination")
        );
        assert_eq!(fs::read_to_string(&saved).expect("read"), "offline html");
        assert!(save(&project, Some(&destination), "overwrite").is_err());
        fs::remove_dir_all(project.parent().unwrap()).expect("cleanup");
    }
}
