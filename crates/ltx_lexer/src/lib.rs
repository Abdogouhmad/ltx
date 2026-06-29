//! The `ltx_lexer` crate provides a lexer for the Latex language.

/// The `Token` enum represents the tokens produced by the lexer.
pub mod token;

/// The mode of the latex {Normal, Math, Verbatim}
pub mod mode;

/// lexer of the Latex language.
pub mod lexer;

/// catcode of latex.
pub mod catcode;

/// Error core for the lexer which calls the `ltx_diagnostic` crate.
pub mod errors_core;

// re-exports
pub use catcode::{LtxCatCode, LtxCatCodeState};
pub use mode::LtxMode;
pub use token::{LtxToken, LtxTokenKind};
pub use lexer::LtxLexer;
pub use errors_core::LexerErrorHandler;
