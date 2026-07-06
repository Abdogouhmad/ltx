//! The lexer for ltx cli

use ltx_diagnostics::LtxFileId;

use crate::{LexerErrorHandler, LtxCatCode, LtxCatCodeState, LtxMode, LtxToken};

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

    /// Stack of currently open environments for matching \begin and \end
    pub(crate) env_stack: Vec<&'lxr str>,
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
            env_stack: Vec::new(),
        }
    }

    /// main tokenizer for normal text
    #[inline]
    #[must_use]
    fn next_normal_token(&mut self) -> Option<LtxToken<'lxr>> {
        if self.is_eof() {
            return None;
        }
        let cat = self.catcode.get(self.peek()?);

        let normal_token = match cat {
            LtxCatCode::Escape => self.scan_command(),
            LtxCatCode::GroupStart => self.scan_group_start(),
            LtxCatCode::GroupEnd => self.scan_group_end(),
            LtxCatCode::MathShift => self.scan_math_shift(),
            LtxCatCode::WhiteSpace => self.scan_whitespace(),
            LtxCatCode::EndOfLine => self.scan_eol(),
            LtxCatCode::Comment => self.scan_comment(),
            _ => self.scan_text(),
        };
        Some(normal_token)
    }

    /// main tokenizer for math mode
    #[inline]
    #[must_use]
    fn next_math_token(&mut self) -> Option<LtxToken<'lxr>> {
        if self.is_eof() {
            return None;
        }

        let cat = self.catcode.get(self.peek()?);

        let token = match cat {
            // Exit math mode
            LtxCatCode::MathShift => self.scan_math_shift(),

            // Commands like \frac, \alpha, ...
            LtxCatCode::Escape => self.scan_command(),

            // Groups
            LtxCatCode::GroupStart => self.scan_group_start(),
            LtxCatCode::GroupEnd => self.scan_group_end(),

            // Spaces
            LtxCatCode::WhiteSpace => self.scan_whitespace(),
            LtxCatCode::EndOfLine => self.scan_eol(),

            // Comments
            LtxCatCode::Comment => self.scan_comment(),

            // Everything else for now
            _ => self.scan_text(),
        };

        Some(token)
    }

    /// The main mod dispatcher
    #[inline]
    #[must_use]
    pub fn next_token(&mut self) -> Option<LtxToken<'lxr>> {
        match self.mode {
            LtxMode::Normal => self.next_normal_token(),
            LtxMode::Math => self.next_math_token(),
        }
    }
}

/// iterator impl for ltx lexer
impl<'lxr> Iterator for LtxLexer<'lxr> {
    type Item = LtxToken<'lxr>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.source.len() - self.cursor;
        (0, Some(remaining))
    }
}
