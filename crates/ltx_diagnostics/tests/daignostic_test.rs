//! test Errors and diagnostic

use ltx_diagnostics::{
    LtxDiagnostic, LtxSourceMap, LtxSpan, diagnostic::LtxDiagnosticInner, errors::LexerError,
};
use miette::Diagnostic;
use pretty_assertions::assert_eq as pretty_assert_eq;
use std::sync::Arc;

#[test]
fn test_diagnostic() {
    let source = "Hello @ world!".to_string();

    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline("main.tex", source.clone());

    let ltx_span = LtxSpan::new(6, 7, file_id);
    // Lexer error code LTX::E001
    let lexer_error = LexerError::UnexpectedToken {
        found: "@".to_string(),
        span: ltx_span,
    };

    // Raise error
    let diagnostic =
        LtxDiagnostic::new(LtxDiagnosticInner::Lexer(lexer_error), Arc::new(source_map));

    println!("Your Diagnostic: \n {:#?}", diagnostic);
    pretty_assert_eq!(format!("{}", diagnostic), "unexpected token `@`");

    pretty_assert_eq!(
        diagnostic.code().map(|c| c.to_string()),
        Some("LTX::E001".to_string())
    );
}
