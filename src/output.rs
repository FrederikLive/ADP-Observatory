use std::{
    env, fs,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
#[cfg(not(target_os = "windows"))]
use std::process::Command;

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
        if existing.starts_with(&project) {
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
    if canonical.starts_with(&project) {
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

/// Return a filename suitable for user-facing messages and for Windows Shell APIs.
/// Rust's canonicalize() adds an extended-path prefix on Windows; Explorer and
/// some associated document applications cannot open that syntax directly.
#[cfg(target_os = "windows")]
fn windows_shell_path(path: &Path) -> Result<PathBuf, String> {
    let text = path.to_string_lossy();
    if let Some(unc) = text.strip_prefix(r"\\?\UNC\") {
        return Ok(PathBuf::from(format!(r"\\{unc}")));
    }
    if let Some(drive) = text.strip_prefix(r"\\?\") {
        let chars = drive.as_bytes();
        if chars.len() >= 3
            && chars[0].is_ascii_alphabetic()
            && chars[1] == b':'
            && chars[2] == b'\\'
        {
            return Ok(PathBuf::from(drive));
        }
        return Err("This Windows report path has no drive or UNC equivalent for the default browser. Open the report from a normal local directory.".into());
    }
    Ok(path.to_path_buf())
}

/// The printed path is for humans and host tools; we retain the canonical
/// path separately for filesystem containment and report creation.
pub fn report_location(path: &Path) -> String {
    #[cfg(target_os = "windows")]
    {
        return windows_shell_path(path)
            .unwrap_or_else(|_| path.to_path_buf())
            .display()
            .to_string();
    }
    #[cfg(not(target_os = "windows"))]
    {
        path.display().to_string()
    }
}

/// Request that the operating system open the HTML document using the
/// user's default association. Success is NOT proof that a browser window
/// became visible in the user's interactive desktop.
#[cfg(target_os = "windows")]
pub fn open_browser(path: &Path) -> Result<(), String> {
    use std::{ffi::c_void, os::windows::ffi::OsStrExt, ptr};
    #[link(name = "shell32")]
    extern "system" {
        fn ShellExecuteW(
            hwnd: *mut c_void,
            operation: *const u16,
            file: *const u16,
            parameters: *const u16,
            directory: *const u16,
            show: i32,
        ) -> *mut c_void;
    }
    let path = windows_shell_path(path)?;
    let document: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let operation: Vec<u16> = "open".encode_utf16().chain(std::iter::once(0)).collect();
    // SAFETY: Strings are NUL-terminated and live for the entire call; all
    // optional pointers are NULL, and ShellExecuteW only reads these inputs.
    let code = unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            operation.as_ptr(),
            document.as_ptr(),
            ptr::null(),
            ptr::null(),
            1,
        )
    } as isize;
    if code > 32 {
        Ok(())
    } else {
        Err(format!(
            "Windows ShellExecuteW could not request opening the report (code {code}). Check your default .html association or open the printed report path manually."
        ))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn open_browser(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let mut command = Command::new("open");
    #[cfg(not(target_os = "macos"))]
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

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_extended_drive_report_path_is_shell_compatible() {
        let path = Path::new(r"\\?\C:\Users\user\ADP Observatory\report ø.html");
        let normal = windows_shell_path(path).expect("supported drive path");
        assert_eq!(normal, PathBuf::from(r"C:\Users\user\ADP Observatory\report ø.html"));
        assert_eq!(report_location(path), r"C:\Users\user\ADP Observatory\report ø.html");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_extended_unc_report_path_is_shell_compatible() {
        let path = Path::new(r"\\?\UNC\server\share\report.html");
        let normal = windows_shell_path(path).expect("supported UNC path");
        assert_eq!(normal, PathBuf::from(r"\\server\share\report.html"));
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_unsupported_shell_namespace_fails_closed() {
        assert!(windows_shell_path(Path::new(r"\\?\Volume{1234}\report.html")).is_err());
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
