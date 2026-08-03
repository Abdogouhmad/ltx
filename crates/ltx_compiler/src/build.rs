//! Build entry point — dispatches to the compiler engine selected in `ltx.toml`.

use std::path::Path;

use ltx_config::CompilerEngine;
use miette::Result as MResult;

use crate::config::CompilerConfig;
use crate::error::CompilerError;
use crate::tectonic::tectonic_compile;

/// Emits an `LTX::COMPILER::W001` warning for an engine that is not wired up
/// yet. The build then proceeds with `Ok(())` so it does not fail on a warning.
fn engine_not_implemented(engine: &'static str) {
    eprintln!(
        "{:?}",
        miette::Report::new(CompilerError::EngineNotImplemented { engine })
    );
}

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
/// Returns [`CompilerError::MainFileNotFound`] if the main input file does
/// not exist, or [`CompilerError::TectonicError`] if the engine fails to
/// compile it. Unimplemented engines produce a [`CompilerError::EngineNotImplemented`]
/// warning (`LTX::COMPILER::W001`).
pub fn build(config: &CompilerConfig, project_root: &Path) -> MResult<()> {
    let main_path = project_root.join(config.main_file());

    if !main_path.exists() {
        return Err(CompilerError::MainFileNotFound { path: main_path }.into());
    }

    match config.engine {
        CompilerEngine::Tectonic => tectonic_compile(
            &main_path,
            config.output_name(),
            &project_root.join("target"),
            &config.compile_options,
        ),
        CompilerEngine::PdfLaTeX => {
            engine_not_implemented("pdflatex");
            Ok(())
        }
        CompilerEngine::XeLaTeX => {
            engine_not_implemented("xelatex");
            Ok(())
        }
        CompilerEngine::LuaLaTeX => {
            engine_not_implemented("lualatex");
            Ok(())
        }
    }
}
