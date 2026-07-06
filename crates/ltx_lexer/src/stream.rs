//! token_stream.rs
//! A cursor over a fully-tokenized source: arbitrary lookahead,
//! checkpoint/rewind for backtracking, and trivia skipping.
//! This is the type the parser should actually hold — not `LtxLexer` directly.

use crate::{LexerErrorHandler, LtxLexer, LtxToken, LtxTokenKind};

/// A cursor over a fully-tokenized source.
///
/// `TokenStream` eagerly drains an [`LtxLexer`] into a `Vec<LtxToken>` and
/// exposes a cursor-style API (`peek`, `bump`, `checkpoint`/`rewind`) suited
/// to recursive-descent parsing. Parsers should hold a `TokenStream`, not an
/// `LtxLexer`, so they never need to think about lexer state (mode, catcode,
/// env stack) directly.
pub struct TokenStream<'lxr> {
    /// All tokens produced by the lexer, materialized up front.
    tokens: Vec<LtxToken<'lxr>>,
    /// Index into `tokens` of the next token to be read.
    pos: usize,
    /// carried over from the lexer so diagnostics  survive past tokenization
    error: LexerErrorHandler,
}

impl<'lxr> TokenStream<'lxr> {
    /// Eagerly drains the lexer. One lifetime (`'lxr`) to carry through
    /// the whole parser instead of juggling `Option<Token<'_>>` per call.
    #[must_use]
    #[inline]
    pub fn new(mut lexer: LtxLexer<'lxr>) -> Self {
        let tokens = lexer.by_ref().collect();
        Self {
            tokens,
            pos: 0,
            error: lexer.error_handler,
        }
    }

    /// Peek at the current token (0 tokens ahead) without consuming it.
    ///
    /// Returns `None` at end of stream. Equivalent to `peek_at(0)`.
    #[must_use]
    #[inline]
    pub fn peek(&self) -> Option<&LtxToken<'lxr>> {
        self.peek_at(0)
    }

    /// Peek `n` tokens ahead of the cursor without consuming anything.
    ///
    /// `peek_at(0)` is the current token, `peek_at(1)` is the next one, etc.
    /// Returns `None` if `pos + n` runs past the end of the stream.
    #[must_use]
    #[inline]
    pub fn peek_at(&self, n: usize) -> Option<&LtxToken<'lxr>> {
        self.tokens.get(self.pos + n)
    }

    /// Peek at the current token's `kind`, without consuming it.
    ///
    /// Shorthand for `peek().map(|t| &t.kind)` — the thing most parser
    /// dispatch code (`match stream.peek_kind() { ... }`) actually wants.
    #[must_use]
    #[inline]
    pub fn peek_kind(&self) -> Option<&LtxTokenKind<'lxr>> {
        self.peek().map(|t| &t.kind)
    }

    /// Consume and return the current token, advancing the cursor by one.
    ///
    /// Returns `None` (and leaves `pos` unchanged) if already at end of stream.
    #[inline]
    #[must_use]
    pub fn bump(&mut self) -> Option<&LtxToken<'lxr>> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            self.tokens.get(self.pos - 1)
        } else {
            None
        }
    }

    /// `true` if the cursor has consumed every token in the stream.
    #[must_use]
    #[inline]
    pub fn at_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Save the current cursor position for speculative parsing.
    ///
    /// Pair with [`rewind`](Self::rewind) to try a parse and back out if it
    /// fails, without needing to clone or re-lex anything.
    #[must_use]
    #[inline]
    pub const fn checkpoint(&self) -> usize {
        self.pos
    }

    /// Restore the cursor to a position previously returned by
    /// [`checkpoint`](Self::checkpoint), discarding any tokens consumed since.
    #[inline]
    pub const fn rewind(&mut self, checkpoint: usize) {
        self.pos = checkpoint;
    }

    /// Advance the cursor past any run of `WhiteSpace`/`EndOfLine` tokens.
    ///
    /// `Comment` tokens are deliberately *not* skipped here, so a caller
    /// that wants to attach doc-comments to the following node can still
    /// see them sitting in the stream.
    #[inline]
    pub fn skip_ws(&mut self) {
        while matches!(
            self.peek_kind(),
            Some(LtxTokenKind::WhiteSpace | LtxTokenKind::EndOfLine)
        ) {
            self.pos += 1;
        }
    }

    /// All not-yet-consumed tokens, from the cursor to the end of the stream.
    ///
    /// Useful for diagnostics ("here's what's left") or debug assertions;
    /// not meant for hot-path parsing.
    #[must_use]
    pub fn rest(&self) -> &[LtxToken<'lxr>] {
        &self.tokens[self.pos..]
    }

    /// Access the error handler collected during lexing.
    #[must_use]
    #[inline]
    pub const fn error_handler(&self) -> &LexerErrorHandler {
        &self.error
    }

    /// Mutable access — needed to call `take_diagnostics()`.
    #[must_use]
    #[inline]
    pub const fn error_handler_mut(&mut self) -> &mut LexerErrorHandler {
        &mut self.error
    }
}
