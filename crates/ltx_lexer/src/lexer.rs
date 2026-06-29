//! The lexer for ltx cli

use ltx_diagnostics::LtxFileId;

use crate::{LtxCatCodeState, LtxMode, LexerErrorHandler};

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
    pub fn new(source: &'source str, file_id: LtxFileId, source_map: ltx_diagnostics::LtxSourceMap) -> Self {
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

    /// Returns Boolean of the EOF.
    #[inline]
    #[must_use]
    pub fn is_eof(&self) -> bool {
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
}
