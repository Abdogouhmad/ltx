//! Unified error definitions for the lexer.
//!
//! The lexer owns every diagnostic it can produce during tokenization:
//! stray tokens, unbalanced braces, malformed escapes, and so on. Each
//! variant carries its own help text, label, and a namespaced error code
//! of the form `LTX::LEXER::E0xx`.
//!
//! Diagnostics are collected through [`LexerErrorHandler`], which pairs a
//! [`LexerError`] with the [`ltx_diagnostics::LtxSourceMap`] needed to render
//! it and pushes the result into an [`ltx_diagnostics::LtxDiagnosticSink`].
//!
//! # Code ranges
//!
//! - `LTX::LEXER::E001` – `E010` — tokenization errors (braces, delimiters,
//!   escapes, encoding).
//! - `LTX::LEXER::E011` — environment mismatch detected during scanning.

use std::borrow::Cow;
use std::sync::Arc;

use ltx_diagnostics::{
    ErrorCode, LtxDiagnostic, LtxDiagnosticSink, LtxDiagnosticSource, LtxFileId, LtxSeverity,
    LtxSourceMap, LtxSpan,
};
use miette::Diagnostic;
use thiserror::Error;

/// All registered lexer diagnostic codes.
///
/// Each entry maps a code (`LTX::LEXER::E0xx`) to its description, default
/// severity, and owning phase (`"lexer"`).
pub const ALL_CODES: &[ErrorCode] = &[
    ErrorCode::new("LTX::LEXER::E001", "Unexpected Token", "error", "lexer"),
    ErrorCode::new(
        "LTX::LEXER::E002",
        "Unexpected End of File",
        "error",
        "lexer",
    ),
    ErrorCode::new("LTX::LEXER::E003", "Unmatched Brace", "error", "lexer"),
    ErrorCode::new(
        "LTX::LEXER::E004",
        "Invalid Math Delimiter",
        "error",
        "lexer",
    ),
    ErrorCode::new(
        "LTX::LEXER::E005",
        "Unterminated Argument",
        "error",
        "lexer",
    ),
    ErrorCode::new(
        "LTX::LEXER::E006",
        "Invalid Escape Sequence",
        "error",
        "lexer",
    ),
    ErrorCode::new("LTX::LEXER::E007", "Invalid Unicode", "error", "lexer"),
    ErrorCode::new(
        "LTX::LEXER::E008",
        "Illegal Parameter Character Usage",
        "error",
        "lexer",
    ),
    ErrorCode::new(
        "LTX::LEXER::E009",
        "Unterminated Verbatim Block",
        "error",
        "lexer",
    ),
    ErrorCode::new("LTX::LEXER::E010", "Invalid Character", "error", "lexer"),
    ErrorCode::new(
        "LTX::LEXER::E011",
        "Mismatched Environment",
        "error",
        "lexer",
    ),
];

/// All diagnosable errors produced by the lexer.
///
/// Implements [`LtxDiagnosticSource`] so it can flow through the shared
/// [`LtxDiagnosticSink`] and render with miette.
#[derive(Debug, Diagnostic, Error, Clone)]
#[non_exhaustive]
pub enum LexerError {
    /// **`LTX::LEXER::E001`: Unexpected Token**
    ///
    /// A token appeared where the grammar didn't expect one — e.g. a stray
    /// symbol in a position reserved for a command name or argument.
    #[error("unexpected token `{found}`")]
    #[diagnostic(
        code(LTX::LEXER::E001),
        help(
            "Check for invalid characters, malformed commands, or unsupported syntax near the highlighted position."
        ),
        url("https://tex.stackexchange.com/search?q=unexpected+token"),
        severity(Error)
    )]
    UnexpectedToken {
        /// The textual representation of the offending token.
        found: Cow<'static, str>,
        /// Location of the unexpected token.
        #[label("unexpected token")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E002`: Unexpected End of File**
    ///
    /// The file ended while a construct (environment, argument, math mode)
    /// was still open.
    #[error("unexpected end of file reached while parsing `{found}`")]
    #[diagnostic(
        code(LTX::LEXER::E002),
        help("Ensure all environments, braces, and command arguments are properly closed."),
        url("https://tex.stackexchange.com/search?q=file+ended+while+scanning"),
        severity(Error)
    )]
    UnexpectedEOF {
        /// Description of what structural component was left open.
        found: Cow<'static, str>,
        /// Location of the premature end-of-file.
        #[label("unexpected end of file")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E003`: Unmatched Brace**
    ///
    /// A `{` or `}` has no matching counterpart.
    #[error("unmatched brace detected: `{found}`")]
    #[diagnostic(
        code(LTX::LEXER::E003),
        help("Verify that every opening brace `{{` has a matching closing brace `}}`."),
        url("https://tex.stackexchange.com/search?q=too+many+%7D%27s"),
        severity(Error)
    )]
    UnmatchedBrace {
        /// The stray brace or surrounding text fragment.
        found: Cow<'static, str>,
        /// Location of the unmatched brace.
        #[label("unmatched brace")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E004`: Invalid Math Delimiter**
    ///
    /// Math-mode delimiters (`$`, `$$`, `\(`, `\)`, `\[`, `\]`) are
    /// malformed, mismatched, or nested illegally.
    #[error("invalid math delimiter detected: `{found}`")]
    #[diagnostic(
        code(LTX::LEXER::E004),
        help("Verify correct usage of $$, $, \\\\(, \\\\), \\\\[ and \\\\]."),
        url("https://tex.stackexchange.com/search?q=missing+%24+inserted"),
        severity(Error)
    )]
    InvalidMathDelimiter {
        /// The malformed delimiter sequence.
        found: Cow<'static, str>,
        /// Location of the broken math delimiter.
        #[label("invalid delimiter")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E005`: Unterminated Argument**
    ///
    /// A macro argument (`\section{...`) was opened but never closed
    /// before the surrounding scope ended.
    #[error("command argument not terminated")]
    #[diagnostic(
        code(LTX::LEXER::E005),
        help("Add the missing closing brace `}}` to complete the macro argument."),
        url("https://tex.stackexchange.com/search?q=argument+of+has+an+extra"),
        severity(Error)
    )]
    UnterminatedArgument {
        /// Location where the unclosed argument began.
        #[label("unterminated argument")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E006`: Invalid Escape Sequence**
    ///
    /// A backslash (`\`) is followed by characters that don't form a valid
    /// command name or primitive symbol.
    #[error("invalid escape sequence")]
    #[diagnostic(
        code(LTX::LEXER::E006),
        help("Verify the command name directly following the backslash `\\`."),
        url("https://tex.stackexchange.com/search?q=undefined+control+sequence"),
        severity(Error)
    )]
    InvalidEscapeSequence {
        /// Location of the malformed escape sequence.
        #[label("invalid escape sequence")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E007`: Invalid Unicode**
    ///
    /// The source bytes contain corrupted or non-UTF-8 sequences, often
    /// from files saved in a legacy encoding (e.g. Latin-1).
    #[error("invalid UTF-8 sequence detected")]
    #[diagnostic(
        code(LTX::LEXER::E007),
        help("Ensure your document is saved with UTF-8 encoding."),
        url("https://tex.stackexchange.com/search?q=inputenc+utf8"),
        severity(Error)
    )]
    InvalidUnicode {
        /// Location of the invalid byte sequence.
        #[label("invalid UTF-8 sequence")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E008`: Illegal Parameter Character Usage**
    ///
    /// A raw `#` appears outside of a macro definition's parameter list,
    /// where it must instead be escaped as `\#`.
    #[error("illegal parameter character usage")]
    #[diagnostic(
        code(LTX::LEXER::E008),
        help(
            "Verify proper use of `\\#` inside body contexts, or arguments within macro definitions."
        ),
        url("https://tex.stackexchange.com/search?q=you+can%27t+use+macro+parameter+character"),
        severity(Error)
    )]
    IllegalParameterChar {
        /// Location of the stray `#`.
        #[label("illegal parameter character")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E009`: Unterminated Verbatim Block**
    ///
    /// A `\begin{verbatim}` (or similar raw-text environment) has no
    /// matching `\end{verbatim}` before the file ends.
    #[error("verbatim environment was not terminated")]
    #[diagnostic(
        code(LTX::LEXER::E009),
        help(
            "Close the verbatim block explicitly using an `\\end{{verbatim}}` marker before the file end."
        ),
        url("https://tex.stackexchange.com/search?q=verbatim+environment+not+closed"),
        severity(Error)
    )]
    UnterminatedVerbatim {
        /// Location where the verbatim block was opened.
        #[label("unterminated environment")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E010`: Invalid Character**
    ///
    /// An unsupported low-level or non-printable control byte appears in
    /// the source, often from copy-pasted text with hidden characters.
    #[error("invalid character encountered: `{found}`")]
    #[diagnostic(
        code(LTX::LEXER::E010),
        help("Remove or replace unsupported tokens or invisible control characters."),
        url("https://tex.stackexchange.com/search?q=invalid+character"),
        severity(Error)
    )]
    InvalidCharacter {
        /// Display form of the offending character.
        found: Cow<'static, str>,
        /// Location of the invalid character.
        #[label("invalid character")]
        span: LtxSpan,
    },

    /// **`LTX::LEXER::E011`: Mismatched Environment**
    ///
    /// A `\begin{env}` was closed by a `\end{other}` with a different name,
    /// or an `\end` appeared with no corresponding `\begin`.
    #[error("mismatched environment: expected `\\end{{{expected}}}`, found `\\end{{{found}}}`")]
    #[diagnostic(
        code(LTX::LEXER::E011),
        help("Environments must be closed with the same name they were opened with."),
        url("https://tex.stackexchange.com/search?q=environment+ended+by"),
        severity(Error)
    )]
    MismatchedEnvironment {
        /// The name of the environment that was actually opened.
        expected: Cow<'static, str>,
        /// The name given in the mismatched `\end{...}`.
        found: Cow<'static, str>,
        /// Location of the mismatched `\end`.
        #[label("mismatched \\end")]
        span: LtxSpan,
    },
}

impl LexerError {
    /// Extracts the source span from the error.
    ///
    /// # Returns
    ///
    /// The primary [`LtxSpan`] the error points at.
    #[must_use]
    #[inline]
    pub const fn span(&self) -> LtxSpan {
        match self {
            Self::UnexpectedToken { span, .. }
            | Self::UnexpectedEOF { span, .. }
            | Self::UnmatchedBrace { span, .. }
            | Self::InvalidMathDelimiter { span, .. }
            | Self::UnterminatedArgument { span, .. }
            | Self::InvalidEscapeSequence { span, .. }
            | Self::InvalidUnicode { span, .. }
            | Self::IllegalParameterChar { span, .. }
            | Self::UnterminatedVerbatim { span, .. }
            | Self::InvalidCharacter { span, .. }
            | Self::MismatchedEnvironment { span, .. } => *span,
        }
    }
}

impl LtxDiagnosticSource for LexerError {
    fn span(&self) -> LtxSpan {
        Self::span(self)
    }
}

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
    /// Creates a new error handler for the given file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file these diagnostics belong to.
    /// * `source_map` - The shared source map used for span resolution.
    ///
    /// # Returns
    ///
    /// A new empty `LexerErrorHandler`.
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
    ///   [`LexerError`] or a `ltx_parser::ParserError`).
    #[inline]
    pub fn push_error<E>(&mut self, error: E)
    where
        E: LtxDiagnosticSource,
    {
        self.sink
            .push(LtxDiagnostic::new(error, self.source_map.clone()));
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

impl LexerErrorHandler {
    // ── LTX::LEXER::E001 — syntax / tokenization ─────────────────────

    /// `LTX::LEXER::E001` — A token appeared where the grammar didn't expect one.
    ///
    /// # Arguments
    ///
    /// * `found` - The offending character.
    /// * `start` - Byte offset of the token start.
    /// * `end` - Byte offset of the token end.
    #[inline]
    pub fn unexpected_token(&mut self, found: char, start: usize, end: usize) {
        self.push_error(LexerError::UnexpectedToken {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E002` — File ended while a construct was still open.
    ///
    /// # Arguments
    ///
    /// * `found` - The construct that was left open.
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn unexpected_eof(&mut self, found: &str, start: usize, end: usize) {
        self.push_error(LexerError::UnexpectedEOF {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E003` — A `{` or `}` has no matching counterpart.
    ///
    /// # Arguments
    ///
    /// * `found` - The stray brace character.
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn unmatched_brace(&mut self, found: char, start: usize, end: usize) {
        self.push_error(LexerError::UnmatchedBrace {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E004` — Math-mode delimiters are malformed or mismatched.
    ///
    /// # Arguments
    ///
    /// * `found` - The malformed delimiter sequence.
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn invalid_math_delimiter(&mut self, found: &str, start: usize, end: usize) {
        self.push_error(LexerError::InvalidMathDelimiter {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E005` — A macro argument was opened but never closed.
    ///
    /// # Arguments
    ///
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn unterminated_argument(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::UnterminatedArgument {
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E006` — Backslash followed by an invalid command name.
    ///
    /// # Arguments
    ///
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn invalid_escape_sequence(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::InvalidEscapeSequence {
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E007` — Source contains non-UTF-8 bytes.
    ///
    /// # Arguments
    ///
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn invalid_unicode(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::InvalidUnicode {
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E008` — Raw `#` outside a macro definition.
    ///
    /// # Arguments
    ///
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn illegal_parameter_char(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::IllegalParameterChar {
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E009` — A `\begin{verbatim}` has no matching `\end{verbatim}`.
    ///
    /// # Arguments
    ///
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn unterminated_verbatim(&mut self, start: usize, end: usize) {
        self.push_error(LexerError::UnterminatedVerbatim {
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E010` — Unsupported or invisible control byte.
    ///
    /// # Arguments
    ///
    /// * `found` - The offending character.
    /// * `start` - Byte offset of the span start.
    /// * `end` - Byte offset of the span end.
    #[inline]
    pub fn invalid_character(&mut self, found: char, start: usize, end: usize) {
        self.push_error(LexerError::InvalidCharacter {
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::LEXER::E011` — `\end{found}` doesn't match the open `\begin{expected}`.
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
        self.push_error(LexerError::MismatchedEnvironment {
            expected: expected.to_string().into(),
            found: found.to_string().into(),
            span: self.span(start, end),
        });
    }
}
