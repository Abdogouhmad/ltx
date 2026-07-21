//! Recursive-descent parser for LaTeX.
//!
//! Consumes a [`ltx_lexer::TokenStream`] and produces an AST.
//!
//! # Architecture
//!
//! - [`parser::LtxParser`] wraps a `TokenStream` and exposes cursor methods
//!   (`peek`, `bump`, `checkpoint`/`rewind`, `skip_ws`).
//! - [`parser_traits::Parse`] is the trait that every AST node implements.
//! - [`ast`] contains the node types (`Document`, `Command`, `Environment`, `Math`, `Group`, `Text`, …).
//! - [`parse_document`] top-level convenience entry point.
//!
//! The parser delegates error reporting to the lexer's
//! [`LexerErrorHandler`](ltx_lexer::LexerErrorHandler), which collects
//! [`LtxDiagnostic`](ltx_diagnostics::LtxDiagnostic) instances that can be
//! rendered after parsing completes.

pub mod ast;
pub mod parser;
pub mod parser_traits;
pub mod utils;

// re-exports
pub use parser::{LtxParser, parse_document};
pub use parser_traits::Parse;
