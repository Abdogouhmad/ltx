//! Low-level filesystem helpers for LTX.

use std::fs;
use std::path::Path;

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
