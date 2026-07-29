//! Token definitions produced by the lexer.

use std::borrow::Cow;
use std::fmt;

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

/// The category/kind of a single token.
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

impl fmt::Display for MathDelimiter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dollar => f.write_str("$"),
            Self::DoubleDollar => f.write_str("$$"),
        }
    }
}

impl fmt::Display for LtxTokenKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DocumentClass(name) => write!(f, "\\documentclass{{{name}}}"),
            Self::Command(name) => write!(f, "\\{name}"),
            Self::BeginEnv(name) => write!(f, "\\begin{{{name}}}"),
            Self::EndEnv(name) => write!(f, "\\end{{{name}}}"),
            Self::Text => f.write_str("text"),
            Self::MathStart(delim) => write!(f, "math start ({delim})"),
            Self::MathEnd(delim) => write!(f, "math end ({delim})"),
            Self::Comment => f.write_str("comment"),
            Self::GroupStart => f.write_str("{"),
            Self::GroupEnd => f.write_str("}"),
            Self::WhiteSpace => f.write_str("whitespace"),
            Self::EndOfLine => f.write_str("end of line"),
            Self::Error(msg) => write!(f, "error: {msg}"),
        }
    }
}
