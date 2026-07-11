//! Unified diagnostic infrastructure for the LTX toolchain.
//!
//! Errors are grouped by what they mean *to the user writing LaTeX*, not by
//! which compiler phase (lexer, parser, linter) detected them. A parser and a
//! lexer can both raise `UnmatchedBrace`; the caller shouldn't need to care
//! which pass caught it.
//!
//! # Core types
//!
//! | Type | Role |
//! |------|------|
//! | [`LtxError`] | A single diagnosable error with code, message, span, and help text. |
//! | [`LtxDiagnostic`] | Wraps an `LtxError` with the [`LtxSourceMap`] needed to render it. |
//! | [`LtxDiagnosticSink`] | Accumulates diagnostics across phases for batch reporting. |
//! | [`LtxSourceMap`] / [`LtxSourceFile`] | Source-text registry for span → line:column resolution. |
//! | [`LtxSpan`] / [`LtxFileId`] | Byte-range location in a specific file. |
//! | [`LtxSeverity`] | Error / Warning / Hint classification. |

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

/// Core diagnostic types for LaTeX errors and warnings.
///
/// Defines the `LtxError` enum with variants for common LaTeX issues
/// (undefined commands, missing braces, etc.). Each variant implements
/// `thiserror::Error` and `miette::Diagnostic` for rich error reporting
/// with source code labels and help messages.
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

/// Unified error variants shared across all compiler phases.
///
/// See the module-level documentation for the error code naming
/// convention (`LTX::E0xx` for syntax, `LTX::E1xx` for structural).
pub mod errors;

/// Source file management and span resolution.
///
/// [`LtxSourceMap`] stores loaded files and provides byte-offset →
/// line:column mapping needed by miette and JSON rendering.
pub mod source_file;

// convenience re-exports
pub use diagnostic::LtxDiagnostic;
pub use errors::LtxError;
pub use render::{JsonDiagnostic, render_json_into, render_pretty, render_pretty_into};
pub use severity::LtxSeverity;
pub use sink::LtxDiagnosticSink;
pub use source_file::{LtxSourceFile, LtxSourceMap};
pub use span::{LtxFileId, LtxSpan};
