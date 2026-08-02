//! Configuration for the Ltx compiler.
use ltx_config::{CompileOptions, LtxManifest, engine::CompilerEngine};

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

/// Errors that can occur while building a [`CompilerConfig`].
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// The `[project]` table is missing a `main` entry.
    #[error("no main file set in ltx.toml — add `main = \"main.tex\"` under `[project]`")]
    MissingMain,
    /// The manifest has no `[build]` section.
    #[error("no `[build]` section in ltx.toml — add an `engine` and `name` for the PDF output")]
    MissingBuild,
}

impl CompilerConfig {
    /// Creates a new [`CompilerConfig`] from a [`LtxManifest`].
    ///
    /// `[project].main` is the input `.tex` file and `[build].name` is the
    /// name of the output PDF.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::MissingMain`] if `[project].main` is not set,
    /// or [`ConfigError::MissingBuild`] if the `[build]` section is absent.
    pub fn from_manifest(manifest: &LtxManifest) -> Result<Self, ConfigError> {
        let main_file = manifest
            .project
            .get_main_project()
            .ok_or(ConfigError::MissingMain)?;

        let build = manifest.build.as_ref().ok_or(ConfigError::MissingBuild)?;

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

    // pub fn command(&self) -> (String, Vec<String>) {
    //     let name = &self.output_name;
    //     match self.engine {
    //         CompilerEngine::PdfLaTeX => {
    //             let mut args = vec!["-interaction=nonstopmode".into(), format!("{name}.tex")];
    //             args.extend(self.engine_args.clone());
    //             ("pdflatex".into(), args)
    //         }
    //         CompilerEngine::XeLaTeX => {
    //             let mut args = vec!["-interaction=nonstopmode".into(), format!("{name}.tex")];
    //             args.extend(self.engine_args.clone());
    //             ("xelatex".into(), args)
    //         }
    //         CompilerEngine::LuaLaTeX => {
    //             let mut args = vec!["-interaction=nonstopmode".into(), format!("{name}.tex")];
    //             args.extend(self.engine_args.clone());
    //             ("lualatex".into(), args)
    //         }
    //         CompilerEngine::Tectonic => {
    //             let mut args = vec![format!("{name}.tex")];
    //             args.extend(self.engine_args.clone());
    //             ("tectonic".into(), args)
    //         }
    //     }
    // }
}
