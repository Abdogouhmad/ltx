use std::path::Path;
use std::time::Instant;
use std::{fs, io};

struct DirStats {
    files: u64,
    bytes: u64,
}

/// Removes the build directory and prints a summary akin to `cargo clean`.
///
/// Output format: `Removed {files} files, {size} total ({duration})`
pub fn clean_build() -> io::Result<()> {
    let build_dir = Path::new("build");

    if !build_dir.exists() {
        println!("Nothing to clean.");
        return Ok(());
    }

    let start = Instant::now();
    let stats = dir_stats(build_dir)?;

    fs::remove_dir_all(build_dir)?;

    let elapsed = start.elapsed();
    println!(
        "Removed {} files, {} total ({})",
        stats.files,
        format_size(stats.bytes),
        format_duration(elapsed),
    );

    Ok(())
}

/// Recursively counts files and accumulates total bytes under `path`.
fn dir_stats(path: &Path) -> io::Result<DirStats> {
    let mut files = 0;
    let mut bytes = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            let sub = dir_stats(&entry.path())?;
            files += sub.files;
            bytes += sub.bytes;
        } else {
            files += 1;
            bytes += metadata.len();
        }
    }

    Ok(DirStats { files, bytes })
}

/// Formats bytes into a human-readable string (B, KiB, MiB, GiB, TiB).
fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];

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

/// Formats a [`Duration`] in a concise human form (e.g. `0.02s`, `5.10s`).
fn format_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs_f64();
    format!("{secs:.2}s")
}
