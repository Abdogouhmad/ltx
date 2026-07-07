/// A modular for the `lexer` errors
pub mod lexer;

/// A modular for the `parser` errors
pub mod parser;

// Re-export
pub use lexer::LexerError;
pub use parser::ParserError;
