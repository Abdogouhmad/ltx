//! The `ltx_lexer` crate provides a lexer for the Latex language.

/// The `Token` enum represents the tokens produced by the lexer.
pub mod token;

/// The mode of the latex {Normal, Math, Verbatim}
pub mod mode;

/// lexer of the Latex language.
pub mod lexer;

/// catcode of latex.
pub mod catcode;

// re-exports
pub use catcode::{CatCode, CatCodeState};
pub use mode::LtxMode;
pub use token::{LtxToken, LtxTokenKind};
