use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
/// Represents the severity of a diagnostic.
pub enum LtxSeverity {
    /// Represents an error severity.
    Error,
    /// Represents a warning severity.
    Warning,
    /// Represents an info severity.
    Hint,
}

/// Severity is a simple enum representing the severity of a diagnostic.
impl LtxSeverity {
    /// Returns `true` if the severity is an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    /// Converts the severity to a miette severity.
    #[must_use]
    pub const fn to_miette(&self) -> miette::Severity {
        match self {
            Self::Error => miette::Severity::Error,
            Self::Warning => miette::Severity::Warning,
            Self::Hint => miette::Severity::Advice,
        }
    }
}
