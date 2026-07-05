//! The lexer for ltx cli

use std::borrow::Cow;

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
    env_stack: Vec<&'lxr str>,
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

    /// Scan a `$` or `$$` and produce either `MathStart` or `MathEnd`.
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

    /// Scan environment name inside `\begin{...}` or `\end{...}`
    /// Returns None if braces are missing or malformed.
    #[inline]
    fn scan_env_name_optional(&mut self) -> Option<&'lxr str> {
        // Expecting {
        if self.peek() != Some('{') {
            return None;
        }
        let _ = self.bump(); // consume {

        let env_start = self.cursor;
        while let Some(ch) = self.peek() {
            if ch == '}' {
                break;
            }
            let _ = self.bump();
        }

        // Check if we found the closing }
        if self.peek() != Some('}') {
            return None;
        }

        let env_name = self.slice(env_start, self.cursor);
        let _ = self.bump(); // consume }
        Some(env_name)
    }
    /// Scan commands with environment validation using `LexerErrorHandler`
    #[inline]
    #[must_use]
    pub fn scan_command(&mut self) -> LtxToken<'lxr> {
        let start = self.cursor;
        let _ = self.bump(); // consume '\'

        let cmd_start = self.cursor;
        let kind;

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

                // Check for special commands
                kind = match cmd_name {
                    "documentclass" => {
                        if let Some(env) = self.scan_env_name_optional() {
                            LtxTokenKind::DocumentClass(env)
                        } else {
                            let span = self.lexer_span(start);
                            self.error_handler
                                .unexpected_token('\\', start, self.cursor);
                            return LtxToken {
                                kind: LtxTokenKind::Error(Cow::Borrowed(
                                    "Expected \\documentclass{...}",
                                )),
                                span,
                                text: self.consumed_source_text(start),
                            };
                        }
                    }
                    "begin" => {
                        if let Some(env) = self.scan_env_name_optional() {
                            // Push environment onto stack
                            self.env_stack.push(env);
                            LtxTokenKind::BeginEnv(env)
                        } else {
                            let span = self.lexer_span(start);
                            self.error_handler.unexpected_token('{', start, self.cursor);
                            return LtxToken {
                                kind: LtxTokenKind::Error(Cow::Borrowed("Expected \\begin{...}")),
                                span,
                                text: self.consumed_source_text(start),
                            };
                        }
                    }
                    "end" => {
                        if let Some(env) = self.scan_env_name_optional() {
                            // Validate matching environment
                            if let Some(&expected) = self.env_stack.last() {
                                if env != expected {
                                    let span = self.lexer_span(start);
                                    self.error_handler.unmatched_brace('}', start, self.cursor);
                                    return LtxToken {
                                        kind: LtxTokenKind::Error(Cow::Owned(format!(
                                            "Mismatched environment: \\end{{{env}}} should match \\begin{{{expected}}}"
                                        ))),
                                        span,
                                        text: self.consumed_source_text(start),
                                    };
                                }
                                // Pop the environment from stack
                                let _ = self.env_stack.pop();
                            } else {
                                let span = self.lexer_span(start);
                                self.error_handler
                                    .unexpected_token('\\', start, self.cursor);
                                return LtxToken {
                                    kind: LtxTokenKind::Error(Cow::Owned(format!(
                                        "\\end{{{env}}} has no matching \\begin"
                                    ))),
                                    span,
                                    text: self.consumed_source_text(start),
                                };
                            }
                            LtxTokenKind::EndEnv(env)
                        } else {
                            let span = self.lexer_span(start);
                            self.error_handler.unexpected_token('{', start, self.cursor);
                            return LtxToken {
                                kind: LtxTokenKind::Error(Cow::Borrowed("Expected \\end{...}")),
                                span,
                                text: self.consumed_source_text(start),
                            };
                        }
                    }
                    _ => LtxTokenKind::Command(cmd_name),
                };
            } else {
                // Control symbol: \$, \%, etc.
                let _ = self.bump(); // consume the symbol
                let sym = self.slice(cmd_start, self.cursor);
                kind = LtxTokenKind::Command(sym);
            }
        } else {
            // Lone backslash at EOF
            let span = self.lexer_span(start);
            self.error_handler
                .invalid_escape_sequence(start, self.cursor);
            return LtxToken {
                kind: LtxTokenKind::Error(Cow::Borrowed("Lone backslash")),
                span,
                text: self.consumed_source_text(start),
            };
        }

        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken { span, kind, text }
    }

    // --------- end of helper CORE LEXING METHODS --------------- //
}
