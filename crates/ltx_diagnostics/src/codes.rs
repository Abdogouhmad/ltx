//! Error code registry for all LTX diagnostics.
//!
//! This module provides a quick-reference table mapping every [`LtxError`](crate::LtxError)
//! variant to its diagnostic code, severity, and a short description.
//!
//! Code ranges follow the convention established in [`errors`](crate::errors):
//!
//! | Range | Category |
//! |-------|----------|
//! | `LTX::E0xx` | Syntax / tokenization (braces, delimiters, escapes) |
//! | `LTX::E1xx` | Structural / semantic (commands, environments, references) |
//! | `LTX::W0xx` | Lint warnings (reserved) |

use std::fmt;

/// Metadata for a single diagnostic code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCode {
    /// The diagnostic code string, e.g. `"LTX::E001"`.
    pub code: &'static str,
    /// Human-readable short description.
    pub description: &'static str,
    /// Default severity level.
    pub severity: &'static str,
}

impl ErrorCode {
    const fn new(code: &'static str, description: &'static str, severity: &'static str) -> Self {
        Self {
            code,
            description,
            severity,
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} — {}", self.code, self.description)
    }
}

/// All registered error codes.
pub const ALL_CODES: &[ErrorCode] = &[
    // ── LTX::E0xx — syntax / tokenization ──────────────────────────
    ErrorCode::new("LTX::E001", "Unexpected Token", "error"),
    ErrorCode::new("LTX::E002", "Unexpected End of File", "error"),
    ErrorCode::new("LTX::E003", "Unmatched Brace", "error"),
    ErrorCode::new("LTX::E004", "Invalid Math Delimiter", "error"),
    ErrorCode::new("LTX::E005", "Unterminated Argument", "error"),
    ErrorCode::new("LTX::E006", "Invalid Escape Sequence", "error"),
    ErrorCode::new("LTX::E007", "Invalid Unicode", "error"),
    ErrorCode::new("LTX::E008", "Illegal Parameter Character Usage", "error"),
    ErrorCode::new("LTX::E009", "Unterminated Verbatim Block", "error"),
    ErrorCode::new("LTX::E010", "Invalid Character", "error"),
    // ── LTX::E1xx — structural / semantic ───────────────────────────
    ErrorCode::new("LTX::E100", "Undefined Control Sequence", "error"),
    ErrorCode::new("LTX::E101", "Mismatched Environment", "error"),
    ErrorCode::new("LTX::E102", "Unclosed Environment", "error"),
    ErrorCode::new("LTX::E103", "Undefined Environment", "error"),
    ErrorCode::new("LTX::E104", "Undefined Reference", "warning"),
    ErrorCode::new("LTX::E105", "Missing Package", "error"),
    ErrorCode::new("LTX::E106", "File Not Found", "error"),
    ErrorCode::new("LTX::E107", "Misplaced Alignment Tab", "error"),
    ErrorCode::new("LTX::E108", "Command Redefined", "error"),
];

/// Lookup an error code by its string identifier.
///
/// Returns `None` if the code is not registered.
#[must_use]
pub fn lookup(code: &str) -> Option<&'static ErrorCode> {
    ALL_CODES.iter().find(|e| e.code == code)
}

/// Returns the total number of registered error codes.
#[must_use]
pub const fn count() -> usize {
    ALL_CODES.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_codes_are_unique() {
        let mut codes: Vec<&str> = ALL_CODES.iter().map(|e| e.code).collect();
        let original_len = codes.len();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(
            codes.len(),
            original_len,
            "duplicate error codes found in ALL_CODES"
        );
    }

    #[test]
    fn test_lookup_existing_code() {
        let e = lookup("LTX::E001").unwrap_or_else(|| panic!("LTX::E001 should be registered"));
        assert_eq!(e.description, "Unexpected Token");
        assert_eq!(e.severity, "error");
    }

    #[test]
    fn test_lookup_nonexistent_code() {
        assert!(lookup("LTX::E999").is_none());
    }

    #[test]
    fn test_count_matches_all_codes() {
        assert_eq!(count(), ALL_CODES.len());
    }
}
