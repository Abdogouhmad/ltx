//! Error handling core for the lexer
//!
//! This module provides helper functions for creating and emitting
//! diagnostic errors from the lexer.

use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticInner, LtxFileId, LtxSourceMap, LtxSpan};
use ltx_diagnostics::errors::LexerError;
use ltx_diagnostics::LtxDiagnosticSink;
use std::sync::Arc;

/// Core error handling functionality for the lexer
///
/// This collects errors during lexing and can convert them
/// to proper `LtxDiagnostic` instances for rendering.
#[derive(Debug)]
pub struct LexerErrorCore {
    /// Diagnostic sink for collecting errors
    sink: LtxDiagnosticSink,
    /// Current file ID
    file_id: LtxFileId,
    /// Source map for creating diagnostics
    source_map: Arc<LtxSourceMap>,
}

impl LexerErrorCore {
    /// Create a new error core
    pub fn new(file_id: LtxFileId, source_map: Arc<LtxSourceMap>) -> Self {
        LexerErrorCore {
            sink: LtxDiagnosticSink::new(),
            file_id,
            source_map,
        }
    }

    /// Create a new error core from a mutable source map
    pub fn from_source_map(file_id: LtxFileId, source_map: &mut LtxSourceMap) -> Self {
        LexerErrorCore {
            sink: LtxDiagnosticSink::new(),
            file_id,
            source_map: Arc::new(source_map.clone()),
        }
    }

    /// Add a diagnostic to the sink
    pub fn push_diagnostic(&mut self, diagnostic: LtxDiagnostic) {
        self.sink.push(diagnostic);
    }

    /// Add an error to the collection (converts to diagnostic internally)
    fn push_error(&mut self, error: LexerError) {
        let diagnostic = LtxDiagnostic::new(
            LtxDiagnosticInner::Lexer(error),
            self.source_map.clone(),
        );
        self.sink.push(diagnostic);
    }

    /// Check if there were any errors
    pub fn has_errors(&self) -> bool {
        self.sink.has_error()
    }

    /// Get count of errors
    pub fn len(&self) -> usize {
        self.sink.len()
    }

    /// Check if sink is empty
    pub fn is_empty(&self) -> bool {
        self.sink.is_empty()
    }

    /// Get all diagnostics (sorted by severity)
    pub fn take_diagnostics(&mut self) -> Vec<LtxDiagnostic> {
        let sink = std::mem::take(&mut self.sink);
        sink.drain_sorted()
    }

    /// Get all raw errors (converted from diagnostics)
    pub fn take_errors(&mut self) -> Vec<LexerError> {
        let diagnostics = self.take_diagnostics();
        diagnostics
            .into_iter()
            .filter_map(|diag| match diag.inner {
                LtxDiagnosticInner::Lexer(error) => Some(error),
                _ => None,
            })
            .collect()
    }

    /// Get the file ID
    pub fn file_id(&self) -> LtxFileId {
        self.file_id
    }

    /// Get the source map
    pub fn source_map(&self) -> Arc<LtxSourceMap> {
        self.source_map.clone()
    }

    /// Create a span from start/end
    fn span(&self, start: usize, end: usize) -> LtxSpan {
        LtxSpan::new(start, end, self.file_id)
    }

    // ===== ERROR FACTORY METHODS =====

    /// Unexpected Token: LTX::E001
    pub fn unexpected_token(&mut self, found: char, start: usize, end: usize) {
        self.push_error(LexerError::UnexpectedToken {
            found: found.to_string(),
            span: self.span(start, end),
        });
    }

    /// Unexpected End of File: LTX::E002
    pub fn unexpected_eof(&mut self, found: &str, start: usize, end: usize) {
        self.push_error(LexerError::UnexpectedEOF {
            found: found.to_string(),
            span: self.span(start, end),
        });
    }

    /// Unmatched Brace: LTX::E003
    pub fn unmatched_brace(&mut self, found: char, start: usize, end: usize) {
        self.push_error(LexerError::UnmatchedBrace {
            found: found.to_string(),
            span: self.span(start, end),
        });
    }

    /// Invalid Math Delimiter: LTX::E004
    pub fn invalid_math_delimiter(&mut self, found: &str, start: usize, end: usize) {
        self.push_error(LexerError::InvalidMathDelimiter {
            found: found.to_string(),
            span: self.span(start, end),
        });
    }

    /// Unterminated Argument: LTX::E005
    pub fn unterminated_argument(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::UnterminatedArgument {
            span: self.span(start, end),
        });
    }

    /// Invalid Escape Sequence: LTX::E006
    pub fn invalid_escape_sequence(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::InvalidEscapeSequence {
            span: self.span(start, end),
        });
    }

    /// Invalid Unicode: LTX::E007
    pub fn invalid_unicode(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::InvalidUnicode {
            span: self.span(start, end),
        });
    }

    /// Illegal Parameter Character: LTX::E008
    pub fn illegal_parameter_char(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::IllegalParameterChar {
            span: self.span(start, end),
        });
    }

    /// Unterminated Verbatim: LTX::E009
    pub fn unterminated_verbatim(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::UnterminatedVerbatim {
            span: self.span(start, end),
        });
    }

    /// Invalid Character: LTX::E010
    pub fn invalid_character(&mut self, found: char, start: usize, end: usize) {
        self.push_error(LexerError::InvalidCharacter {
            found: found.to_string(),
            span: self.span(start, end),
        });
    }
}

impl Default for LexerErrorCore {
    fn default() -> Self {
        let source_map = Arc::new(LtxSourceMap::new());
        Self::new(LtxFileId(0), source_map)
    }
}
