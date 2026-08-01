use crate::ctx::{AppContext, CliCommand};
use clap::Args;
use ltx_compiler::config::CompilerConfig;
use ltx_config::LtxManifest;
use std::path::{Path, PathBuf};

/// Arguments for `ltx build`.
///
/// Compiles the project described in `ltx.toml` into a PDF. The input file
/// comes from `[project].main` and the output name from `[build].name`.
#[derive(Debug, Clone, Args)]
pub struct BuildArgs;

impl CliCommand for BuildArgs {
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

        let manifest = LtxManifest::from_file(&manifest_path)
            .map_err(|e| miette::miette!("failed to read `{}`: {e}", manifest_path.display()))?;

        let config =
            CompilerConfig::from_manifest(&manifest).map_err(|e| miette::miette!("{e}"))?;

        ltx_compiler::build::build(&config, &project_root)
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
