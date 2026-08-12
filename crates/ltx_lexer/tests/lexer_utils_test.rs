//! Tests for `LtxLexer` utility methods.

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::LtxLexer;
use pretty_assertions::assert_eq;

fn make_lexer(source: &str) -> LtxLexer<'_> {
    let mut map = LtxSourceMap::new();
    let id = map.add_inline("test.tex", source);
    LtxLexer::new(source, id, map)
}

#[test]
fn test_scan_env_name_simple() {
    let mut lex = make_lexer("{enumerate}");
    assert_eq!(lex.scan_env_name_optional(), Some("enumerate"));
}

#[test]
fn test_scan_env_name_with_optional_prefix() {
    let mut lex = make_lexer("[12pt,a4paper]{document}");
    assert_eq!(lex.scan_env_name_optional(), Some("document"));
}

#[test]
fn test_scan_env_name_with_nested_brackets_returns_none() {
    let mut lex = make_lexer("[key=[val]]{tabular}");
    // Nested brackets are not supported; scanner exits optional at first `]`.
    assert_eq!(lex.scan_env_name_optional(), None);
}

#[test]
fn test_scan_env_name_with_nested_braces_inside_optional() {
    let mut lex = make_lexer("[opt={val}]{itemize}");
    assert_eq!(lex.scan_env_name_optional(), Some("itemize"));
}

#[test]
fn test_scan_env_name_missing_braces() {
    let mut lex = make_lexer("enumerate");
    assert_eq!(lex.scan_env_name_optional(), None);
}

#[test]
fn test_scan_env_name_missing_closing_brace() {
    let mut lex = make_lexer("{enumerate");
    assert_eq!(lex.scan_env_name_optional(), None);
}

#[test]
fn test_scan_env_name_empty_optional_then_braces() {
    let mut lex = make_lexer("[]{figure}");
    assert_eq!(lex.scan_env_name_optional(), Some("figure"));
}
