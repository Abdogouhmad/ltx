//! Test for JSON rendering of diagnostics
use ltx_diagnostics::{LtxDiagnostic, LtxSpan, errors::LexerError, render_json};
use miette::{NamedSource, Result};
use pretty_assertions::assert_eq;
use serde_json::Value;

#[test]
fn test_json_render() -> Result<(), Box<dyn std::error::Error>> {
    let source = "Hello @ world!".to_string();
    let file_name = "main.tex".to_string();

    let ltx_span = LtxSpan::new(6, 7, file_name.clone());

    let core_diag = LtxDiagnostic::Lexer(LexerError::UnexpectedToken {
        found: "@".to_string(),
        span: ltx_span.clone().into(),
        src: NamedSource::new(&file_name, source.clone()),
    });

    // 3. Contextualize it with source code via your existing with_source pipeline
    let diag = core_diag.with_source(ltx_span, source, file_name);
    let json = render_json(&[diag]);
    let actual_json: Vec<Value> = serde_json::from_str(&json)?;

    println!("your json: \n {:#?}", actual_json);

    // Verify the collection has exactly one error element
    assert_eq!(actual_json.len(), 1);

    let first = &actual_json[0];

    // Extract strings securely to ensure stable comparisons
    assert_eq!(first["severity"].as_str(), Some("error"));
    assert_eq!(first["code"].as_str(), Some("LTX::E001")); // Fixed: Changed from ltx::parse::E003 to LTX::E001

    Ok(())
}
