//! The lexer for ltx cli

use ltx_diagnostics::LtxFileId;

use crate::{
    LexerErrorHandler, LtxCatCode, LtxCatCodeState, LtxMode, LtxToken, LtxTokenKind,
    token::MathDelimiter,
};

/// The lexer for the LTX language.
#[derive(Debug)]
pub struct LtxLexer<'lxr> {
    /// The source code to lex.
    pub source: &'lxr str,

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

impl<'lxr> LtxLexer<'lxr> {
    /// Creates a new `LtxLexer` with the given source code, file id, and source map.
    #[inline]
    #[must_use]
    pub fn new(
        source: &'lxr str,
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

    /// slice helper (zero-copy)
    #[inline]
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> &'lxr str {
        &self.source[start..end]
    }

    /// get the consumed source text as a zero-copy slice
    #[inline]
    #[must_use]
    fn consumed_source_text(&self, start: usize) -> &'lxr str {
        &self.source[start..self.current_cursor()]
    }
    // --------- end of Helper methods --------------- //

    // ------------ Start of helper CORE LEXING METHODS --------------- //

    /// Scan a whitespace character and advance the cursor.
    #[inline]
    #[must_use]
    pub fn scan_whitespace(&mut self) -> LtxToken<'lxr> {
        let starting_cursor = self.cursor;
        while let Some(ch) = self.peek() {
            if self.catcode.get(ch) == LtxCatCode::WhiteSpace {
                let _ = self.bump();
            } else {
                break;
            }
        }
        let sp = self.lexer_span(starting_cursor);
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
    pub fn scan_eol(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        // Consume \r\n or \n or \r
        if self.peek() == Some('\r') {
            let _ = self.bump();
            if self.peek() == Some('\n') {
                let _ = self.bump();
            }
        } else if self.peek() == Some('\n') {
            let _ = self.bump();
        }
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::EndOfLine,
            span,
            text,
        }
    }

    /// Scan comment and advance the cursor.
    #[inline]
    #[must_use]
    pub fn scan_comment(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        let _ = self.bump();
        while let Some(ch) = self.peek() {
            if ch == '\n' {
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

    /// scan begin and end of a group
    #[inline]
    #[must_use]
    pub fn scan_group_start(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        let _ = self.bump();
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::GroupStart,
            span,
            text,
        }
    }

    /// scan end of a group
    #[inline]
    #[must_use]
    pub fn scan_group_end(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        let _ = self.bump();
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::GroupEnd,
            span,
            text,
        }
    }

    /// scan Escape sequence (consumes `\` + following character)
    #[inline]
    #[must_use]
    pub fn scan_escape(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        let _ = self.bump(); // consume '\'
        // Consume the escaped character if it exists
        if self.peek().is_some() {
            let _ = self.bump();
        }
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::Escape,
            span,
            text,
        }
    }

    /// Scan a `$` or `$$` and produce either `InlineMathStart` or `InlineMathEnd`.
    #[inline]
    #[must_use]
    pub fn scan_math_shift(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;

        let _ = self.bump(); // consume first `$`
        let delimiter = if self.peek() == Some('$') {
            let _ = self.bump(); // consume second `$`
            MathDelimiter::DoubleDollar
        } else {
            MathDelimiter::Dollar
        };

        let kind = if self.mode == LtxMode::Math {
            self.mode = LtxMode::Normal;
            LtxTokenKind::InlineMathEnd(delimiter)
        } else {
            self.mode = LtxMode::Math;
            LtxTokenKind::InlineMathStart(delimiter)
        };

        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken { span, kind, text }
    }

    /// Scan text content until hitting a special character.
    #[inline]
    #[must_use]
    pub fn scan_text(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        while let Some(ch) = self.peek() {
            let cat = self.catcode.get(ch);
            match cat {
                LtxCatCode::Escape
                | LtxCatCode::GroupStart
                | LtxCatCode::GroupEnd
                | LtxCatCode::MathShift
                | LtxCatCode::Comment
                | LtxCatCode::WhiteSpace
                | LtxCatCode::EndOfLine => break,
                _ => {
                    let _ = self.bump();
                }
            }
        }
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::Text,
            span,
            text,
        }
    }

    // --------- end of helper CORE LEXING METHODS --------------- //
}
