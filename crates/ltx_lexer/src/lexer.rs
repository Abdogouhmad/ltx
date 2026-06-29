//! The lexer for ltx cli


use ltx_diagnostics::LtxFileId;

use crate::{LexerErrorHandler, LtxCatCode, LtxCatCodeState, LtxMode, LtxToken, LtxTokenKind};

/// The lexer for the LTX language.
#[derive(Debug)]
pub struct LtxLexer<'source> {
    /// The source code to lex.
    pub source: &'source str,

    /// The current cursor position in the source code.
    pub cursor: usize,

    /// The file id of the source code.
    pub file_id: LtxFileId,

    /// the catcode of the current character.
    pub catcode: LtxCatCodeState,

    /// mode of the lexer.
    pub mode: LtxMode,

    /// the error handler for the lexer.
    pub error_handler: LexerErrorHandler,
}

impl<'source> LtxLexer<'source> {
    /// Creates a new `LtxLexer` with the given source code, file id, and source map.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code to lex.
    /// * `file_id` - The file id of the source code.
    /// * `source_map` - The source map for the lexer.
    #[inline]
    #[must_use]
    pub fn new(
        source: &'source str,
        file_id: LtxFileId,
        source_map: ltx_diagnostics::LtxSourceMap,
    ) -> Self {
        let source_map_arc = std::sync::Arc::new(source_map);
        Self {
            source,
            cursor: 0,
            file_id,
            catcode: LtxCatCodeState::default(),
            mode: LtxMode::default(),
            error_handler: LexerErrorHandler::new(file_id, source_map_arc),
        }
    }

    // --------- Helper methods --------------- //
    /// Returns Boolean of the EOF.
    #[inline]
    #[must_use]
    pub const fn is_eof(&self) -> bool {
        self.cursor >= self.source.len()
    }

    /// Peek method that returns the character at the given offset without advancing the cursor.
    #[inline]
    #[must_use]
    pub fn peek_nth(&self, offset: usize) -> Option<char> {
        self.source[self.cursor..].chars().nth(offset)
    }

    /// Peek method that returns the character at the current cursor position without advancing the cursor.
    #[inline]
    #[must_use]
    pub fn peek(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    /// bump method
    #[inline]
    #[must_use]
    pub fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.cursor += ch.len_utf8();
        Some(ch)
    }

    /// Return the span based on Fileid and cursor position.
    #[inline]
    #[must_use]
    pub const fn lexer_span(&self, start: usize) -> ltx_diagnostics::LtxSpan {
        ltx_diagnostics::LtxSpan::new(start, self.cursor, self.file_id)
    }

    /// get the current cursor position.
    #[inline]
    #[must_use]
    pub const fn current_cursor(&self) -> usize {
        self.cursor
    }

    /// slice helper
    ///
    /// # Arguments
    ///
    /// * `start` - The starting cursor position.
    /// * `end` - The ending cursor position.
    ///
    /// # Returns
    ///
    /// The source text as a `&'source str` slice.
    ///
    /// # Panics
    ///
    /// Panics if `start` is greater than `end` or `end` is greater than the source length.
    #[inline]
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> &'source str {
        &self.source[start..end]
    }

    /// get the consumed source text up to the current cursor position.
    #[inline]
    #[must_use]
    fn consumed_source_text(&self, start: usize) -> String {
        self.source[start..self.current_cursor()].to_string()
    }
    // --------- end of Helper methods --------------- //

    /// Scan a whitespace character and advance the cursor.
    #[inline]
    #[must_use]
    pub fn scan_whitespace(&mut self) -> LtxToken {
        let starting_cursor = self.cursor;
        while let Some(ch) = self.peek() {
            if self.catcode.get(ch) == LtxCatCode::WhiteSpace {
               let _ = self.bump();
            } else {
                break;
            }
        }
        let sp  = self.lexer_span(starting_cursor);
        let txt = self.consumed_source_text(starting_cursor);
        LtxToken {
            span: sp,
            kind: LtxTokenKind::WhiteSpace,
            text: txt,
        }
    }

    /// Scan an EOL character and advance the cursor.
    #[inline]
    #[must_use]
    pub fn scan_eol(&mut self) -> LtxToken {
        let start = self.cursor;
        // Consume \r\n or \n or \r
        if let Some('\r') = self.peek() {
            let _ = self.bump();
            if let Some('\n') = self.peek() {
                let _ = self.bump();
            }
        } else if let Some('\n') = self.peek() {
            let _ = self.bump();
        }
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::EOL,
            span,
            text,
        }
    }

    /// Scan comment and advance the cursor.
    #[inline]
    #[must_use]
    pub fn scan_comment(&mut self) -> LtxToken {
        let start = self.cursor;
        let _ = self.bump();
        while let Some(ch) = self.peek() {
            if self.catcode.get(ch) == LtxCatCode::EndOfLine {
                break;
            }
            let _ = self.bump();
        }
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::Comment,
            span,
            text,
        }
    }
}
