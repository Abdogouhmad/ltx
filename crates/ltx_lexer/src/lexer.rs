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

    // ------------ Start of helper CORE LEXING METHODS --------------- //

    /// Scan a whitespace character and advance the cursor.
    #[inline]
    #[must_use]
    pub fn scan_whitespace(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        while let Some(ch) = self.peek() {
            if self.catcode.get(ch) == LtxCatCode::WhiteSpace {
                let _ = self.bump();
            } else {
                break;
            }
        }
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::WhiteSpace,
            span,
            text,
        }
    }

    /// Scan an EOL character and advance the cursor.
    #[inline]
    #[must_use]
    pub fn scan_eol(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
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
        let _ = self.bump();
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

    /// Scan a `$` or `$$` and produce either `MathStart` or `MathEnd`.
    #[inline]
    #[must_use]
    pub fn scan_math_shift(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        let _ = self.bump();
        let delimiter = if self.peek() == Some('$') {
            let _ = self.bump();
            MathDelimiter::DoubleDollar
        } else {
            MathDelimiter::Dollar
        };

        let kind = if self.mode == LtxMode::Math {
            self.mode = LtxMode::Normal;
            LtxTokenKind::MathEnd(delimiter)
        } else {
            self.mode = LtxMode::Math;
            LtxTokenKind::MathStart(delimiter)
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

    /// Scan commands with environment validation using `LexerErrorHandler`
    #[inline]
    #[must_use]
    pub fn scan_command(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        let _ = self.bump(); // consume '\'

        let cmd_start = self.cursor;

        // Check if it's a control word (letters) or control symbol (single char)
        if let Some(ch) = self.peek() {
            if self.catcode.is_letter(ch) {
                // Control word: \LaTeX, \section, etc.
                while let Some(ch) = self.peek() {
                    if self.catcode.is_letter(ch) {
                        let _ = self.bump();
                    } else {
                        break;
                    }
                }
                let cmd_name = self.slice(cmd_start, self.cursor);

                return match cmd_name {
                    "documentclass" => self.scan_documentclass(start),
                    "begin" => self.scan_begin(start),
                    "end" => self.scan_end(start),
                    _ => self.normal_cmd(start, cmd_name),
                };
            }
            // Control symbol: \$, \%, etc.
            let _ = self.bump();
            let sym = self.slice(cmd_start, self.cursor);
            let span = self.lexer_span(start);
            let text = self.consumed_source_text(start);
            return LtxToken {
                kind: LtxTokenKind::Command(sym),
                span,
                text,
            };
        }

        // Lone backslash at EOF
        self.error_handler
            .invalid_escape_sequence(start, self.cursor);
        self.error_token(start, "Lone backslash")
    }

    // --------- end of helper CORE LEXING METHODS --------------- //
}
