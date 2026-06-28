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
