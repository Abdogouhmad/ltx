//! this is example of json rendering to errors
use std::sync::Arc;

use ltx_diagnostics::{
    LtxDiagnostic, LtxSourceMap, LtxSpan, diagnostic::LtxDiagnosticInner, errors::LexerError,
    render_json,
};

fn main() {
    let source = "Hello @ world!".to_string();

    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline("main.tex", source.clone());

    let ltx_span = LtxSpan::new(6, 7, file_id);
    let lexer_error = LexerError::UnexpectedToken {
        found: '@'.to_string(),
        span: ltx_span,
    };

    let src_map = Arc::new(source_map);
    let dia = LtxDiagnostic::new(LtxDiagnosticInner::Lexer(lexer_error), src_map.clone());
    let json = render_json(&[dia], &src_map);
    println!("{}", json);
}
