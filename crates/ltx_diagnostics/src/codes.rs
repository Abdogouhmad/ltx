//! Error code registry metadata.
//!
//! This module defines the [`ErrorCode`] struct used by every crate to expose
//! its diagnostics reference table. Domain crates publish their own static
//! lists (e.g. `ltx_lexer::ALL_CODES`) and the `ltx code` CLI command
//! aggregates them, filtering by the owning phase.
//!
//! Code conventions per crate:
//!
//! | Prefix | Phase |
//! |--------|-------|
//! | `LTX::LEXER::` | Lexical analysis |
//! | `LTX::PARSER::` | Syntax parsing / AST |
//! | `LTX::CONFIG::` | Manifest / configuration |
//! | `LTX::COMPILER::` | Compilation pipeline |
//!
//! Within a phase, `E0xx` denotes errors and `W0xx` denotes warnings.

use std::fmt;

/// Metadata for a single diagnostic code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCode {
    /// The diagnostic code string, e.g. `"LTX::LEXER::E001"`.
    pub code: &'static str,
    /// Human-readable short description.
    pub description: &'static str,
    /// Default severity level (`"error"` or `"warning"`).
    pub severity: &'static str,
    /// The crate / compiler phase that owns this code.
    pub phase: &'static str,
}

impl ErrorCode {
    /// Creates a new code entry with its metadata.
    ///
    /// # Arguments
    ///
    /// * `code` - The diagnostic code string (e.g. `"LTX::LEXER::E001"`).
    /// * `description` - A short human-readable description.
    /// * `severity` - `"error"` or `"warning"`.
    /// * `phase` - The owning phase (e.g. `"lexer"`).
    #[must_use]
    pub const fn new(
        code: &'static str,
        description: &'static str,
        severity: &'static str,
        phase: &'static str,
    ) -> Self {
        Self {
            code,
            description,
            severity,
            phase,
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} — {}", self.code, self.description)
    }
}
