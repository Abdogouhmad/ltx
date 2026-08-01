//! Build entry point — dispatches to the compiler engine selected in `ltx.toml`.

use std::path::Path;

use ltx_config::CompilerEngine;
use miette::Result as MResult;

use crate::config::CompilerConfig;
use crate::tectonic::tectonic_compile;

/// Compiles the project configured in [`CompilerConfig`].
///
/// Resolves the main input file relative to the project root, then invokes
/// the engine chosen in the `[build]` section of `ltx.toml`. Currently only
/// the `tectonic` engine is implemented.
///
/// # Arguments
///
/// * `config` - The resolved compiler configuration.
/// * `project_root` - Directory containing `ltx.toml`.
///
/// # Errors
///
/// Returns an error if the main input file does not exist or if the engine
/// fails to compile it.
pub fn build(config: &CompilerConfig, project_root: &Path) -> MResult<()> {
    let main_path = project_root.join(config.main_file());

    if !main_path.exists() {
        return Err(miette::miette!(
            "main file `{}` not found — check the `main` entry in ltx.toml",
            main_path.display()
        ));
    }

    match config.engine {
        CompilerEngine::Tectonic => tectonic_compile(
            &main_path,
            config.output_name(),
            &project_root.join("target"),
        ),
        CompilerEngine::PdfLaTeX => {
            println!("`pdflatex` engine is not implemented yet — only `tectonic` is available");
            Ok(())
        }
        CompilerEngine::XeLaTeX => {
            println!("`xelatex` engine is not implemented yet — only `tectonic` is available");
            Ok(())
        }
        CompilerEngine::LuaLaTeX => {
            println!("`lualatex` engine is not implemented yet — only `tectonic` is available");
            Ok(())
        }
    }
}
