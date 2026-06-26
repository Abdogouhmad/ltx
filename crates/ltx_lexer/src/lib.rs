//! The `ltx_lexer` crate provides a lexer for the Latex language.


/// The `Token` enum represents the tokens produced by the lexer.
pub mod token;

/// The mode of the latex {Normal, Math, Verbatim}
pub mod mode;

/// lexer of the Latex language.
pub mod lexer;


// re-exports
pub use token::{LtxTokenKind, LtxToken};
pub use mode::LtxMode;
