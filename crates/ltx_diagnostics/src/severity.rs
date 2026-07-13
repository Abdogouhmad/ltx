use serde::Serialize;

/// Severity of a diagnostic message.
///
/// Ordered by increasing urgency: `Hint` < `Warning` < `Error`.
/// Implements `Ord` so diagnostics can be sorted errors-first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LtxSeverity {
    /// Must fix — compilation fails.
    Error = 2,
    /// Should fix — works but is bad practice.
    Warning = 1,
    /// Informational — style suggestions, FYI.
    Hint = 0,
}

impl LtxSeverity {
    /// Returns `true` if the severity is [`Error`](Self::Error).
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    /// Converts to the corresponding [`miette::Severity`].
    #[must_use]
    #[inline]
    pub const fn to_miette(&self) -> miette::Severity {
        match self {
            Self::Error => miette::Severity::Error,
            Self::Warning => miette::Severity::Warning,
            Self::Hint => miette::Severity::Advice,
        }
    }

    /// Converts from a [`miette::Severity`] back to [`LtxSeverity`].
    ///
    /// Unknown miette variants (which don't exist today, but the API is
    /// non-exhaustive) fall back to [`Error`](Self::Error).
    #[must_use]
    #[inline]
    pub const fn from_miette(sev: miette::Severity) -> Self {
        match sev {
            miette::Severity::Error => Self::Error,
            miette::Severity::Warning => Self::Warning,
            miette::Severity::Advice => Self::Hint,
        }
    }
}
