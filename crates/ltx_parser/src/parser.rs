//! implementation of ltx parser

use crate::parser_traits::Parse;
use ltx_lexer::{LtxToken, LtxTokenKind, TokenStream};

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

    // ===== Convenience helpers that reduce boilerplate in `Parse` impls =====

    /// Convenience: parse any `T: Parse` from the current position.
    ///
    /// Equivalent to `T::parse(self)` but lets callers write
    /// `parser.parse::<Command>()` or `let cmd: Command = parser.parse()`.
    #[inline]
    pub fn parse<T: Parse<'src>>(&mut self) -> T {
        T::parse(self)
    }

    /// If the current token satisfies the predicate `f`, consume it and return
    /// `true`.  Otherwise return `false` without advancing.
    ///
    /// `f` receives a reference to the current token's kind and should return
    /// `true` to accept it.
    #[inline]
    pub fn accept(&mut self, f: impl FnOnce(&LtxTokenKind<'src>) -> bool) -> bool {
        if self.peek_kind().map_or(false, |k| f(k)) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Assert that the current token satisfies `f`, consume it, and return a
    /// reference to it.
    ///
    /// # Panics
    ///
    /// Panics if the predicate fails or the stream is at EOF.  In a real
    /// parser this would emit a diagnostic and attempt error recovery; the
    /// panic is a placeholder until the error-recovery layer is added.
    #[inline]
    pub fn expect(&mut self, ctx: &str, f: impl FnOnce(&LtxTokenKind<'src>) -> bool) -> &LtxToken<'src> {
        if self.peek_kind().map_or(false, |k| f(k)) {
            self.bump().expect("expect: bump after peek returned None")
        } else {
            panic!("expected {ctx} at position {}", self.checkpoint());
        }
    }
}
