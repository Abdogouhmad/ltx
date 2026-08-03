//! Unified diagnostic infrastructure for the LTX toolchain.
//!
//! This crate is **purely infrastructure** — it defines no domain-specific
//! errors. Instead, every compiler phase (lexer, parser, config, compiler)
//! owns its own error enum and plugs it into the shared pipeline through the
//! [`LtxDiagnosticSource`] trait.
//!
//! # Core types
//!
//! | Type | Role |
//! |------|------|
//! | [`LtxDiagnostic`] | Wraps any [`LtxDiagnosticSource`] with the [`LtxSourceMap`] needed to render it. |
//! | [`LtxDiagnosticSink`] | Accumulates diagnostics across phases for batch reporting. |
//! | [`LtxSourceMap`] / [`LtxSourceFile`] | Source-text registry for span → line:column resolution. |
//! | [`LtxSpan`] / [`LtxFileId`] | Byte-range location in a specific file. |
//! | [`LtxSeverity`] | Error / Warning / Hint classification. |
//! | [`ErrorCode`] | Registry entry describing a single diagnostic code. |

/// Severity levels for LaTeX diagnostics.
///
/// Categorizes issues by urgency:
/// - `Error` — must fix (compilation fails)
/// - `Warning` — should fix (works but bad practice)
/// - `Hint` — FYI (style suggestions)
pub mod severity;

/// Byte-range tracking for source code locations.
///
/// Uses byte offsets (not char/column) to align with `logos` lexer output
/// and `miette` diagnostic expectations. File paths are stored as `Arc<str>`
/// for efficient cloning.
pub mod span;

/// Core diagnostic types: the [`LtxDiagnosticSource`] trait, [`LtxDiagnostic`],
/// and its [`miette::Diagnostic`] implementation.
pub mod diagnostic;

/// Diagnostic collection buffer that never panics.
///
/// [`LtxDiagnosticSink`] accumulates [`LtxDiagnostic`] instances from lexer,
/// parser, and linter phases. Allows error recovery by continuing
/// processing after issues are found, then reporting all problems
/// at once.
pub mod sink;

/// Serialization layer for diagnostics output.
///
/// Converts [`LtxDiagnostic`] into JSON-serializable structures for
/// consumption by `ltx-cli`. Terminal rendering happens in the CLI
/// crate; this module only handles data transformation.
pub mod render;

/// Error code registry metadata — the [`ErrorCode`] struct that every crate
/// uses to publish its reference table.
pub mod codes;

/// Source file management and span resolution.
///
/// [`LtxSourceMap`] stores loaded files and provides byte-offset →
/// line:column mapping needed by miette and JSON rendering.
pub mod source_file;

// convenience re-exports
pub use codes::ErrorCode;
pub use diagnostic::{LtxDiagnostic, LtxDiagnosticSource};
pub use render::{JsonDiagnostic, render_json_into, render_pretty, render_pretty_into};
pub use severity::LtxSeverity;
pub use sink::LtxDiagnosticSink;
pub use source_file::{LtxSourceFile, LtxSourceMap};
pub use span::{LtxFileId, LtxSpan};
