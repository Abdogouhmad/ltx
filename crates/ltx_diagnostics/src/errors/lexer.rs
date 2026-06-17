//! A systematic error for Ltx lexer
//!
//! lexer support the following errors code `LTX::E001 -- LTX::E099`

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

// Assuming these are defined elsewhere in your crate
// use crate::{LtxSeverity, LtxSpan};

/// enum error for lexer
#[derive(Debug, Diagnostic, Error, Clone)]
pub enum LexerError {
    /// Represents an unexpected token that was found during parsing.
    #[error("unexpected token `{found}`")]
    #[diagnostic(
        code(LTX::E001),
        help(
            "Check for invalid characters, malformed commands, or unsupported syntax near the highlighted position."
        ),
        severity(Error)
    )]
    UnexpectedToken {
        /// The token that was found.
        found: String,
        #[label("unexpected token")]
        /// The span of the token that was found.
        span: SourceSpan,
        #[source_code]
        /// The source code that the token was found in.
        src: miette::NamedSource<String>,
    },

    /// Represents an Unexpected end of file that was found during parsing.
    #[error("unexpected end of file `{found}`")]
    #[diagnostic(
        code(LTX::E002),
        help("Ensure all environments, braces, and command arguments are properly closed."),
        severity(Error)
    )]
    UnexpectedEOF {
        /// Represents what was being looked for or found.
        found: String,
        #[label("unexpected end of file")]
        /// The span of the token that was found.
        span: SourceSpan,
        #[source_code]
        /// The source code that the token was found in.
        src: miette::NamedSource<String>,
    },

    /// Represents Unmatched brace detected that was found during parsing.
    #[error("Unmatched brace detected. `{found}`")]
    #[diagnostic(
        code(LTX::E003),
        // Fixed: Escaped curly braces by doubling them contextually
        help("Verify that every \\{{ has a matching \\}}."), 
        severity(Error)
    )]
    UnmatchedBrace {
        /// Represents the unmatched brace details.
        found: String,
        #[label("unmatched brace")] // Fixed: Changed from "unexpected end of file"
        /// The span of the token that was found.
        span: SourceSpan,
        #[source_code]
        /// The source code that the token was found in.
        src: miette::NamedSource<String>,
    },

    /// Represents Invalid math delimiter detected that was found during parsing.
    #[error("Invalid math delimiter detected. `{found}`")]
    #[diagnostic(
        code(LTX::E004),
        help("Verify correct usage of $$, $$$, \\(, \\), \\[, and \\]."),
        severity(Error)
    )]
    InvalidMathDelimiter {
        /// Represents the invalid delimiter found.
        found: String,
        #[label("Invalid math delimiter detected.")]
        /// The span of the token that was found.
        span: SourceSpan,
        #[source_code]
        /// The source code that the token was found in.
        src: miette::NamedSource<String>,
    },
}
