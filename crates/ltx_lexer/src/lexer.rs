//! The lexer for ltx cli

use ltx_diagnostics::LtxFileId;

use crate::{LtxCatCodeState, LtxMode, LexerErrorHandler};

/// The lexer for the LTX language.
#[derive(Debug)]
pub struct LtxLexer<'source> {
    /// The source code to lex.
    source: &'source str,

    /// The current cursor position in the source code.
    cursor: usize,

    /// The file id of the source code.
    file_id: LtxFileId,

    /// the catcode of the current character.
    catcode: LtxCatCodeState,

    /// mode of the lexer.
    mode: LtxMode,

    /// the error handler for the lexer.
    error_handler: LexerErrorHandler,
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
}
