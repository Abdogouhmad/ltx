//! implementation of ltx parser

use ltx_lexer::TokenStream;

/// Parser state machine over a tokenized source.
///
/// Wraps a [`TokenStream`] (not `Peekable`) because `TokenStream` already
/// provides lookahead via `peek()`/`peek_at()`, backtracking via
/// `checkpoint()`/`rewind()`, and trivia-skipping via `skip_ws()`.
pub struct LtxParser<'src> {
    pub stream: TokenStream<'src>,
}

impl<'src> LtxParser<'src> {
    /// Create a new parser that drains the given `TokenStream`.
    #[inline]
    #[must_use]
    pub const fn new(stream: TokenStream<'src>) -> Self {
        Self { stream }
    }
}
