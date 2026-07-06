//! Helper / utility methods for `LtxLexer`.
//!
//! Split into a separate module so `lexer.rs` can focus on the
//! core scanning logic.

use std::borrow::Cow;

use ltx_diagnostics::LtxSpan;

use crate::{LtxCatCode, LtxLexer, LtxMode, LtxToken, LtxTokenKind, MathDelimiter};

impl<'lxr> LtxLexer<'lxr> {
    // ===== POSITION & NAVIGATION =====

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

    /// Get the consumed source text as a zero-copy slice.
    #[inline]
    #[must_use]
    pub fn consumed_source_text(&self, start: usize) -> &'lxr str {
        &self.source[start..self.cursor]
    }

    // ===== ENVIRONMENT HELPERS =====

    /// Scan environment name inside `\begin{...}` or `\end{...}`
    /// Returns `None` if braces are missing or malformed.
    #[inline]
    pub fn scan_env_name_optional(&mut self) -> Option<&'lxr str> {
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

    /// Get the current environment from the stack (last pushed).
    #[inline]
    #[must_use]
    pub fn current_env(&self) -> Option<&'lxr str> {
        self.env_stack.last().copied()
    }

    /// Push an environment onto the stack.
    #[inline]
    pub fn push_env(&mut self, name: &'lxr str) {
        self.env_stack.push(name);
    }

    /// Pop the last environment from the stack.
    #[inline]
    pub fn pop_env(&mut self) -> Option<&'lxr str> {
        self.env_stack.pop()
    }

    /// Check if the environment stack is empty.
    #[inline]
    #[must_use]
    pub fn env_stack_is_empty(&self) -> bool {
        self.env_stack.is_empty()
    }

    // ===== ERROR HELPERS =====

    /// Create an error token with the given message.
    #[inline]
    pub fn error_token(&mut self, start: usize, msg: &'static str) -> LtxToken<'lxr> {
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::Error(Cow::Borrowed(msg)),
            span,
            text,
        }
    }

    /// Create an error token with an owned error message.
    #[inline]
    pub fn error_token_owned(&mut self, start: usize, msg: String) -> LtxToken<'lxr> {
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::Error(Cow::Owned(msg)),
            span,
            text,
        }
    }

    // ===== COMMAND HELPERS =====

    /// Scan a `\documentclass{...}` command.
    #[inline]
    pub fn scan_documentclass(&mut self, start: usize) -> LtxToken<'lxr> {
        if let Some(env) = self.scan_env_name_optional() {
            let span = self.lexer_span(start);
            let text = self.consumed_source_text(start);
            LtxToken {
                kind: LtxTokenKind::DocumentClass(env),
                span,
                text,
            }
        } else {
            self.error_handler
                .unexpected_token('\\', start, self.cursor);
            self.error_token(start, "Expected \\documentclass{...}")
        }
    }

    /// Scan a `\begin{...}` command.
    #[inline]
    pub fn scan_begin(&mut self, start: usize) -> LtxToken<'lxr> {
        if let Some(env) = self.scan_env_name_optional() {
            self.push_env(env);
            let span = self.lexer_span(start);
            let text = self.consumed_source_text(start);
            LtxToken {
                kind: LtxTokenKind::BeginEnv(env),
                span,
                text,
            }
        } else {
            self.error_handler.unexpected_token('{', start, self.cursor);
            self.error_token(start, "Expected \\begin{...}")
        }
    }

    /// Scan a `\end{...}` command with environment validation.
    #[inline]
    pub fn scan_end(&mut self, start: usize) -> LtxToken<'lxr> {
        if let Some(env) = self.scan_env_name_optional() {
            // Validate matching environment
            if let Some(expected) = self.current_env() {
                if env != expected {
                    let _ = self.pop_env();
                    let msg = format!(
                        "Mismatched environment: \\end{{{env}}} should match \\begin{{{expected}}}"
                    );
                    self.error_handler.unmatched_brace('}', start, self.cursor);
                    return self.error_token_owned(start, msg);
                }
                let _ = self.pop_env();
            } else {
                let msg = format!("\\end{{{env}}} has no matching \\begin");
                self.error_handler
                    .unexpected_token('\\', start, self.cursor);
                return self.error_token_owned(start, msg);
            }

            let span = self.lexer_span(start);
            let text = self.consumed_source_text(start);
            LtxToken {
                kind: LtxTokenKind::EndEnv(env),
                span,
                text,
            }
        } else {
            self.error_handler.unexpected_token('{', start, self.cursor);
            self.error_token(start, "Expected \\end{...}")
        }
    }

    /// Scan a control symbol (e.g., `\$`, `\%`).
    #[inline]
    pub fn scan_control_symbol(&mut self, start: usize, sym: &'lxr str) -> LtxToken<'lxr> {
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        LtxToken {
            kind: LtxTokenKind::Command(sym),
            span,
            text,
        }
    }

    /// Scan normal command \textbf{hello}
    #[inline]
    pub fn normal_cmd(&mut self, start: usize, cmd_name: &'lxr str) -> LtxToken<'lxr> {
        let span = self.lexer_span(start);
        let text = self.consumed_source_text(start);
        while let Some(ch) = self.peek() {
            let cat = self.catcode.get(ch);
            if matches!(cat, LtxCatCode::WhiteSpace | LtxCatCode::EndOfLine) {
                let _ = self.bump();
            } else {
                break;
            }
        }
        LtxToken {
            kind: LtxTokenKind::Command(cmd_name),
            span,
            text,
        }
    }

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
            let _ = self.bump();
            let sym = self.slice(cmd_start, self.cursor);
            return self.scan_control_symbol(start, sym);
        }

        // Lone backslash at EOF
        self.error_handler
            .invalid_escape_sequence(start, self.cursor);
        self.error_token(start, "Lone backslash")
    }
}
