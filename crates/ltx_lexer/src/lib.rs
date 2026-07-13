//! Byte-level tokenizer for LaTeX source files.
//!
//! Converts raw `.tex` text into a stream of [`LtxToken`]s, each carrying a
//! [`LtxTokenKind`], a [`LtxSpan`], and the source text slice it was parsed
//! from. The lexer is **mode-aware** (Normal / Math) and tracks catcode state
//! per the TeX specification.
//!
//! # Entry points
//!
//! | Type | Role |
//! |------|------|
//! | [`LtxLexer`] | Streaming iterator — call `.next()` or `.next_token()` to get one token at a time. |
//! | [`TokenStream`] | Eagerly drains `LtxLexer` and provides a cursor API (`peek`, `bump`, `checkpoint`/`rewind`) for the parser. |
//!
//! Most consumers should create a [`TokenStream`] and pass it to the parser;
//! only the streaming lexer itself needs `LtxLexer` directly.

/// Token definitions: [`LtxToken`], [`LtxTokenKind`], and [`MathDelimiter`].
pub mod token;

/// Lexer operating modes: [`LtxMode`] (Normal / Math).
pub mod mode;

/// Core lexer: [`LtxLexer`] — the streaming tokenizer.
pub mod lexer;

/// Helper / utility methods for [`LtxLexer`] (scanning, environment tracking, etc.).
pub mod lexer_utils;

/// TeX category codes: [`LtxCatCode`] and the lookup table [`LtxCatCodeState`].
pub mod catcode;

/// Error collection during lexing: [`LexerErrorHandler`].
pub mod errors_core;

/// Error-code factory methods for [`LexerErrorHandler`].
pub mod errors_factory;

/// Fully-tokenized cursor: [`TokenStream`].
pub mod stream;

// re-exports
pub use catcode::{LtxCatCode, LtxCatCodeState};
pub use errors_core::LexerErrorHandler;
pub use lexer::LtxLexer;
pub use mode::LtxMode;
pub use stream::TokenStream;
pub use token::{LtxToken, LtxTokenKind, MathDelimiter};
