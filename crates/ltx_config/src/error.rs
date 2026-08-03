//! Unified error definitions for the config crate.
//!
//! The config crate owns every error it can produce while reading or
//! validating `ltx.toml` and while scaffolding a new project. Each variant
//! carries its own help text, label, and a namespaced error code of the form
//! `LTX::CONFIG::E0xx`.
//!
//! Manifest validation errors embed the source text and a byte span so they
//! render with miette snippets pointing at the offending table or key. Read
//! failures and scaffold I/O errors have no source snippet.
//!
//! # Code ranges
//!
//! - `LTX::CONFIG::E001` – `E006` — manifest validation (missing fields,
//!   malformed TOML, unreadable file).
//! - `LTX::CONFIG::E007` – `E008` — project scaffolding.

use std::path::{Path, PathBuf};

use ltx_diagnostics::ErrorCode;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// All registered config diagnostic codes.
///
/// Each entry maps a code (`LTX::CONFIG::E0xx`) to its description, default
/// severity, and owning phase (`"config"`).
pub const ALL_CODES: &[ErrorCode] = &[
    ErrorCode::new("LTX::CONFIG::E001", "Missing Main Field", "error", "config"),
    ErrorCode::new(
        "LTX::CONFIG::E002",
        "Missing Build Section",
        "error",
        "config",
    ),
    ErrorCode::new("LTX::CONFIG::E003", "Missing Build Name", "error", "config"),
    ErrorCode::new(
        "LTX::CONFIG::E004",
        "Main File Not Found",
        "error",
        "config",
    ),
    ErrorCode::new("LTX::CONFIG::E005", "Invalid TOML", "error", "config"),
    ErrorCode::new(
        "LTX::CONFIG::E006",
        "Manifest Read Failed",
        "error",
        "config",
    ),
    ErrorCode::new("LTX::CONFIG::E007", "Scaffold I/O Error", "error", "config"),
    ErrorCode::new(
        "LTX::CONFIG::E008",
        "Project Already Exists",
        "error",
        "config",
    ),
];

/// Every error the config crate can produce, merged into one enum.
///
/// Implements [`miette::Diagnostic`] directly so errors returned by
/// [`validate_manifest`](crate::validate_manifest) and
/// [`LtxManifest::from_file`](crate::LtxManifest::from_file) render with
/// source snippets, labels, and help text.
#[derive(Debug, Error, Diagnostic)]
#[non_exhaustive]
pub enum ConfigError {
    /// **`LTX::CONFIG::E001`: Missing Main Field**
    ///
    /// The `[project]` section has no `main` key.
    #[error("missing `main` in the `[project]` section")]
    #[diagnostic(
        code(LTX::CONFIG::E001),
        severity(Error),
        help("add `main = \"src/main.tex\"` (or the path to your main file)")
    )]
    MissingMain {
        /// The raw `ltx.toml` text used to render the snippet.
        #[source_code]
        source_text: String,
        /// Byte span of the `[project]` table.
        #[label("the `main` key is required here")]
        span: SourceSpan,
    },

    /// **`LTX::CONFIG::E002`: Missing Build Section**
    ///
    /// No `[build]` section was found in the manifest.
    #[error("missing `[build]` section")]
    #[diagnostic(
        code(LTX::CONFIG::E002),
        severity(Error),
        help("add a `[build]` section with `name = \"...\"` and `engine = \"tectonic\"`")
    )]
    MissingBuild {
        /// The raw `ltx.toml` text used to render the snippet.
        #[source_code]
        source_text: String,
        /// Byte span of the `[project]` table.
        #[label("the `[build]` section is required here")]
        span: SourceSpan,
    },

    /// **`LTX::CONFIG::E003`: Missing Build Name**
    ///
    /// The `[build]` section exists but has no `name` key.
    #[error("missing `name` in the `[build]` section")]
    #[diagnostic(
        code(LTX::CONFIG::E003),
        severity(Error),
        help("add `name = \"output\"` (without the `.pdf` extension)")
    )]
    MissingBuildName {
        /// The raw `ltx.toml` text used to render the snippet.
        #[source_code]
        source_text: String,
        /// Byte span of the `[build]` table.
        #[label("the `name` key is required here")]
        span: SourceSpan,
    },

    /// **`LTX::CONFIG::E004`: Main File Not Found**
    ///
    /// `[project].main` points to a file that does not exist on disk.
    #[error("main file not found: `{path}`")]
    #[diagnostic(
        code(LTX::CONFIG::E004),
        severity(Error),
        help("create the file or fix the `main` value in the `[project]` section")
    )]
    MainFileNotFound {
        /// The path that was expected to exist.
        path: PathBuf,
        /// The raw `ltx.toml` text used to render the snippet.
        #[source_code]
        source_text: String,
        /// Byte span of the `main = ...` key.
        #[label("this file does not exist")]
        span: SourceSpan,
    },

    /// **`LTX::CONFIG::E005`: Invalid TOML**
    ///
    /// The manifest could not be parsed (malformed TOML or unknown keys).
    #[error("invalid `ltx.toml`: {reason}")]
    #[diagnostic(
        code(LTX::CONFIG::E005),
        severity(Error),
        help("fix the TOML syntax or remove the unknown key before continuing")
    )]
    InvalidToml {
        /// A short, human-readable reason taken from the parser detail.
        reason: String,
        /// The full parser error detail shown in the help line.
        detail: String,
        /// The raw `ltx.toml` text used to render the snippet.
        #[source_code]
        source_text: String,
        /// Byte span reported by the TOML parser (falls back to the whole file).
        #[label("this is not valid `ltx.toml`")]
        span: SourceSpan,
    },

    /// **`LTX::CONFIG::E006`: Manifest Read Failed**
    ///
    /// The `ltx.toml` file could not be read from disk.
    #[error("failed to read `{path}`: {error}")]
    #[diagnostic(
        code(LTX::CONFIG::E006),
        severity(Error),
        help("make sure the file exists and is readable")
    )]
    ReadFailed {
        /// The path that could not be read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        error: std::io::Error,
    },

    /// **`LTX::CONFIG::E007`: Scaffold I/O Error**
    ///
    /// An I/O failure occurred while creating directories or files during
    /// project scaffolding.
    #[error(transparent)]
    #[diagnostic(code(LTX::CONFIG::E007), severity(Error))]
    Io(#[from] std::io::Error),

    /// **`LTX::CONFIG::E008`: Project Already Exists**
    ///
    /// The target directory already exists and is non-empty, so scaffolding
    /// refuses to overwrite it.
    #[error("project directory `{0}` already exists")]
    #[diagnostic(
        code(LTX::CONFIG::E008),
        severity(Error),
        help("remove the directory or choose a different project name")
    )]
    AlreadyExists(String),
}

impl ConfigError {
    /// Builds an [`E001`](ConfigError::MissingMain) diagnostic.
    pub(crate) fn missing_main(source: &str, span: SourceSpan) -> Self {
        Self::MissingMain {
            source_text: source.to_string(),
            span,
        }
    }

    /// Builds an [`E002`](ConfigError::MissingBuild) diagnostic.
    pub(crate) fn missing_build(source: &str, span: SourceSpan) -> Self {
        Self::MissingBuild {
            source_text: source.to_string(),
            span,
        }
    }

    /// Builds an [`E003`](ConfigError::MissingBuildName) diagnostic.
    pub(crate) fn missing_build_name(source: &str, span: SourceSpan) -> Self {
        Self::MissingBuildName {
            source_text: source.to_string(),
            span,
        }
    }

    /// Builds an [`E004`](ConfigError::MainFileNotFound) diagnostic.
    pub(crate) fn main_file_not_found(
        path: impl Into<PathBuf>,
        source: &str,
        span: SourceSpan,
    ) -> Self {
        Self::MainFileNotFound {
            path: path.into(),
            source_text: source.to_string(),
            span,
        }
    }

    /// Builds an [`E005`](ConfigError::InvalidToml) diagnostic from a TOML
    /// parser error.
    pub(crate) fn invalid_toml(err: &toml::de::Error, source: &str) -> Self {
        let detail = err.to_string();
        let reason = detail
            .lines()
            .last()
            .map_or_else(|| detail.as_str(), str::trim)
            .to_string();
        let span = err
            .span()
            .map_or_else(|| whole_span(source), SourceSpan::from);
        Self::InvalidToml {
            reason,
            detail,
            source_text: source.to_string(),
            span,
        }
    }

    /// Builds an [`E006`](ConfigError::ReadFailed) diagnostic.
    pub(crate) fn read_failed(err: &std::io::Error, path: &Path) -> Self {
        Self::ReadFailed {
            path: path.to_path_buf(),
            error: std::io::Error::new(err.kind(), err.to_string()),
        }
    }
}

/// Returns the byte span of the line whose trimmed content equals `needle`.
pub(crate) fn line_span(source: &str, needle: &str) -> Option<SourceSpan> {
    let mut offset = 0usize;
    for line in source.split_inclusive('\n') {
        if line.trim() == needle {
            return Some(SourceSpan::new(offset.into(), line.trim_end().len()));
        }
        offset += line.len();
    }
    None
}

/// Returns the byte span of the `[table]` header line.
pub(crate) fn table_span(source: &str, table: &str) -> Option<SourceSpan> {
    line_span(source, table)
}

/// Returns the byte span of the `key = ...` line inside the given `[table]`.
pub(crate) fn key_span(source: &str, table: &str, key: &str) -> Option<SourceSpan> {
    let mut offset = 0usize;
    let mut in_table = false;
    for line in source.split_inclusive('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_table = trimmed == table;
        } else if in_table
            && trimmed
                .strip_prefix(key)
                .is_some_and(|rest| rest.trim_start().starts_with('='))
        {
            return Some(SourceSpan::new(offset.into(), line.trim_end().len()));
        }
        offset += line.len();
    }
    None
}

/// Span covering the entire source, used as a fallback when a table is absent.
pub(crate) fn whole_span(source: &str) -> SourceSpan {
    SourceSpan::new(0.into(), source.len())
}
