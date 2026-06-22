//! Test for JSON rendering of diagnostics
use ltx_diagnostics::{
    LtxDiagnostic, LtxSourceMap, LtxSpan, diagnostic::LtxDiagnosticInner, errors::LexerError,
    render_json,
};
use miette::Result;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::sync::Arc;

#[test]
fn test_json_render() -> Result<(), Box<dyn std::error::Error>> {
    let source = "Hello @ world!".to_string();

    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline("main.tex", source.clone());

    let ltx_span = LtxSpan::new(6, 7, file_id);

    let lexer_error = LexerError::UnexpectedToken {
        found: "@".to_string(),
        span: ltx_span,
    };

    let source_map = Arc::new(source_map);
    let diag = LtxDiagnostic::new(LtxDiagnosticInner::Lexer(lexer_error), source_map.clone());

    let json = render_json(&[diag], &source_map);
    let actual_json: Vec<Value> = serde_json::from_str(&json)?;

    // Verify the collection has exactly one error element
    assert_eq!(actual_json.len(), 1);

    let first = &actual_json[0];

    // Extract strings securely to ensure stable comparisons
    assert_eq!(first["severity"].as_str(), Some("error"));
    assert_eq!(first["code"].as_str(), Some("LTX::E001"));

    Ok(())
}
