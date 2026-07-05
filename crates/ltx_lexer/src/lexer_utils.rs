//! Helper / utility methods for `LtxLexer`.
//!
//! Split into a separate module so `lexer.rs` can focus on the
//! core scanning logic.

use ltx_diagnostics::LtxSpan;

use crate::LtxLexer;

impl<'lxr> LtxLexer<'lxr> {
    /// Returns `true` when the cursor has reached (or passed) the end of source.
    #[inline]
    #[must_use]
    pub const fn is_eof(&self) -> bool {
        self.cursor >= self.source.len()
    }

    /// Peek at the Nth character ahead (0‑based) without advancing the cursor.
    #[inline]
    #[must_use]
    pub fn peek_nth(&self, offset: usize) -> Option<char> {
        self.source[self.cursor..].chars().nth(offset)
    }

    /// Peek at the current character without advancing the cursor.
    #[inline]
    #[must_use]
    pub fn peek(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    /// Advance the cursor by one char and return the consumed character.
    #[inline]
    #[must_use]
    pub fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.cursor += ch.len_utf8();
        Some(ch)
    }

    /// Build a `LtxSpan` from a saved start position to the current cursor.
    #[inline]
    #[must_use]
    pub const fn lexer_span(&self, start: usize) -> LtxSpan {
        LtxSpan::new(start, self.cursor, self.file_id)
    }

    /// Return the current byte‑offset cursor.
    #[inline]
    #[must_use]
    pub const fn current_cursor(&self) -> usize {
        self.cursor
    }

    /// Zero‑copy slice of the source.
    #[inline]
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> &'lxr str {
        &self.source[start..end]
    }

    /// get the consumed source text as a zero-copy slice
    #[inline]
    #[must_use]
    pub fn consumed_source_text(&self, start: usize) -> &'lxr str {
        &self.source[start..self.cursor]
    }

    // TODO Scan command helper functions
    // ------------------ scan cmd help func -------------- //
    pub fn scan_doc_class() {
        todo!()
    }

    pub fn scan_begin_ends_env() {
        todo!()
    }
}
