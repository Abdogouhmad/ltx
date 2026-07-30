//! Build output configuration for the `[build]` section of `config.toml`.

use serde::{Deserialize, Serialize};

use crate::engine::CompilerEngine;

/// Controls how and where the project is compiled.
///
/// The `[build]` section specifies the target PDF name, the LaTeX engine,
/// and optional extra arguments. The output directory is always forced to
/// `<project>/target/` by the compiler crate.
///
/// # Examples
///
/// ```rust
/// use ltx_config::{Build, CompilerEngine};
///
/// let build = Build::new("paper", CompilerEngine::PdfLaTeX);
/// assert_eq!(build.name(), Some("paper"));
/// assert_eq!(build.engine(), CompilerEngine::PdfLaTeX);
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    /// Name of the output PDF file (without extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// The LaTeX compiler engine to use.
    engine: CompilerEngine,

    /// Extra command-line arguments passed to the compiler.
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_args: Option<Vec<String>>,
}

impl Build {
    /// Creates a new build configuration with the given output name and engine.
    #[must_use]
    pub fn new(name: impl Into<String>, engine: CompilerEngine) -> Self {
        Self {
            name: Some(name.into()),
            engine,
            engine_args: None,
        }
    }

    /// Returns the output PDF name, if set.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the compiler engine.
    #[must_use]
    pub const fn engine(&self) -> CompilerEngine {
        self.engine
    }

    /// Returns the extra arguments, if any.
    #[must_use]
    pub fn engine_args(&self) -> Option<&[String]> {
        self.engine_args.as_deref()
    }

    /// Sets the extra command-line arguments for the engine.
    pub fn set_engine_args(&mut self, args: Vec<String>) {
        self.engine_args = Some(args);
    }
}

impl Default for Build {
    fn default() -> Self {
        Self {
            name: None,
            engine: CompilerEngine::default(),
            engine_args: None,
        }
    }
}
