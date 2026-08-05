//! Unified error definitions for the compiler.
//!
//! The compiler owns every error it can produce while turning a manifest
//! into a compiled PDF: missing configuration, missing input files, and
//! engine (tectonic) failures. Each variant carries its own help text and a
//! namespaced error code of the form `LTX::COMPILER::Exxx` (errors) or
//! `LTX::COMPILER::W0xx` (warnings).
//!
//! # Code ranges
//!
//! - `LTX::COMPILER::E001` – `E004` — configuration and compilation errors.
//! - `LTX::COMPILER::E005` – `E006` — watch-mode errors.
//! - `LTX::COMPILER::W001` — unimplemented engine warning.

use std::path::PathBuf;

use ltx_diagnostics::ErrorCode;
use miette::Diagnostic;
use thiserror::Error;

/// All registered compiler diagnostic codes.
///
/// Each entry maps a code (`LTX::COMPILER::E0xx` / `W0xx`) to its
/// description, default severity, and owning phase (`"compiler"`).
pub const ALL_CODES: &[ErrorCode] = &[
    ErrorCode::new(
        "LTX::COMPILER::E001",
        "Missing Main Field",
        "error",
        "compiler",
    ),
    ErrorCode::new(
        "LTX::COMPILER::E002",
        "Missing Build Section",
        "error",
        "compiler",
    ),
    ErrorCode::new(
        "LTX::COMPILER::E003",
        "Main File Not Found",
        "error",
        "compiler",
    ),
    ErrorCode::new("LTX::COMPILER::E004", "Engine Failure", "error", "compiler"),
    ErrorCode::new(
        "LTX::COMPILER::E005",
        "Watcher Init Failed",
        "error",
        "compiler",
    ),
    ErrorCode::new(
        "LTX::COMPILER::E006",
        "Watch Channel Closed",
        "error",
        "compiler",
    ),
    ErrorCode::new(
        "LTX::COMPILER::W001",
        "Engine Not Implemented",
        "warning",
        "compiler",
    ),
];

/// Every error the compiler crate can produce.
///
/// Implements [`miette::Diagnostic`] so it converts into a `miette::Report`
/// via `From` and can be returned directly from `miette::Result` functions.
#[derive(Debug, Error, Diagnostic)]
#[non_exhaustive]
pub enum CompilerError {
    /// **`LTX::COMPILER::E001`: Missing Main Field**
    ///
    /// The `[project]` section of `ltx.toml` has no `main` entry, so the
    /// compiler does not know which file to compile.
    #[error("no main file set in ltx.toml — add `main = \"main.tex\"` under `[project]`")]
    #[diagnostic(
        code(LTX::COMPILER::E001),
        severity(Error),
        help("set the `[project].main` key to the path of your main `.tex` file")
    )]
    MissingMain,

    /// **`LTX::COMPILER::E002`: Missing Build Section**
    ///
    /// The manifest has no `[build]` section, so the engine and output name
    /// are unknown.
    #[error("no `[build]` section in ltx.toml — add an `engine` and `name` for the PDF output")]
    #[diagnostic(
        code(LTX::COMPILER::E002),
        severity(Error),
        help("add a `[build]` section with `name = \"...\"` and `engine = \"tectonic\"`")
    )]
    MissingBuild,

    /// **`LTX::COMPILER::E003`: Main File Not Found**
    ///
    /// The main input file resolved from `[project].main` does not exist on
    /// disk at build time.
    #[error("main file `{path}` not found — check the `main` entry in ltx.toml")]
    #[diagnostic(
        code(LTX::COMPILER::E003),
        severity(Error),
        help("create the file or fix the `main` value in the `[project]` section")
    )]
    MainFileNotFound {
        /// The resolved path that was expected to exist.
        path: PathBuf,
    },

    /// **`LTX::COMPILER::E004`: Engine Failure**
    ///
    /// The selected compiler engine (tectonic) failed to fetch its support
    /// bundle, create its processing session, or finish the compilation.
    #[error("{message}")]
    #[diagnostic(
        code(LTX::COMPILER::E004),
        severity(Error),
        help("check the engine logs; for bundle issues verify your network connection")
    )]
    TectonicError {
        /// A human-readable description of the engine failure.
        message: String,
    },

    /// **`LTX::COMPILER::W001`: Engine Not Implemented**
    ///
    /// The selected engine has not been wired up yet — only `tectonic` is
    /// available, so the build falls through without producing a PDF.
    #[error("`{engine}` engine is not implemented yet — only `tectonic` is available")]
    #[diagnostic(
        code(LTX::COMPILER::W001),
        severity(Warning),
        help("use `engine = \"tectonic\"` in `ltx.toml`, or wait for this engine to land")
    )]
    EngineNotImplemented {
        /// The name of the unimplemented engine (e.g. `"pdflatex"`).
        engine: &'static str,
    },

    /// **`LTX::COMPILER::E005`: Watcher Init Failed**
    ///
    /// The underlying file watcher (`notify`) could not be created or could
    /// not watch the project root.
    #[error("failed to initialize file watcher: {0}")]
    #[diagnostic(
        code(LTX::COMPILER::E005),
        severity(Error),
        help("make sure the watched directory exists and is readable")
    )]
    Init(#[from] notify::Error),

    /// **`LTX::COMPILER::E006`: Watch Channel Closed**
    ///
    /// The event channel between the watcher and the compile loop disconnected
    /// before the watch session finished.
    #[error("watch channel closed unexpectedly")]
    #[diagnostic(
        code(LTX::COMPILER::E006),
        severity(Error),
        help("the file watcher terminated — this is usually a platform limitation")
    )]
    ChannelClosed,
}
