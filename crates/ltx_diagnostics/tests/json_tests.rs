//! Tests for json render of `LtxDiagnostic`.
use ltx_diagnostics::{LtxDiagnostic, LtxSpan, render_json};
use serde_json::Value;

#[test]
fn test_render_json() {
    // 1. Create a lightweight span (what the lexer/parser produces)
    let span = LtxSpan::new(0, 0, "main.tex");

    // 2. Build the diagnostic using the official helper
    let diag = LtxDiagnostic::MissingDocumentClass {
        span: (0, 0).into(),
        src: miette::NamedSource::new("", "".to_string()),
    }
    .with_source(span, r"\begin{document}".to_string(), "main.tex".to_string());

    // 3. Render to JSON and parse it
    let json = render_json(&[diag]);
    let parsed: Vec<Value> = serde_json::from_str(&json).unwrap();

    // 4. Verify structure and content semantically
    assert_eq!(parsed.len(), 1);

    let first = &parsed[0];
    assert_eq!(first["severity"], "error");
    assert_eq!(first["code"], "ltx::parse::E003");

    // No escaping concerns - this matches the actual message string in memory
    assert_eq!(first["message"], r"missing \documentclass");
}
