//! Low-level filesystem helpers for LTX.

use std::path::{Path, PathBuf};
use std::{fs, io};

/// Creates a directory at `path`, including all missing parents.
///
/// # Errors
///
/// Returns an error if the directory cannot be created.
pub fn create_dir(path: &Path) -> std::io::Result<()> {
    fs::create_dir_all(path)
}

/// Creates a file at `path`, including all missing parent directories.
///
/// # Errors
///
/// Returns an error if the file or its parent directories cannot be created.
pub fn create_file(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(path)?;
    Ok(())
}

/// Writes `contents` to the file at `path`, creating parent dirs as needed.
///
/// # Errors
///
/// Returns an error if the file or its parent directories cannot be created,
/// or if writing fails.
pub fn write_file(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)
}

/// Returns the path to the main entry file.
///
/// If no path is provided, returns `main.tex`.
/// Otherwise, returns the user-provided path if it exists.
///
/// # Errors
///
/// Returns [`io::ErrorKind::NotFound`] if the specified path does not exist.
pub fn resolve_main_file(path: Option<impl AsRef<Path>>) -> io::Result<PathBuf> {
    let path = path
        .as_ref()
        .map(|p| p.as_ref().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("main.tex"));

    if path.exists() {
        Ok(path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("file '{}' does not exist", path.display()),
        ))
    }
}
