//! The `tokens` module contains the token definitions for the Latex lexer.
use ltx_diagnostics::LtxSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a token produced by the Latex lexer.
pub struct LtxToken<'source> {
    /// The span of the token in the file.
    pub span: LtxSpan,
    /// The kind of the token.
    pub kind: LtxTokenKind,
    /// The source text slice for this token.
    pub text: &'source str,
}

/// Represents a token produced by the Latex lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LtxTokenKind {
    /// Document class: \documentclass
    DocumentClass(String),
    /// Control sequence: \LaTeX, \section, etc.
    Command(String),
    /// Begin of an env
    BeginEnv(String),
    /// End of an env
    EndEnv(String),
    /// Regular text
    Text,
    /// Math mode content: $...$
    InlineMathStart(MathDelimiter),
    /// Math mode content: $...$
    InlineMathEnd(MathDelimiter),
    /// Verbatim content: \verb|...|
    Verbatim(String),
    /// Start of verbatim (used internally for mode switching)
    VerbatimStart,
    /// Parameter: #1, #2, etc.
    Parameter(String),
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
    /// Escape sequence: \$
    Escape,
    /// Error token
    Error(String),
}

/// Represents the delimiter used in math mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathDelimiter {
    /// Single dollar sign: $...$
    Dollar,
    /// Double dollar sign: $$...$$
    DoubleDollar,
}
