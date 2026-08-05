#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

//! Integration tests for the LTX lexer.
//!
//! Covers catcode lookup, basic tokenization, mode dispatch,
//! environment scanning, and the `TokenStream` cursor API.

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{
    LtxCatCode, LtxCatCodeState, LtxLexer, LtxToken, LtxTokenKind, MathDelimiter, TokenStream,
};
use pretty_assertions::assert_eq;

// ── helpers ──────────────────────────────────────────────────────────

fn make_lexer(source: &str) -> LtxLexer<'_> {
    let mut map = LtxSourceMap::new();
    let id = map.add_inline("test.tex", source);
    LtxLexer::new(source, id, map)
}

fn lex_all(source: &str) -> Vec<LtxToken<'_>> {
    make_lexer(source).collect()
}

fn lex_kinds(source: &str) -> Vec<LtxTokenKind<'_>> {
    lex_all(source).into_iter().map(|t| t.kind).collect()
}

// ── CatCode tests ────────────────────────────────────────────────────

#[test]
fn test_catcode_escape_is_letter() {
    let state = LtxCatCodeState::default();
    assert_eq!(state.get('\\'), LtxCatCode::Escape);
    assert_ne!(state.get('\\'), LtxCatCode::Letter);
}

#[test]
fn test_catcode_braces() {
    let state = LtxCatCodeState::default();
    assert_eq!(state.get('{'), LtxCatCode::GroupStart);
    assert_eq!(state.get('}'), LtxCatCode::GroupEnd);
}

#[test]
fn test_catcode_dollar() {
    let state = LtxCatCodeState::default();
    assert_eq!(state.get('$'), LtxCatCode::MathShift);
}

#[test]
fn test_catcode_letters() {
    let state = LtxCatCodeState::default();
    for ch in 'a'..='z' {
        assert_eq!(
            state.get(ch),
            LtxCatCode::Letter,
            "expected Letter for '{ch}'"
        );
    }
    for ch in 'A'..='Z' {
        assert_eq!(
            state.get(ch),
            LtxCatCode::Letter,
            "expected Letter for '{ch}'"
        );
    }
}

#[test]
fn test_catcode_digit_is_other() {
    let state = LtxCatCodeState::default();
    for ch in '0'..='9' {
        assert_eq!(
            state.get(ch),
            LtxCatCode::Other,
            "expected Other for '{ch}'"
        );
    }
}

#[test]
fn test_catcode_out_of_range() {
    let state = LtxCatCodeState::default();
    assert_eq!(state.get('\u{20AC}'), LtxCatCode::Other);
    assert_eq!(state.get('\u{4E2D}'), LtxCatCode::Other);
}

// ── Lexer basic tests ────────────────────────────────────────────────

#[test]
fn test_lexer_empty_source() {
    let tokens = lex_all("");
    assert!(tokens.is_empty());
}

#[test]
fn test_lexer_single_text() {
    let tokens = lex_all("hello");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, LtxTokenKind::Text);
    assert_eq!(tokens[0].text, "hello");
}

#[test]
fn test_lexer_command() {
    let tokens = lex_all("\\textbf");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, LtxTokenKind::Command("textbf"));
    assert_eq!(tokens[0].text, "\\textbf");
}

#[test]
fn test_lexer_group() {
    let tokens = lex_all("{}");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, LtxTokenKind::GroupStart);
    assert_eq!(tokens[0].text, "{");
    assert_eq!(tokens[1].kind, LtxTokenKind::GroupEnd);
    assert_eq!(tokens[1].text, "}");
}

#[test]
fn test_lexer_whitespace() {
    let tokens = lex_all("   ");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, LtxTokenKind::WhiteSpace);
    assert_eq!(tokens[0].text, "   ");
}

#[test]
fn test_lexer_comment() {
    let tokens = lex_all("% comment");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, LtxTokenKind::Comment);
    assert_eq!(tokens[0].text, "% comment");
}

#[test]
fn test_lexer_math_shift_single() {
    let tokens = lex_all("$");
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].kind,
        LtxTokenKind::MathStart(MathDelimiter::Dollar)
    );
}

#[test]
fn test_lexer_math_shift_double() {
    let tokens = lex_all("$$");
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].kind,
        LtxTokenKind::MathStart(MathDelimiter::DoubleDollar)
    );
}

#[test]
fn test_lexer_text_with_special_chars() {
    let tokens = lex_all("hello world");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, LtxTokenKind::Text);
    assert_eq!(tokens[0].text, "hello");
    assert_eq!(tokens[1].kind, LtxTokenKind::WhiteSpace);
    assert_eq!(tokens[1].text, " ");
    assert_eq!(tokens[2].kind, LtxTokenKind::Text);
    assert_eq!(tokens[2].text, "world");
}

// ── Mode dispatch ────────────────────────────────────────────────────

#[test]
fn test_lexer_math_mode_toggle() {
    let kinds = lex_kinds("$x$");
    assert_eq!(
        kinds,
        vec![
            LtxTokenKind::MathStart(MathDelimiter::Dollar),
            LtxTokenKind::Text,
            LtxTokenKind::MathEnd(MathDelimiter::Dollar),
        ]
    );
}

#[test]
fn test_lexer_double_dollar_mode() {
    let kinds = lex_kinds("$$x$$");
    assert_eq!(
        kinds,
        vec![
            LtxTokenKind::MathStart(MathDelimiter::DoubleDollar),
            LtxTokenKind::Text,
            LtxTokenKind::MathEnd(MathDelimiter::DoubleDollar),
        ]
    );
}

// ── Environment scanning ─────────────────────────────────────────────

#[test]
fn test_lexer_begin_env() {
    let tokens = lex_all("\\begin{itemize}");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, LtxTokenKind::BeginEnv("itemize"));
    assert_eq!(tokens[0].text, "\\begin{itemize}");
}

#[test]
fn test_lexer_end_env() {
    let tokens = lex_all("\\begin{itemize}\\end{itemize}");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, LtxTokenKind::BeginEnv("itemize"));
    assert_eq!(tokens[1].kind, LtxTokenKind::EndEnv("itemize"));
    assert_eq!(tokens[1].text, "\\end{itemize}");
}

#[test]
fn test_lexer_documentclass() {
    let tokens = lex_all("\\documentclass{article}");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, LtxTokenKind::DocumentClass("article"));
    assert_eq!(tokens[0].text, "\\documentclass{article}");
}

// ── TokenStream tests ────────────────────────────────────────────────

#[test]
fn test_stream_peek() {
    let stream = TokenStream::new(make_lexer("hello"));
    let first = stream.peek().expect("should have first token");
    assert_eq!(first.kind, LtxTokenKind::Text);
    assert_eq!(first.text, "hello");
    assert_eq!(stream.peek().unwrap().text, "hello");
}

#[test]
fn test_stream_bump() {
    let mut stream = TokenStream::new(make_lexer("a b"));
    let first = stream.bump().expect("should bump first token");
    assert_eq!(first.kind, LtxTokenKind::Text);
    assert_eq!(first.text, "a");
    let second = stream.bump().expect("should bump second token");
    assert_eq!(second.kind, LtxTokenKind::WhiteSpace);
}

#[test]
fn test_stream_checkpoint_rewind() {
    let mut stream = TokenStream::new(make_lexer("a b"));

    let checkpoint = stream.checkpoint();
    assert_eq!(checkpoint, 0);

    let first = stream.bump().expect("bump first");
    assert_eq!(first.text, "a");
    let second = stream.bump().expect("bump second");
    assert_eq!(second.kind, LtxTokenKind::WhiteSpace);

    stream.rewind(checkpoint);
    assert_eq!(stream.peek().unwrap().text, "a");
}

#[test]
fn test_stream_skip_ws() {
    let mut stream = TokenStream::new(make_lexer("hello   world"));

    assert_eq!(stream.peek().unwrap().text, "hello");
    let _ = stream.bump();

    assert_eq!(stream.peek().unwrap().kind, LtxTokenKind::WhiteSpace);
    stream.skip_ws();

    assert_eq!(stream.peek().unwrap().kind, LtxTokenKind::Text);
    assert_eq!(stream.peek().unwrap().text, "world");
}

#[test]
fn test_stream_at_eof() {
    let mut stream = TokenStream::new(make_lexer("a"));
    assert!(!stream.at_eof());

    let _ = stream.bump();
    assert!(stream.at_eof());

    assert!(stream.bump().is_none());
    assert!(stream.at_eof());
}
