//! The `tokens` module contains the token definitions for the Latex lexer.

use ltx_diagnostics::LtxSpan;


#[derive(Debug, Clone, PartialEq)]
/// Represents a token produced by the Latex lexer.
pub struct LtxToken {
    /// The span of the token in the file.
    pub span: LtxSpan,
    /// The kind of the token.
    pub kind: LtxTokenKind,
    /// Represents the text of `main.tex`.
    pub text: String,
}

/// Represents a token produced by the Latex lexer.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum LtxTokenKind {
    /// Document class: \documentclass
    DocumentClass(String) = 1,
    /// Control sequence: \LaTeX, \section, etc.
    Command(String) = 2,
    /// Begin of an env
    BeginEnv(String) = 3,
    /// End of an env
    EndEnv(String) = 4,
    /// Regular text
    Text(String) = 5,
    /// Math mode content: $...$
    Math(String) = 6,
    /// Comment: %...
    Comment = 7,
    /// Group start: {
    GroupStart = 8,
    /// Group end: }
    GroupEnd = 9,
    /// Whitespace (single space)
    Space = 10,
    /// End of file
    Eof = 11,
    /// Error token
    Error(String) = 12,
}
