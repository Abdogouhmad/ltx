//! The `tokens` module contains the token definitions for the Latex lexer.
use std::borrow::Cow;

use ltx_diagnostics::LtxSpan;

/// Represents a token produced by the Latex lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LtxToken<'token> {
    /// The span of the token in the file.
    pub span: LtxSpan,
    /// The kind of the token.
    pub kind: LtxTokenKind<'token>,
    /// The source text slice for this token (zero-copy).
    pub text: &'token str,
}

/// Represents a token produced by the Latex lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LtxTokenKind<'token> {
    /// Document class: \documentclass{...}
    DocumentClass(&'token str),
    /// Control sequence: \LaTeX, \section, etc.
    Command(&'token str),
    /// Begin of an environment: \begin{...}
    BeginEnv(&'token str),
    /// End of an environment: \end{...}
    EndEnv(&'token str),
    /// Regular text
    Text,
    /// Math mode start: $, $$
    MathStart(MathDelimiter),
    /// Math mode end: $, $$
    MathEnd(MathDelimiter),
    /// Verbatim content: \verb|...|
    Verbatim(&'token str),
    /// Start of verbatim (used internally for mode switching)
    VerbatimStart,
    /// Parameter: #1, #2, etc.
    Parameter(&'token str),
    /// Active character: ~
    Active(char),
    /// Comment: %...
    Comment,
    /// Group start: {
    GroupStart,
    /// Group end: }
    GroupEnd,
    /// Whitespace (single space)
    WhiteSpace,
    /// End of line
    EndOfLine,
    /// Escape sequence: \$, \(, etc.
    Escape,
    /// Error token (still may need owned String for dynamic messages)
    Error(Cow<'static, str>),
}

/// Represents the delimiter used in math mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathDelimiter {
    /// Single dollar sign: $...$
    Dollar,
    /// Double dollar sign: $$...$$
    DoubleDollar,
}
