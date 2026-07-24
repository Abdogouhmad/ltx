//! LaTeX engine configuration.

use std::fmt;
use std::str::FromStr;

use serde::Serialize;

/// Supported LaTeX compilers.
///
/// Each variant maps to its CLI binary name (e.g. `PdfLaTeX` → `"pdflatex"`).
/// Used as the value of `--engine` in the CLI and serialized as a string in
/// `config.toml`.
///
/// # Examples
///
/// ```rust
/// use ltx_config::CompilerEngine;
///
/// let engine: CompilerEngine = "xelatex".parse().unwrap();
/// assert_eq!(engine, CompilerEngine::XeLaTeX);
/// assert_eq!(engine.to_string(), "xelatex");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CompilerEngine {
    /// `pdflatex` — the most common engine.
    #[default]
    PdfLaTeX,
    /// `xelatex` — Unicode and system-font support.
    XeLaTeX,
    /// `lualatex` — Lua-extensible engine.
    LuaLaTeX,
    /// `tectonic` — self-contained, Cargo-like LaTeX toolchain.
    Tectonic,
}

impl CompilerEngine {
    /// Returns the binary name of the engine.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PdfLaTeX => "pdflatex",
            Self::XeLaTeX => "xelatex",
            Self::LuaLaTeX => "lualatex",
            Self::Tectonic => "tectonic",
        }
    }
}

impl fmt::Display for CompilerEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for CompilerEngine {
    type Err = String;

    /// Parses a case-insensitive engine name.
    ///
    /// # Errors
    ///
    /// Returns the input string if it doesn't match any known engine.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "pdflatex" => Ok(Self::PdfLaTeX),
            "xelatex" => Ok(Self::XeLaTeX),
            "lualatex" => Ok(Self::LuaLaTeX),
            "tectonic" => Ok(Self::Tectonic),
            other => Err(format!(
                "unknown engine `{other}` — expected one of: pdflatex, xelatex, lualatex, tectonic"
            )),
        }
    }
}

/// The `config.toml` `[engine]` section.
///
/// # Examples
///
/// ```rust
/// use ltx_config::{CompilerEngine, Engine};
///
/// let engine = Engine::new(CompilerEngine::LuaLaTeX);
/// assert_eq!(engine.compiler(), CompilerEngine::LuaLaTeX);
/// assert!(engine.args().is_none());
/// ```
#[derive(Serialize, Debug)]
pub struct Engine {
    /// The compiler to invoke.
    compiler: CompilerEngine,

    /// Optional extra command-line arguments passed to the compiler.
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<Vec<String>>,
}

impl Engine {
    /// Creates a new engine with no extra arguments.
    #[must_use]
    pub const fn new(compiler: CompilerEngine) -> Self {
        Self {
            compiler,
            args: None,
        }
    }

    /// Returns the compiler.
    #[must_use]
    pub const fn compiler(&self) -> CompilerEngine {
        self.compiler
    }

    /// Returns the extra arguments, if any have been set.
    #[must_use]
    pub fn args(&self) -> Option<&[String]> {
        self.args.as_deref()
    }

    /// Sets the extra command-line arguments.
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = Some(args);
    }
}
