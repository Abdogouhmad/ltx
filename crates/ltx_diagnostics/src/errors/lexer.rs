//! A systematic error for Ltx lexer
//!
//! Lexer supports the error codes `LTX::E001 -- LTX::E099`

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Errors encountered during the lexical analysis phase of the Ltx parser.
///
/// This enum categorizes all syntax, character encoding, and structural tokenization
/// failures. Each variant maps directly to an explicit `LTX::E0xx` code and integrates
/// with `miette` to provide contextual, human-readable compiler diagnostics.
#[derive(Debug, Diagnostic, Error, Clone)]
pub enum LexerError {
    /// **`LTX::E001`: Unexpected Token**
    ///
    /// Triggered when the lexer encounters a validly formed token that completely
    /// violates the structural rules of the grammar at its current state.
    ///
    /// *Example:* A misplaced punctuation mark or structural tag where a command
    /// identifier or plain text string payload was expected.
    #[error("unexpected token `{found}`")]
    #[diagnostic(
        code(LTX::E001),
        help(
            "Check for invalid characters, malformed commands, or unsupported syntax near the highlighted position."
        ),
        severity(Error)
    )]
    UnexpectedToken {
        /// The textual representation of the token that violated the grammar rules.
        found: String,
        /// The precise source location bounds of the unexpected token.
        #[label("unexpected token")]
        span: SourceSpan,
        /// The complete source file wrapper containing the faulty token payload.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E002`: Unexpected End of File**
    ///
    /// Triggered when the stream of tokens terminates abruptly while the lexer or
    /// parser is still expecting a closing sequence or finishing token.
    ///
    /// *Example:* Reaching the end of the input file while deep inside an unclosed
    /// environment or expression scope.
    #[error("unexpected end of file reached while parsing `{found}`")]
    #[diagnostic(
        code(LTX::E002),
        help("Ensure all environments, braces, and command arguments are properly closed."),
        severity(Error)
    )]
    UnexpectedEOF {
        /// A string describing what structural component was left dangling or unclosed.
        found: String,
        /// The position in the source code where the EOF was prematurely hit.
        #[label("unexpected end of file")]
        span: SourceSpan,
        /// The complete source file wrapper containing the truncated code string.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E003`: Unmatched Brace**
    ///
    /// Triggered when structural grouping brackets (`{` or `}`) fail to pair correctly.
    /// This happens when a closing brace is found without an opening counterpart,
    /// or when scopes overlap incorrectly.
    ///
    /// *Example:* `\command{text}}` (too many closing braces).
    #[error("unmatched brace detected: `{found}`")]
    #[diagnostic(
        code(LTX::E003),
        help("Verify that every opening brace `{{` has a matching closing brace `}}`."),
        severity(Error)
    )]
    UnmatchedBrace {
        /// The structural grouping detail or the isolated brace character itself.
        found: String,
        /// The location of the stray or non-matching brace character.
        #[label("unmatched brace")]
        span: SourceSpan,
        /// The complete source file wrapper containing the bad syntax token.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **``LTX::E004``: Invalid Math Delimiter**
    ///
    /// Triggered when mathematical equation delimiters are malformed, nested illegally,
    /// or paired with mismatched partners.
    ///
    /// *Example:* Opening an inline math environment with `\(` but attempting to close it with `]`.
    #[error("invalid math delimiter detected: `{found}`")]
    #[diagnostic(
        code(LTX::E004),
        help("Verify correct usage of $$, $, \\(, \\), \\[ and \\]."),
        severity(Error)
    )]
    InvalidMathDelimiter {
        /// The bad sequence string (e.g., `$$$`) or the mismatched delimiter pair encountered.
        found: String,
        /// The span locating the broken math block delimiter.
        #[label("invalid delimiter")]
        span: SourceSpan,
        /// The complete source file wrapper containing the math layout string.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **``LTX::E005``: Unterminated Argument**
    ///
    /// Triggered when an explicit block parameter or argument sequence for a macro
    /// is opened, but the document stream ends or changes structural scope before a
    /// closing token is found.
    ///
    /// *Example:* `\section{Introduction` (missing the final `}`).
    #[error("command argument not terminated")]
    #[diagnostic(
        code(LTX::E005),
        help("Add the missing closing brace `}}` to complete the macro argument."),
        severity(Error)
    )]
    UnterminatedArgument {
        /// The span tracking the opening of the parameter context that was left hanging.
        #[label("unterminated argument")]
        span: SourceSpan,
        /// The complete source file wrapper containing the unclosed macro payload.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **``LTX::E006``: Invalid Escape Sequence**
    ///
    /// Triggered when a backslash (`\`) control character is supplied, but the characters
    /// directly following it do not form a valid macro identifier name or allowed
    /// primitive symbol sequence.
    ///
    /// *Example:* Using numbers, invalid symbols, or empty spaces illegally right after an escape flag.
    #[error("invalid escape sequence")]
    #[diagnostic(
        code(LTX::E006),
        help("Verify the command name directly following the backslash `\\`."),
        severity(Error)
    )]
    InvalidEscapeSequence {
        /// The span mapping to the illegal backslash or malformed macro sequence.
        #[label("invalid escape sequence")]
        span: SourceSpan,
        /// The complete source file wrapper containing the bad escape character.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **``LTX::E007``: Invalid Unicode**
    ///
    /// Triggered if the input payload bytes contain corrupted, incomplete, or
    /// entirely non-compliant UTF-8 character sequences.
    ///
    /// *Example:* This occurs when handling binary data or documents encoded in legacy format
    /// variants (like ISO-8859-1) without translating them to proper UTF-8 strings.
    #[error("invalid UTF-8 sequence detected")]
    #[diagnostic(
        code(LTX::E007),
        help("Ensure your document code syntax matches standard UTF-8 encoding configurations."),
        severity(Error)
    )]
    InvalidUnicode {
        /// The byte boundary range where the invalid character parsing error occurred.
        #[label("invalid UTF-8 sequence")]
        span: SourceSpan,
        /// The raw string reference tracking data context mapping.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **``LTX::E008``: Illegal Parameter Character Usage**
    ///
    /// Triggered when the macro argument designator sequence (`#`) is used in a context
    /// where it isn't lexically allowed.
    ///
    /// *Example:* Utilizing raw `#` values out in standard paragraph text instead of utilizing
    /// escaped `\#` notation, or applying macro indices out-of-bounds.
    #[error("illegal parameter character usage")]
    #[diagnostic(
        code(LTX::E008),
        help(
            "Verify proper use of `\\#` inside body contexts or arguments within macro definitions."
        ),
        severity(Error)
    )]
    IllegalParameterChar {
        /// The exact location span tracking the rogue `#` token occurrence.
        #[label("illegal parameter character")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **``LTX::E009``: Unterminated Verbatim Block**
    ///
    /// Triggered when a raw pre-formatted verbatim text structure block environment
    /// is opened, but the system hits the end of input code before hitting a clean termination command.
    ///
    /// *Example:* Declaring `\begin{verbatim}` without ever typing `\end{verbatim}` before the file closes.
    #[error("verbatim environment was not terminated")]
    #[diagnostic(
        code(LTX::E009),
        help(
            "Close the verbatim block explicitly using an `\\end{{verbatim}}` marker before the file end."
        ),
        severity(Error)
    )]
    UnterminatedVerbatim {
        /// The location sequence monitoring the unclosed verbatim environment boundary block.
        #[label("unterminated environment")]
        span: SourceSpan,
        /// The complete source file wrapper payload.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **``LTX::E010``: Invalid Character**
    ///
    /// Triggered when encountering a totally unsupported or illegal low-level ASCII symbol,
    /// non-printable control byte, or invalid character value.
    ///
    /// *Example:* Stray invisible control characters hidden inside copied text scripts.
    #[error("invalid character encountered: `{found}`")]
    #[diagnostic(
        code(LTX::E010),
        help("Remove or replace unsupported tokens or invisible control characters."),
        severity(Error)
    )]
    InvalidCharacter {
        /// The textual display value representing the rogue/non-printable character detected.
        found: String,
        /// The index location pointing directly to the illegal item.
        #[label("invalid character")]
        span: SourceSpan,
        /// The complete source file wrapper payload.
        #[source_code]
        src: miette::NamedSource<String>,
    },
}


impl LexerError {
    /// Mutates or rebuilds the internal variant payload to attach source code context.
    /// We use an Arc wrapper for `NamedSource` if you want to avoid expensive copies of the source string,
    /// or just stick to standard `NamedSource` if performance isn't an issue.
    #[must_use]
    pub fn with_source(self, span: SourceSpan, src: miette::NamedSource<String>) -> Self {
        match self {
            Self::UnexpectedToken { found, .. } => Self::UnexpectedToken {
                found,
                span,
                src,
            },
            Self::UnexpectedEOF { found, .. } => Self::UnexpectedEOF {
                found,
                span,
                src,
            },
            Self::UnmatchedBrace { found, .. } => Self::UnmatchedBrace {
                found,
                span,
                src,
            },
            Self::InvalidMathDelimiter { found, .. } => Self::InvalidMathDelimiter {
                found,
                span,
                src,
            },
            Self::UnterminatedArgument { .. } => Self::UnterminatedArgument {
                span,
                src,
            },
            Self::InvalidEscapeSequence { .. } => Self::InvalidEscapeSequence {
                span,
                src,
            },
            Self::InvalidUnicode { .. } => Self::InvalidUnicode {
                span,
                src,
            },
            Self::IllegalParameterChar { .. } => Self::IllegalParameterChar {
                span,
                src,
            },
            Self::UnterminatedVerbatim { .. } => Self::UnterminatedVerbatim {
                span,
                src,
            },
            Self::InvalidCharacter { found, .. } => Self::InvalidCharacter {
                found,
                span,
                src,
            },
        }
    }
}
