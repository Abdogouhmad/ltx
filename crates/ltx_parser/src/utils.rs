use ltx_lexer::{LtxToken, LtxTokenKind};

use crate::LtxParser;

impl<'src> LtxParser<'src> {
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
