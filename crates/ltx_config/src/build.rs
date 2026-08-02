//! Build output configuration for the `[build]` section of `config.toml`.

use serde::{Deserialize, Serialize};

use crate::engine::CompilerEngine;

/// Tectonic compilation options for the `[build.options]` section.
///
/// Each field defaults to a sensible value when left out of `ltx.toml`, so a
/// project only needs to set the options it wants to override.
///
/// # Examples
///
/// ```toml
/// [build]
/// name = "paper"
/// engine = "tectonic"
///
/// [build.options]
/// keep_logs = false
/// only_cached = true
/// ```
// Clippy: this mirrors tectonic's own option set; grouping the flags would
// obscure the one-to-one mapping with `ProcessingSessionBuilder`.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompileOptions {
    /// Keep the `.log` file produced by the compiler.
    #[serde(default = "default_true")]
    pub keep_logs: bool,
    /// Keep intermediate build artifacts (e.g. `.aux`, `.synctex.gz`).
    #[serde(default)]
    pub keep_intermediates: bool,
    /// Emit `SyncTeX` data for editor / PDF synchronization.
    #[serde(default = "default_true")]
    pub synctex: bool,
    /// If true, never hit the network — fail if the bundle isn't cached.
    #[serde(default)]
    pub only_cached: bool,
}

/// Serde default used for options that are on by default.
const fn default_true() -> bool {
    true
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            keep_logs: true,
            keep_intermediates: false,
            synctex: true,
            only_cached: false,
        }
    }
}

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
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Build {
    /// Name of the output PDF file (without extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// The LaTeX compiler engine to use.
    engine: CompilerEngine,

    /// Extra command-line arguments passed to the compiler.
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_args: Option<Vec<String>>,

    /// Compilation options (logs, intermediates, `SyncTeX`, offline mode).
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<CompileOptions>,
}

impl Build {
    /// Creates a new build configuration with the given output name and engine.
    #[must_use]
    pub fn new(name: impl Into<String>, engine: CompilerEngine) -> Self {
        Self {
            name: Some(name.into()),
            engine,
            engine_args: None,
            options: None,
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

    /// Returns the resolved compilation options, with defaults applied.
    ///
    /// Returns [`CompileOptions::default()`] when no `[build.options]`
    /// section is present.
    #[must_use]
    pub fn compile_options(&self) -> CompileOptions {
        self.options.unwrap_or_default()
    }

    /// Sets the compilation options.
    pub const fn set_options(&mut self, options: CompileOptions) {
        self.options = Some(options);
    }
}
