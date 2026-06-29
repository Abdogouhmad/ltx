//! The `tokens` module contains the token definitions for the Latex lexer.
use ltx_diagnostics::LtxSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Verbatim content: \verb|...|
    Verbatim(String) = 7,
    /// Start of verbatim (used internally for mode switching)
    VerbatimStart = 8,
    /// Parameter: #1, #2, etc.
    Parameter(String) = 9,
    /// Active character: ~
    Active(char) = 10,
    /// Comment: %...
    Comment = 11,
    /// Group start: {
    GroupStart = 12,
    /// Group end: }
    GroupEnd = 13,
    /// Whitespace (single space)
    WhiteSpace = 14,
    /// EOL
    EOL = 15,
    /// Error token
    Error(String) = 16,
}
