use crate::ctx::{AppContext, CliCommand};
use clap::Args;
use ltx_compiler::config::CompilerConfig;
use ltx_compiler::watch::WatchConfig;
use ltx_config::LtxManifest;
use std::path::{Path, PathBuf};

/// Arguments for `ltx watch`.
///
/// Watches the project for changes and rebuilds the PDF on every relevant
/// file save, giving a tight write–compile–preview loop. The manifest is
/// validated first, and an initial compile runs on startup so the user gets
/// immediate feedback.
#[derive(Debug, Clone, Args)]
pub struct WatchArgs;

impl CliCommand for WatchArgs {
    fn execute(&self, ctx: &AppContext) -> miette::Result<()> {
        let manifest_path = resolve_manifest(ctx.manifest_path.as_deref())?;
        let project_root = manifest_path
            .parent()
            .ok_or_else(|| {
                miette::miette!(
                    "cannot resolve the directory of `{}`",
                    manifest_path.display()
                )
            })?
            .to_path_buf();

        let manifest = LtxManifest::from_file(&manifest_path)?;
        let config = CompilerConfig::from_manifest(&manifest)?;

        // Watch targets (`src/` or the main file) are derived from the
        // manifest inside `WatchConfig`, never the compiler's output dir.
        WatchConfig::new(project_root, manifest_path, config)
            .run_watch()
            .map_err(Into::into)
    }
}

/// Resolves the manifest file, defaulting to `ltx.toml` in the current
/// directory when `--manifest-path` is not given.
fn resolve_manifest(manifest_path: Option<&Path>) -> miette::Result<PathBuf> {
    let path = manifest_path.unwrap_or_else(|| Path::new("ltx.toml"));
    if !path.exists() {
        return Err(miette::miette!(
            "no ltx.toml found at `{}` — run `ltx new` to scaffold a project",
            path.display()
        ));
    }
    Ok(path.to_path_buf())
}
