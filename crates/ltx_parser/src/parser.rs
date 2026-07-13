//! The main parser struct — wraps a [`TokenStream`] and drives [`Parse`] impls.

use crate::parser_traits::Parse;
use ltx_lexer::{LtxToken, LtxTokenKind, TokenStream};

/// Parser state machine over a fully-tokenized source.
///
/// Wraps a [`TokenStream`] (not `Peekable`) because the stream already
/// provides lookahead via `peek()`/`peek_at()`, backtracking via
/// `checkpoint()`/`rewind()`, and trivia-skipping via `skip_ws()`.
pub struct LtxParser<'src> {
    /// The underlying token stream.
    pub stream: TokenStream<'src>,
}

// ===== Low-level cursor methods (delegated to TokenStream) =====

impl<'src> LtxParser<'src> {
    /// Create a new parser that drains the given `TokenStream`.
    #[inline]
    #[must_use]
    pub const fn new(stream: TokenStream<'src>) -> Self {
        Self { stream }
    }

    /// Peek at the current token kind without consuming it.
    #[inline]
    #[must_use]
    pub fn peek_kind(&self) -> Option<&LtxTokenKind<'src>> {
        self.stream.peek_kind()
    }

    /// Peek `n` tokens ahead.
    #[inline]
    #[must_use]
    pub fn peek_at(&self, n: usize) -> Option<&LtxToken<'src>> {
        self.stream.peek_at(n)
    }

    /// Consume and return the current token.
    #[inline]
    pub fn bump(&mut self) -> Option<&LtxToken<'src>> {
        self.stream.bump()
    }

    /// Save the current cursor position for backtracking.
    #[inline]
    #[must_use]
    pub const fn checkpoint(&self) -> usize {
        self.stream.checkpoint()
    }

    /// Restore to a previous checkpoint.
    #[inline]
    pub const fn rewind(&mut self, checkpoint: usize) {
        self.stream.rewind(checkpoint);
    }

    /// Skip past `WhiteSpace` and `EndOfLine` tokens.
    #[inline]
    pub fn skip_ws(&mut self) {
        self.stream.skip_ws();
    }

    /// True if all tokens have been consumed.
    #[inline]
    #[must_use]
    pub fn at_eof(&self) -> bool {
        self.stream.at_eof()
    }

    /// Mutable access to the error handler for pushing diagnostics.
    #[inline]
    #[must_use]
    pub fn error_handler_mut(&mut self) -> &mut ltx_lexer::LexerErrorHandler {
        self.stream.error_stream_mut()
    }

    /// Access the error handler (read-only).
    #[inline]
    #[must_use]
    pub fn error_handler(&self) -> &ltx_lexer::LexerErrorHandler {
        self.stream.error_stream()
    }
}

// ===== Convenience helpers that reduce boilerplate in `Parse` impls =====

impl<'src> LtxParser<'src> {
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
    #[inline]
    pub fn accept(&mut self, f: impl FnOnce(&LtxTokenKind<'src>) -> bool) -> bool {
        if self.peek_kind().is_some_and(f) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Expect the current token to satisfy `f`, consume it, and return it.
    ///
    /// On mismatch: emits an `LTX::E001` diagnostic and skips forward until
    /// a command, group start, or EOF is reached (error recovery).
    /// Returns `None` instead of panicking.
    pub fn expect(
        &mut self,
        ctx: &str,
        f: impl FnOnce(&LtxTokenKind<'src>) -> bool,
    ) -> Option<&LtxToken<'src>> {
        if self.peek_kind().is_some_and(f) {
            self.bump()
        } else {
            let (found, span_start, span_end) = match self.stream.peek() {
                Some(tok) => {
                    let name = format!("{:?}", tok.kind);
                    (name, tok.span.start(), tok.span.end())
                }
                None => ("EOF".to_string(), self.stream.checkpoint(), self.stream.checkpoint()),
            };
            self.error_handler_mut()
                .expected_token(ctx, &found, span_start, span_end);
            self.skip_to_boundary();
            None
        }
    }

    /// Skip tokens until a recovery point: a `Command`, `GroupStart`, or EOF.
    ///
    /// Used after emitting a diagnostic to avoid cascading errors.
    fn skip_to_boundary(&mut self) {
        loop {
            match self.peek_kind() {
                None | Some(LtxTokenKind::Command(_) | LtxTokenKind::GroupStart) => break,
                Some(LtxTokenKind::GroupEnd) => break,
                _ => {
                    self.bump();
                }
            }
        }
    }
}
