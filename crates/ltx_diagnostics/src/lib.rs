//! This crate provides diagnostic utilities for LaTeX compilation.

/// Byte-range tracking for source code locations.
///
/// Uses byte offsets (not char/column) to align with `logos` lexer output
/// and `miette` diagnostic expectations. File paths are stored as `Arc<str>`
/// for efficient cloning.
pub mod severity;

/// Severity levels for LaTeX diagnostics.
///
/// Categorizes issues by urgency:
/// - `Error`: Must fix (compilation fails)
/// - `Warning`: Should fix (works but bad practice)
/// - `Hint`: FYI (style suggestions)
pub mod span;

/// Core diagnostic types for LaTeX errors and warnings.
///
/// Defines the `LtxDiagnostic` enum with variants for common LaTeX issues
/// (undefined commands, missing braces, etc.). Each variant implements
/// `thiserror::Error` and `miette::Diagnostic` for rich error reporting
/// with source code labels and help messages.
pub mod diagnostic;

/// Diagnostic collection buffer that never panics.
///
/// `DiagnosticSink` accumulates `LtxDiagnostic` instances from lexer,
/// parser, and linter phases. Allows error recovery by continuing
/// processing after issues are found, then reporting all problems
/// at once.
pub mod sink;

/// Serialization layer for diagnostics output.
///
/// Converts `LtxDiagnostic` into JSON-serializable structures for
/// consumption by `ltx-cli`. Terminal rendering happens in the CLI
/// crate; this module only handles data transformation.
pub mod render;

/// A mod of errors
pub mod errors;

/// Source file management and span resolution.
pub mod source_file;

// convenience re-exports
pub use diagnostic::{LtxDiagnostic, LtxDiagnosticInner};
pub use render::{JsonDiagnostic, render_json};
pub use severity::LtxSeverity;
pub use sink::LtxDiagnosticSink;
pub use source_file::{LtxSourceFile, LtxSourceMap};
pub use span::{LtxSpan, LtxFileId};
