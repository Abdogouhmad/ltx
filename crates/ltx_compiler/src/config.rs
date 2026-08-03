//! Configuration for the Ltx compiler.
use ltx_config::{CompileOptions, LtxManifest, engine::CompilerEngine};

use crate::error::CompilerError;

/// Configuration for the Ltx compiler.
pub struct CompilerConfig {
    /// The compiler engine to use.
    pub engine: CompilerEngine,
    /// Additional arguments to pass to the compiler.
    pub engine_args: Vec<String>,
    /// The name of the output file (without extension).
    pub output_name: String,
    /// Path to the main `.tex` input file, relative to the project root.
    pub main_file: String,
    /// Compilation options (logs, intermediates, `SyncTeX`, offline mode).
    pub compile_options: CompileOptions,
}

impl CompilerConfig {
    /// Creates a new [`CompilerConfig`] from a [`LtxManifest`].
    ///
    /// `[project].main` is the input `.tex` file and `[build].name` is the
    /// name of the output PDF.
    ///
    /// # Errors
    ///
    /// Returns [`CompilerError::MissingMain`] if `[project].main` is not set,
    /// or [`CompilerError::MissingBuild`] if the `[build]` section is absent.
    pub fn from_manifest(manifest: &LtxManifest) -> Result<Self, CompilerError> {
        let main_file = manifest
            .project
            .get_main_project()
            .ok_or(CompilerError::MissingMain)?;

        let build = manifest.build.as_ref().ok_or(CompilerError::MissingBuild)?;

        Ok(Self {
            engine: build.engine(),
            engine_args: build.engine_args().map_or(Vec::new(), <[String]>::to_vec),
            output_name: build.name().unwrap_or("output").to_string(),
            main_file: main_file.to_string(),
            compile_options: build.compile_options(),
        })
    }

    /// function that returns the name of the engine.
    ///
    /// Returns the engine name as a `&str` (e.g. `"pdflatex"`).
    #[inline]
    #[must_use]
    pub const fn engine_name(&self) -> &str {
        match self.engine {
            CompilerEngine::PdfLaTeX => "pdflatex",
            CompilerEngine::XeLaTeX => "xelatex",
            CompilerEngine::LuaLaTeX => "lualatex",
            CompilerEngine::Tectonic => "tectonic",
        }
    }

    /// Function that returns the name of the output file.
    ///
    /// Returns the output name as a `&str` (e.g. `"output"`).
    #[inline]
    #[must_use]
    pub fn output_name(&self) -> &str {
        &self.output_name
    }

    /// Function that returns the path to the main `.tex` input file.
    ///
    /// Returns the path as a `&str` (e.g. `"main.tex"`), relative to the
    /// project root.
    #[inline]
    #[must_use]
    pub fn main_file(&self) -> &str {
        &self.main_file
    }
}
