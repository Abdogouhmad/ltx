//! Error handling for the lexer.
//!
//! [`LexerErrorHandler`] collects [`ltx_diagnostics::LtxDiagnostic`]s during tokenization.
//! Factory methods create [`ltx_diagnostics::LtxError`] variants at specific spans, wrap them
//! in [`ltx_diagnostics::LtxDiagnostic`]s (pairing the error with the [`ltx_diagnostics::LtxSourceMap`]), and
//! push them into an internal buffer.
//!
//! Error-code factory methods live in [`crate::errors_factory`].

use ltx_diagnostics::{
    LtxDiagnostic, LtxDiagnosticSink, LtxError, LtxFileId, LtxSourceMap, LtxSpan,
};
use std::sync::Arc;

/// Collects diagnostics produced during lexing.
///
/// Wraps an [`LtxDiagnosticSink`] and an [`Arc<LtxSourceMap>`] so that
/// factory methods can construct fully renderable [`LtxDiagnostic`]s
/// without the caller needing to juggle the source map.
#[derive(Debug)]
pub struct LexerErrorHandler {
    sink: LtxDiagnosticSink,
    file_id: LtxFileId,
    source_map: Arc<LtxSourceMap>,
}

impl LexerErrorHandler {
    /// Create a new error handler for the given file.
    #[must_use]
    #[inline]
    pub fn new(file_id: LtxFileId, source_map: Arc<LtxSourceMap>) -> Self {
        Self {
            sink: LtxDiagnosticSink::new(),
            file_id,
            source_map,
        }
    }

    /// Push a pre-built diagnostic directly.
    #[inline]
    pub fn push_diagnostic(&mut self, diagnostic: LtxDiagnostic) {
        self.sink.push(diagnostic);
    }

    /// Returns `true` if any error-severity diagnostics have been collected.
    #[must_use]
    #[inline]
    pub const fn has_errors(&self) -> bool {
        self.sink.has_error()
    }

    /// Number of error-severity diagnostics.
    #[must_use]
    #[inline]
    pub fn error_count(&self) -> usize {
        self.sink
            .get_by_severity(ltx_diagnostics::LtxSeverity::Error)
            .len()
    }

    /// Total number of diagnostics (errors + warnings + hints).
    #[must_use]
    #[inline]
    pub fn total_count(&self) -> usize {
        self.sink.len()
    }

    /// Returns `true` if no diagnostics have been collected.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.sink.is_empty()
    }

    /// Drain all diagnostics, sorted errors-first.
    pub fn take_diagnostics(&mut self) -> Vec<LtxDiagnostic> {
        std::mem::take(&mut self.sink).drain_sorted()
    }

    /// Renders all collected diagnostics to a pretty-printed string.
    #[must_use]
    pub fn render_pretty(&self) -> String {
        self.sink.render_pretty()
    }

    /// The file these diagnostics belong to.
    #[must_use]
    #[inline]
    pub const fn file_id(&self) -> LtxFileId {
        self.file_id
    }

    /// A reference to the source map used for span resolution.
    #[must_use]
    #[inline]
    pub const fn source_map(&self) -> &Arc<LtxSourceMap> {
        &self.source_map
    }

    /// a helper method for init the span
    #[inline]
    #[must_use]
    pub const fn span(&self, start: usize, end: usize) -> LtxSpan {
        LtxSpan::new(start, end, self.file_id)
    }

    /// helper method for pushing errors to `LtxDiagnostic`
    #[inline]
    pub fn push_error(&mut self, error: LtxError) {
        self.sink
            .push(LtxDiagnostic::new(error, self.source_map.clone()));
    }
}
