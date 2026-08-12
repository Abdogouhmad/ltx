use crate::ctx::{AppContext, CliCommand};
use crate::error::CliError;
use clap::Args;
use ltx_config::LtxManifest;
use ltx_linter::{lint_file, lint_project};
use std::path::{Path, PathBuf};

/// Arguments for `ltx check`.
///
/// Runs the full lex → parse → lint → diagnostics pipeline on `.tex` files
/// and reports syntax errors, style lints and warnings without producing any
/// output files.
///
/// With no arguments every `.tex` file under the project is checked
/// recursively (like `cargo check`); use `-p` to check a single file.
///
/// Exit codes:
///   0 — no issues found (or only warnings)
///   4 — one or more errors found
#[derive(Args, Debug, Clone)]
pub struct CheckArgs {
    /// Path to a single `.tex` file to lint and parse.
    ///
    /// If omitted, every `.tex` file under the project is checked recursively.
    #[arg(short = 'p', long)]
    pub path: Option<PathBuf>,

    /// Skip the style linter; only lex and parse.
    #[arg(long)]
    pub no_lint: bool,
}

impl CliCommand for CheckArgs {
    fn execute(&self, ctx: &AppContext) -> miette::Result<()> {
        let manifest_path = ctx
            .manifest_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("ltx.toml"));

        // With no `-p`, check every `.tex` file under the project.
        let Some(path) = &self.path else {
            return self.check_all(&manifest_path);
        };

        // A single file: `[lints]` from the manifest (if any) still applies.
        let lints = if manifest_path.is_file() {
            LtxManifest::from_file(&manifest_path)?.lints
        } else {
            None
        };

        let result = lint_file(path, lints.as_ref(), !self.no_lint)?;

        if !result.is_empty() {
            eprintln!(
                "{}",
                result
                    .render_pretty()
                    .map_err(|e| miette::miette!("Failed to render diagnostics: {e}"))?
            );
        }

        let error_count = result.error_count();
        let warning_count = result.warning_count();

        if error_count > 0 {
            eprintln!("Check failed with {error_count} error(s).");
            Err(CliError::DiagnosticsFound.into())
        } else if warning_count > 0 {
            eprintln!("Check passed with {warning_count} warning(s).");
            Ok(())
        } else {
            eprintln!("Check passed — no issues found.");
            Ok(())
        }
    }
}

impl CheckArgs {
    /// Runs `ltx check` over every `.tex` file under the project, recursively.
    ///
    /// The scan root is the directory containing `ltx.toml`, or the current
    /// directory when no manifest exists. Generated `target/` output is
    /// skipped, mirroring `cargo check`.
    fn check_all(&self, manifest_path: &Path) -> miette::Result<()> {
        let (root, lint_table) = if manifest_path.is_file() {
            let manifest = LtxManifest::from_file(manifest_path).map_err(miette::Report::from)?;
            (scan_root(manifest_path), manifest.lints)
        } else {
            let cwd = std::env::current_dir()
                .map_err(|e| miette::miette!("cannot determine the current directory: {e}"))?;
            (cwd, None)
        };

        let mut files = Vec::new();
        collect_tex_files(&root, &mut files)?;
        files.sort();

        if files.is_empty() {
            return Err(miette::miette!(
                "no `.tex` files found under `{}`",
                root.display()
            ));
        }

        let result = lint_project(&files, lint_table.as_ref(), !self.no_lint)?;
        if !result.is_empty() {
            eprintln!(
                "{}",
                result
                    .render_pretty()
                    .map_err(|e| { miette::miette!("Failed to render diagnostics: {e}") })?
            );
        }
        let total_errors = result.error_count();
        let total_warnings = result.warning_count();

        if total_errors > 0 {
            eprintln!(
                "Check failed with {total_errors} error(s) across {} file(s).",
                files.len()
            );
            Err(CliError::DiagnosticsFound.into())
        } else if total_warnings > 0 {
            eprintln!(
                "Check passed with {total_warnings} warning(s) across {} file(s).",
                files.len()
            );
            Ok(())
        } else {
            eprintln!(
                "Check passed — no issues found across {} file(s).",
                files.len()
            );
            Ok(())
        }
    }
}

/// Recursively collects every `*.tex` file under `dir`, skipping `target/`
/// and `.git/` directories.
fn collect_tex_files(dir: &Path, out: &mut Vec<PathBuf>) -> miette::Result<()> {
    for entry in std::fs::read_dir(dir)
        .map_err(|e| miette::miette!("failed to scan `{}`: {e}", dir.display()))?
    {
        let entry =
            entry.map_err(|e| miette::miette!("failed to scan `{}`: {e}", dir.display()))?;

        let path = entry.path();

        if path.is_dir() {
            let name = entry.file_name();
            if name == "target" || name == ".git" {
                continue;
            }
            collect_tex_files(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "tex") {
            out.push(path);
        }
    }

    Ok(())
}

/// The directory a recursive scan roots from: the directory containing the
/// manifest, or `.` when the manifest path is relative with no parent.
fn scan_root(manifest_path: &Path) -> PathBuf {
    manifest_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::scan_root;
    use std::path::Path;

    #[test]
    fn scan_root_uses_manifest_directory() {
        assert_eq!(scan_root(Path::new("proj/ltx.toml")), Path::new("proj"));
    }

    #[test]
    fn scan_root_falls_back_to_dot_for_relative_manifest() {
        assert_eq!(scan_root(Path::new("ltx.toml")), Path::new("."));
    }
}
