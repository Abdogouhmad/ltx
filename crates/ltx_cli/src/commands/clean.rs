use std::{fs, io, path::Path};

/// Removes the build directory and prints the reclaimed disk space.
pub fn clean_build() -> io::Result<()> {
    let build_dir = Path::new("build");

    if !build_dir.exists() {
        println!("Nothing to clean.");
        return Ok(());
    }

    let size = dir_size(build_dir)?;

    fs::remove_dir_all(build_dir)?;

    println!("Removed build directory ({})", format_size(size),);

    Ok(())
}

/// Recursively computes the size of a directory.
fn dir_size(path: &Path) -> io::Result<u64> {
    let mut size = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            size += dir_size(&entry.path())?;
        } else {
            size += metadata.len();
        }
    }

    Ok(size)
}

/// Formats bytes into a human-readable string.
fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{bytes} {}", UNITS[unit])
    } else {
        format!("{size:.1} {}", UNITS[unit])
    }
}
