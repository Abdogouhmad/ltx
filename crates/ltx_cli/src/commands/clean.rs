use crate::ctx::{AppContext, CliCommand};
use clap::Args;
use owo_colors::OwoColorize;
use std::path::Path;
use std::time::Instant;
use std::{fs, io};

/// Arguments for `ltx clean`.
///
/// Removes the `target/` directory and prints a summary of what was deleted,
/// akin to `cargo clean`. Takes no arguments of its own.
#[derive(Debug, Clone, Args)]
pub struct CleanArgs;

struct DirStats {
    files: u64,
    bytes: u64,
}

impl CliCommand for CleanArgs {
    fn execute(&self, ctx: &AppContext) -> miette::Result<()> {
        let project_root = ctx
            .manifest_path
            .as_deref()
            .and_then(|p| p.parent())
            .unwrap_or_else(|| Path::new("."));

        let target_dir = project_root.join("target");

        if !target_dir.exists() {
            println!();
            println!("{}: {}", "[INFO]".cyan().bold(), "Nothing to clean.");
            return Ok(());
        }

        let start = Instant::now();

        let stats = dir_stats(&target_dir).map_err(|e| miette::miette!("{e}"))?;

        if ctx.verbose >= 2 {
            println!("Files scheduled for removal:");
            print_tree(&target_dir).map_err(|e| miette::miette!("{e}"))?;
            println!();
        }

        let directory = target_dir.canonicalize().unwrap_or(target_dir);

        fs::remove_dir_all(&directory).map_err(|e| miette::miette!("{e}"))?;

        let elapsed = start.elapsed();

        match ctx.verbose {
            0 => {
                println!(
                    "Removed {} files, {} total ({})",
                    stats.files,
                    format_size(stats.bytes),
                    format_duration(elapsed),
                );
            }
            _ => {
                println!("Clean completed successfully.");
                println!("  Directory : {}", directory.display());
                println!("  Files     : {}", stats.files);
                println!("  Size      : {}", format_size(stats.bytes));
                println!("  Duration  : {}", format_duration(elapsed));
            }
        }

        Ok(())
    }
}

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

fn print_tree(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            print_tree(&entry.path())?;
        } else {
            println!(
                "  {} ({})",
                entry.path().display(),
                format_size(metadata.len())
            );
        }
    }

    Ok(())
}

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

fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs_f64();
    format!("{secs:.2}s")
}
