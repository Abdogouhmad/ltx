use crate::commands::resolve_manifest_path;
use crate::ctx::{AppContext, CliCommand};
use clap::Args;
use ltx_compiler::config::CompilerConfig;
use ltx_config::LtxManifest;

/// Arguments for `ltx build`.
///
/// Compiles the project described in `ltx.toml` into a PDF. The manifest is
/// validated first (missing keys, typos, missing input file) and the build is
/// only started once it passes.
#[derive(Debug, Clone, Args)]
pub struct BuildArgs;

impl CliCommand for BuildArgs {
    fn execute(&self, ctx: &AppContext) -> miette::Result<()> {
        let manifest_path = resolve_manifest_path(ctx.manifest_path.as_deref())?;
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

        ltx_compiler::build::build(&config, &project_root)
    }
}
