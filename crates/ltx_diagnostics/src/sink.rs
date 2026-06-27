use std::cmp::Reverse;

use crate::{LtxDiagnostic, LtxSeverity};

/// A `LtxDiagnosticSink` is a sink for Ltx diagnostics
/// that collects and stores them for later retrieval
/// and sorting.
#[derive(Default, Debug)]
pub struct LtxDiagnosticSink {
    /// The diagnostics that have been reported.
    inner: Vec<LtxDiagnostic>,
    /// Cached indicator if the sink contains any error-severity diagnostics.
    has_error: bool,
}

impl LtxDiagnosticSink {
    /// Creates a new, empty diagnostic sink
    ///
    /// # Returns
    ///
    /// A new `LtxDiagnosticSink` with no diagnostics
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a diagnostic to the sink.
    ///
    /// # Arguments
    ///
    /// * `diagnostic` - The diagnostic to push.
    ///
    /// # Panics
    ///
    /// Panics if the diagnostic's severity is `LtxSeverity::Fatal`.
    #[inline]
    pub fn push(&mut self, diagnostic: LtxDiagnostic) {
        if diagnostic.severity() == LtxSeverity::Error {
            self.has_error = true;
        }
        self.inner.push(diagnostic);
    }

    /// Returns `true` if the sink contains any diagnostics.
    ///
    /// # Returns
    ///
    /// `true` if the sink contains any diagnostics, `false` otherwise.
    #[must_use]
    #[inline]
    pub fn has_error(&self) -> bool {
        self.has_error
    }

    /// Returns `true` if the inner collection of diagnostics is empty.
    ///
    /// # Returns
    ///
    /// `true` if the inner collection of diagnostics is empty, `false` otherwise.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Return the len of the inner collection of diagnostics.
    ///
    /// # Returns
    ///
    /// The number of diagnostics in the sink.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Consumes the sink and returns all collected diagnostics.
    ///
    /// # Returns
    ///
    /// A vector of all collected diagnostics.
    #[must_use]
    #[inline]
    pub fn into_diagnostics(self) -> Vec<LtxDiagnostic> {
        self.inner
    }

    /// Returns all the diagnostics in the sink.
    ///
    /// # Returns
    ///
    /// A slice of all the diagnostics in the sink.
    ///
    /// # Notes
    ///
    /// The returned slice is a borrowed reference to the inner collection of diagnostics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ltx_diagnostics::LtxDiagnosticSink;
    /// let sink = LtxDiagnosticSink::default();
    /// assert!(sink.all().is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub fn all(&self) -> &[LtxDiagnostic] {
        &self.inner
    }

    /// Consume the sink; return diagnostics sorted errors-first.
    ///
    /// # Returns
    ///
    /// A vector of diagnostics sorted errors-first.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ltx_diagnostics::LtxDiagnosticSink;
    /// let sink = LtxDiagnosticSink::default();
    /// assert!(sink.drain_sorted().is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub fn drain_sorted(mut self) -> Vec<LtxDiagnostic> {
        self.inner.sort_by_key(|d| Reverse(d.severity()));
        self.inner
    }

    /// Returns all the diagnostics in the sink that have the given severity.
    ///
    /// # Returns
    ///
    /// A vector of diagnostics that have the given severity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ltx_diagnostics::LtxDiagnosticSink;
    /// let sink = LtxDiagnosticSink::default();
    /// assert!(sink.get_by_severity(ltx_diagnostics::LtxSeverity::Error).is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub fn get_by_severity(mut self, severity: LtxSeverity) -> Vec<LtxDiagnostic> {
        self.inner.retain(|d| d.severity() == severity);
        self.inner
    }
}
