//! Unified error definitions for the parser.
//!
//! The parser owns every diagnostic it produces while building the AST:
//! missing tokens, unclosed environments, mismatched `\end` names, and so
//! on. Each variant carries its own help text, label, and a namespaced error
//! code of the form `LTX::PARSER::E0xx`.
//!
//! Diagnostics are collected through [`ParserErrorHandler`], which pairs a
//! [`ParserError`] with the [`LtxSourceMap`] needed to render it and pushes
//! the result into an [`LtxDiagnosticSink`]. The lexer's diagnostics are
//! absorbed into the same handler when an [`LtxParser`](crate::LtxParser)
//! is constructed, so a single sink reports both phases.
//!
//! # Code ranges
//!
//! - `LTX::PARSER::E001` – `E006` — structural parsing errors (expected
//!   tokens, environments, groups, math delimiters).

use std::sync::Arc;

use ltx_diagnostics::{
    ErrorCode, LtxDiagnostic, LtxDiagnosticSink, LtxDiagnosticSource, LtxFileId, LtxSeverity,
    LtxSourceMap, LtxSpan,
};
use miette::Diagnostic;
use thiserror::Error;

/// All registered parser diagnostic codes.
///
/// Each entry maps a code (`LTX::PARSER::E0xx`) to its description, default
/// severity, and owning phase (`"parser"`).
pub const ALL_CODES: &[ErrorCode] = &[
    ErrorCode::new("LTX::PARSER::E001", "Expected Token", "error", "parser"),
    ErrorCode::new(
        "LTX::PARSER::E002",
        "Unexpected End of File",
        "error",
        "parser",
    ),
    ErrorCode::new(
        "LTX::PARSER::E003",
        "Unclosed Environment",
        "error",
        "parser",
    ),
    ErrorCode::new(
        "LTX::PARSER::E004",
        "Mismatched Environment",
        "error",
        "parser",
    ),
    ErrorCode::new(
        "LTX::PARSER::E005",
        "Missing Closing Brace",
        "error",
        "parser",
    ),
    ErrorCode::new(
        "LTX::PARSER::E006",
        "Unexpected End of File While Parsing",
        "error",
        "parser",
    ),
];

/// All diagnosable errors produced by the parser.
///
/// Implements [`LtxDiagnosticSource`] so it can flow through the shared
/// [`LtxDiagnosticSink`] and render with miette.
#[derive(Debug, Diagnostic, Error, Clone)]
#[non_exhaustive]
pub enum ParserError {
    /// **`LTX::PARSER::E001`: Expected Token**
    ///
    /// The parser expected a particular token (e.g. `{`, `\begin{...}`) at
    /// the cursor position but found something else or hit end of file.
    #[error("expected {ctx}, found `{found}`")]
    #[diagnostic(
        code(LTX::PARSER::E001),
        help("Check the syntax near the highlighted position — a token is missing or misplaced."),
        url("https://tex.stackexchange.com/search?q=missing+%7B+inserted"),
        severity(Error)
    )]
    ExpectedToken {
        /// Description of what was expected (e.g. `{` or `environment`).
        ctx: String,
        /// The token text that was actually found.
        found: String,
        /// Location of the unexpected token.
        #[label("expected {ctx}")]
        span: LtxSpan,
    },

    /// **`LTX::PARSER::E002`: Unexpected End of File**
    ///
    /// A construct being parsed (e.g. inline math) ran to the end of file
    /// without its closing delimiter.
    #[error("unexpected end of file while parsing `{found}`")]
    #[diagnostic(
        code(LTX::PARSER::E002),
        help("Close the construct with its matching delimiter before the file end."),
        url("https://tex.stackexchange.com/search?q=unexpected+end+of+file"),
        severity(Error)
    )]
    UnexpectedEOF {
        /// The construct that was left open.
        found: String,
        /// Location where the open construct began.
        #[label("unexpected end of file")]
        span: LtxSpan,
    },

    /// **`LTX::PARSER::E003`: Unclosed Environment**
    ///
    /// A `\begin{env}` has no matching `\end{env}` before the end of file.
    #[error("environment `{name}` was not closed")]
    #[diagnostic(
        code(LTX::PARSER::E003),
        help("Add a matching `\\end{{{name}}}` to close the environment."),
        url("https://tex.stackexchange.com/search?q=environment+never+closed"),
        severity(Error)
    )]
    UnclosedEnvironment {
        /// The name of the environment that was left open.
        name: String,
        /// Location of the `\begin{...}`.
        #[label("unclosed environment")]
        span: LtxSpan,
    },

    /// **`LTX::PARSER::E004`: Mismatched Environment**
    ///
    /// A `\end{found}` closes an environment opened as `\begin{expected}`
    /// with a different name.
    #[error("mismatched environment: expected `\\end{{{expected}}}`, found `\\end{{{found}}}`")]
    #[diagnostic(
        code(LTX::PARSER::E004),
        help("Environments must be closed with the same name they were opened with."),
        url("https://tex.stackexchange.com/search?q=environment+ended+by"),
        severity(Error)
    )]
    MismatchedEnvironment {
        /// The name of the environment that was actually opened.
        expected: String,
        /// The name given in the mismatched `\end{...}`.
        found: String,
        /// Location of the mismatched `\end`.
        #[label("mismatched \\end")]
        span: LtxSpan,
    },

    /// **`LTX::PARSER::E005`: Missing Closing Brace**
    ///
    /// A braced group `{ ... ` reached the end of the token stream without
    /// a matching `}`.
    #[error("missing closing brace `}}` for the group opened here")]
    #[diagnostic(
        code(LTX::PARSER::E005),
        help("Add the matching closing brace `}}` to terminate the group."),
        url("https://tex.stackexchange.com/search?q=missing+%7D+inserted"),
        severity(Error)
    )]
    MissingClosingBrace {
        /// Location of the opening `{`.
        #[label("group opened here")]
        span: LtxSpan,
    },

    /// **`LTX::PARSER::E006`: Unexpected End of File While Parsing**
    ///
    /// The top-level document parsing ended before a required structure
    /// (e.g. `\begin{document}`) was found.
    #[error("unexpected end of file while parsing `{what}`")]
    #[diagnostic(
        code(LTX::PARSER::E006),
        help("Ensure the required LaTeX structure (e.g. `\\begin{{document}}`) is present."),
        url("https://tex.stackexchange.com/search?q=file+ended+while+scanning"),
        severity(Error)
    )]
    UnexpectedEOFWhileParsing {
        /// Description of what was being parsed.
        what: String,
        /// Location where parsing expected the structure.
        #[label("expected to find here")]
        span: LtxSpan,
    },
}

impl ParserError {
    /// Extracts the source span from the error.
    ///
    /// # Returns
    ///
    /// The primary [`LtxSpan`] the error points at.
    #[must_use]
    #[inline]
    pub const fn span(&self) -> LtxSpan {
        match self {
            Self::ExpectedToken { span, .. }
            | Self::UnexpectedEOF { span, .. }
            | Self::UnclosedEnvironment { span, .. }
            | Self::MismatchedEnvironment { span, .. }
            | Self::MissingClosingBrace { span, .. }
            | Self::UnexpectedEOFWhileParsing { span, .. } => *span,
        }
    }
}

impl LtxDiagnosticSource for ParserError {
    fn span(&self) -> LtxSpan {
        Self::span(self)
    }
}

/// Collects diagnostics produced during parsing.
///
/// Mirrors the lexer's [`ltx_lexer::LexerErrorHandler`]: it wraps an
/// [`LtxDiagnosticSink`] plus the shared source map, and provides factory
/// methods that build fully renderable [`LtxDiagnostic`]s.
#[derive(Debug)]
pub struct ParserErrorHandler {
    sink: LtxDiagnosticSink,
    file_id: LtxFileId,
    source_map: Arc<LtxSourceMap>,
}

impl ParserErrorHandler {
    /// Creates a new error handler for the given file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file these diagnostics belong to.
    /// * `source_map` - The shared source map used for span resolution.
    ///
    /// # Returns
    ///
    /// A new empty `ParserErrorHandler`.
    #[must_use]
    #[inline]
    pub fn new(file_id: LtxFileId, source_map: Arc<LtxSourceMap>) -> Self {
        Self {
            sink: LtxDiagnosticSink::new(),
            file_id,
            source_map,
        }
    }

    /// Pushes a pre-built diagnostic directly.
    ///
    /// # Arguments
    ///
    /// * `diagnostic` - The diagnostic to collect.
    #[inline]
    pub fn push_diagnostic(&mut self, diagnostic: LtxDiagnostic) {
        self.sink.push(diagnostic);
    }

    /// Pushes any crate-specific error wrapped in an [`LtxDiagnostic`].
    ///
    /// # Arguments
    ///
    /// * `error` - An error implementing [`LtxDiagnosticSource`] (e.g. a
    ///   [`ParserError`]).
    #[inline]
    pub fn push_error<E>(&mut self, error: E)
    where
        E: LtxDiagnosticSource,
    {
        self.sink
            .push(LtxDiagnostic::new(error, self.source_map.clone()));
    }

    /// Absorbs diagnostics produced by another phase (e.g. the lexer).
    ///
    /// # Arguments
    ///
    /// * `diagnostics` - The diagnostics to re-queue in this handler.
    #[inline]
    pub fn extend(&mut self, diagnostics: Vec<LtxDiagnostic>) {
        for diagnostic in diagnostics {
            self.sink.push(diagnostic);
        }
    }

    /// Returns `true` if any error-severity diagnostics have been collected.
    ///
    /// # Returns
    ///
    /// `true` if at least one error-severity diagnostic exists.
    #[must_use]
    #[inline]
    pub const fn has_errors(&self) -> bool {
        self.sink.has_error()
    }

    /// Number of error-severity diagnostics.
    ///
    /// # Returns
    ///
    /// The count of diagnostics whose severity is `Error`.
    #[must_use]
    #[inline]
    pub fn error_count(&self) -> usize {
        self.sink.get_by_severity(LtxSeverity::Error).count()
    }

    /// Total number of diagnostics (errors + warnings + hints).
    ///
    /// # Returns
    ///
    /// The total number of collected diagnostics.
    #[must_use]
    #[inline]
    pub fn total_count(&self) -> usize {
        self.sink.len()
    }

    /// Returns `true` if no diagnostics have been collected.
    ///
    /// # Returns
    ///
    /// `true` if the sink is empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.sink.is_empty()
    }

    /// Drains all diagnostics, sorted errors-first.
    ///
    /// # Returns
    ///
    /// A vector of all collected diagnostics, ordered by severity.
    pub fn take_diagnostics(&mut self) -> Vec<LtxDiagnostic> {
        std::mem::take(&mut self.sink).drain_sorted()
    }

    /// Renders all collected diagnostics to a pretty-printed string.
    ///
    /// # Returns
    ///
    /// The miette graphical rendering, or an empty string on failure.
    #[must_use]
    pub fn render_pretty(&self) -> String {
        self.sink.render_pretty().unwrap_or_default()
    }

    /// The file these diagnostics belong to.
    ///
    /// # Returns
    ///
    /// The [`LtxFileId`] this handler was created with.
    #[must_use]
    #[inline]
    pub const fn file_id(&self) -> LtxFileId {
        self.file_id
    }

    /// A reference to the source map used for span resolution.
    ///
    /// # Returns
    ///
    /// The shared [`LtxSourceMap`] reference.
    #[must_use]
    #[inline]
    pub const fn source_map(&self) -> &Arc<LtxSourceMap> {
        &self.source_map
    }

    /// A helper method to build an [`LtxSpan`] for this file.
    ///
    /// # Arguments
    ///
    /// * `start` - The byte offset of the span start.
    /// * `end` - The byte offset of the span end.
    ///
    /// # Returns
    ///
    /// A span within this handler's file.
    #[inline]
    #[must_use]
    pub const fn span(&self, start: usize, end: usize) -> LtxSpan {
        LtxSpan::new(start, end, self.file_id)
    }
}

impl ParserErrorHandler {
    /// `LTX::PARSER::E001` — A required token was not found.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Description of what was expected (e.g. `{`).
    /// * `found` - The token text actually found.
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn expected_token(&mut self, ctx: &str, found: &str, start: usize, end: usize) {
        self.push_error(ParserError::ExpectedToken {
            ctx: ctx.to_string(),
            found: found.to_string(),
            span: self.span(start, end),
        });
    }

    /// `LTX::PARSER::E002` — A construct ran to the end of file.
    ///
    /// # Arguments
    ///
    /// * `found` - The construct that was left open.
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn unexpected_eof(&mut self, found: &str, start: usize, end: usize) {
        self.push_error(ParserError::UnexpectedEOF {
            found: found.to_string(),
            span: self.span(start, end),
        });
    }

    /// `LTX::PARSER::E003` — A `\begin{env}` has no matching `\end{env}`.
    ///
    /// # Arguments
    ///
    /// * `name` - The environment name that was left open.
    /// * `start` - Byte offset of the `\begin` span start.
    /// * `end` - Byte offset of the `\begin` span end.
    #[inline]
    pub fn unclosed_environment(&mut self, name: &str, start: usize, end: usize) {
        self.push_error(ParserError::UnclosedEnvironment {
            name: name.to_string(),
            span: self.span(start, end),
        });
    }

    /// `LTX::PARSER::E004` — `\end{found}` doesn't match the open `\begin{expected}`.
    ///
    /// # Arguments
    ///
    /// * `expected` - The environment name that was actually opened.
    /// * `found` - The name given in the mismatched `\end{...}`.
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn mismatched_environment(
        &mut self,
        expected: &str,
        found: &str,
        start: usize,
        end: usize,
    ) {
        self.push_error(ParserError::MismatchedEnvironment {
            expected: expected.to_string(),
            found: found.to_string(),
            span: self.span(start, end),
        });
    }

    /// `LTX::PARSER::E005` — A braced group was never closed.
    ///
    /// # Arguments
    ///
    /// * `start` - Byte offset of the opening `{`.
    #[inline]
    pub fn missing_closing_brace(&mut self, start: usize) {
        self.push_error(ParserError::MissingClosingBrace {
            span: self.span(start, start + 1),
        });
    }

    /// `LTX::PARSER::E006` — End of file reached while parsing a structure.
    ///
    /// # Arguments
    ///
    /// * `what` - Description of what was being parsed.
    /// * `pos` - Byte offset where the structure was expected.
    #[inline]
    pub fn unexpected_eof_parsing(&mut self, what: &str, pos: usize) {
        self.push_error(ParserError::UnexpectedEOFWhileParsing {
            what: what.to_string(),
            span: self.span(pos, pos),
        });
    }
}
