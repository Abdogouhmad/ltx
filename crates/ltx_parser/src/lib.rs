//! Recursive-descent parser over [`ltx_lexer`] token streams.

pub mod ast;
pub mod parser;
pub mod parser_traits;

// re-export
pub use parser::LtxParser;
pub use parser_traits::Parse;
