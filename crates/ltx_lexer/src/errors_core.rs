//! Error handling for the lexer.
//!
//! [`LexerErrorHandler`] collects [`LtxDiagnostic`]s during tokenization.
//! Factory methods create [`LtxError`] variants at specific spans, wrap them
//! in [`LtxDiagnostic`]s (pairing the error with the [`LtxSourceMap`]), and
//! push them into an internal buffer.

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

    // ── span helper ──────────────────────────────────────────────────

    #[inline]
    const fn span(&self, start: usize, end: usize) -> LtxSpan {
        LtxSpan::new(start, end, self.file_id)
    }

    #[inline]
    fn push_error(&mut self, error: LtxError) {
        self.sink
            .push(LtxDiagnostic::new(error, self.source_map.clone()));
    }

    // ── factory methods (LTX::E0xx — syntax / tokenization) ─────────

    /// `LTX::E001` — A token appeared where the grammar didn't expect one.
    #[inline]
    pub fn unexpected_token(&mut self, found: char, start: usize, end: usize) {
        self.push_error(LtxError::UnexpectedToken {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E002` — File ended while a construct was still open.
    #[inline]
    pub fn unexpected_eof(&mut self, found: &str, start: usize, end: usize) {
        self.push_error(LtxError::UnexpectedEOF {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E003` — A `{` or `}` has no matching counterpart.
    #[inline]
    pub fn unmatched_brace(&mut self, found: char, start: usize, end: usize) {
        self.push_error(LtxError::UnmatchedBrace {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E004` — Math-mode delimiters are malformed or mismatched.
    #[inline]
    pub fn invalid_math_delimiter(&mut self, found: &str, start: usize, end: usize) {
        self.push_error(LtxError::InvalidMathDelimiter {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E005` — A macro argument was opened but never closed.
    #[inline]
    pub fn unterminated_argument(&mut self, start: usize, end: usize) {
        self.push_error(LtxError::UnterminatedArgument {
            span: self.span(start, end),
        });
    }

    /// `LTX::E006` — Backslash followed by an invalid command name.
    #[inline]
    pub fn invalid_escape_sequence(&mut self, start: usize, end: usize) {
        self.push_error(LtxError::InvalidEscapeSequence {
            span: self.span(start, end),
        });
    }

    /// `LTX::E007` — Source contains non-UTF-8 bytes.
    #[inline]
    pub fn invalid_unicode(&mut self, start: usize, end: usize) {
        self.push_error(LtxError::InvalidUnicode {
            span: self.span(start, end),
        });
    }

    /// `LTX::E008` — Raw `#` outside a macro definition.
    #[inline]
    pub fn illegal_parameter_char(&mut self, start: usize, end: usize) {
        self.push_error(LtxError::IllegalParameterChar {
            span: self.span(start, end),
        });
    }

    /// `LTX::E010` — Unsupported or invisible control byte.
    #[inline]
    pub fn invalid_character(&mut self, found: char, start: usize, end: usize) {
        self.push_error(LtxError::InvalidCharacter {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    // ── factory methods (LTX::E1xx — structural / semantic) ─────────

    /// `LTX::E101` — `\end{found}` doesn't match the open `\begin{expected}`.
    #[inline]
    pub fn mismatched_environment(&mut self, expected: &str, found: &str, start: usize, end: usize) {
        self.push_error(LtxError::MismatchedEnvironment {
            expected: expected.to_string().into(),
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E102` — `\begin{env}` was never closed.
    #[inline]
    pub fn unclosed_environment(&mut self, name: &str, start: usize, end: usize) {
        self.push_error(LtxError::UnclosedEnvironment {
            name: name.to_string().into(),
            span: self.span(start, end),
        });
    }
}
