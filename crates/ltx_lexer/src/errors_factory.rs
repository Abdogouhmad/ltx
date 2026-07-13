//! Error-code factory methods for [`LexerErrorHandler`].
//!
//! Organized by code range:
//! - **E0xx** — syntax / tokenization
//! - **E1xx** — structural / semantic
//! - **parser-phase** helpers (convenience wrappers for the parser)

use crate::LexerErrorHandler;
use ltx_diagnostics::LtxError;

impl LexerErrorHandler {
    // ── E0xx — syntax / tokenization ────────────────────────────────

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

    /// `LTX::E009` — A `\begin{verbatim}` has no matching `\end{verbatim}`.
    #[inline]
    pub fn unterminated_verbatim(&mut self, start: usize, end: usize) {
        self.push_error(LtxError::UnterminatedVerbatim {
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

    // ── E1xx — structural / semantic ─────────────────────────────────

    /// `LTX::E100` — Command used but never defined.
    #[inline]
    pub fn undefined_control_sequence(&mut self, name: &str, start: usize, end: usize) {
        self.push_error(LtxError::UndefinedControlSequence {
            name: name.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E101` — `\end{found}` doesn't match the open `\begin{expected}`.
    #[inline]
    pub fn mismatched_environment(
        &mut self,
        expected: &str,
        found: &str,
        start: usize,
        end: usize,
    ) {
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

    /// `LTX::E103` — `\begin{env}` names an undefined environment.
    #[inline]
    pub fn undefined_environment(&mut self, name: &str, start: usize, end: usize) {
        self.push_error(LtxError::UndefinedEnvironment {
            name: name.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E104` — `\ref`/`\cite` points to a label that was never defined.
    #[inline]
    pub fn undefined_reference(&mut self, key: &str, start: usize, end: usize) {
        self.push_error(LtxError::UndefinedReference {
            key: key.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E105` — Command requires a package that was never loaded.
    #[inline]
    pub fn missing_package(&mut self, command: &str, package: &str, start: usize, end: usize) {
        self.push_error(LtxError::MissingPackage {
            command: command.to_string().into(),
            package: package.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E106` — `\input`/`\include`/`\includegraphics` references a missing file.
    #[inline]
    pub fn file_not_found(&mut self, path: &str, start: usize, end: usize) {
        self.push_error(LtxError::FileNotFound {
            path: path.to_string().into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E107` — `&` alignment character outside an alignment environment.
    #[inline]
    pub fn misplaced_alignment_tab(&mut self, start: usize, end: usize) {
        self.push_error(LtxError::MisplacedAlignmentTab {
            span: self.span(start, end),
        });
    }

    /// `LTX::E108` — `\newcommand` used for an already-defined command.
    #[inline]
    pub fn command_already_defined(&mut self, name: &str, start: usize, end: usize) {
        self.push_error(LtxError::CommandAlreadyDefined {
            name: name.to_string().into(),
            span: self.span(start, end),
        });
    }

    // ── parser-phase helpers ─────────────────────────────────────────

    /// `LTX::E001` — Expected a specific token, found something else.
    #[inline]
    pub fn expected_token(&mut self, expected: &str, found: &str, start: usize, end: usize) {
        self.push_error(LtxError::UnexpectedToken {
            found: format!("{expected} (found `{found}`)").into(),
            span: self.span(start, end),
        });
    }

    /// `LTX::E002` — Unexpected end of file while parsing a construct.
    #[inline]
    pub fn unexpected_eof_parsing(&mut self, construct: &str, start: usize) {
        self.push_error(LtxError::UnexpectedEOF {
            found: construct.to_string().into(),
            span: self.span(start, start),
        });
    }

    /// `LTX::E005` — Unterminated argument (missing closing `}`).
    #[inline]
    pub fn missing_closing_brace(&mut self, start: usize) {
        self.push_error(LtxError::UnterminatedArgument {
            span: self.span(start, start),
        });
    }
}
