//! an implementation of Ltx Parser

/// main parser entry point
pub mod parser;

/// A helper modular helped to implement parser
pub mod parser_utils;

/// Trait for managing states of the parser
pub mod parser_traits;

/// AST implementation
pub mod ast;

// re-exports
pub use parser::LtxParser;
pub use parser_traits::Parse;
