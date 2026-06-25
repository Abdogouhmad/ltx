//! The `tokens` module contains the token definitions for the Latex lexer.

/// Represents a token produced by the Latex lexer.
pub enum LtxToken {
    /// Represents a regular Latex command (e.g. `\section`).
    Command,
    /// Represents a regular Latex environment (e.g. `\begin{document}`).
    Environment,
    /// Represents a regular Latex macro (e.g. `\newcommand`).
    Macro,
    /// Represents a regular Latex string (e.g. `Hello, World!`).
    String,
    /// Represents a regular Latex number (e.g. `1.23`).
    Number,
    /// Represents a regular Latex symbol (e.g. `$`, `%`, `&`).
    Symbol,
    /// Represents a regular Latex comment (e.g. `% This is a comment`).
    Comment,
    /// Represents a regular Latex whitespace (e.g. ` `, `\t`, `\n`).
    Whitespace,
}
