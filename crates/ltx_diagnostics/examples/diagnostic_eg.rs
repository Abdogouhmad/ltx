//! End-to-end tour of `ltx-diagnostics`.
//!
//! Simulates what a lexer/parser front-end does after finding problems in a
//! `.tex` file: register the source, build `LtxError`s at specific spans,
//! wrap them into `LtxDiagnostic`s, collect them in a `LtxDiagnosticSink`,

#![allow(clippy::print_literal, clippy::print_stdout, clippy::expect_used)]
use std::borrow::Cow;
use std::io;
use std::sync::Arc;

use miette::Diagnostic as _;

use ltx_diagnostics::{
    LtxDiagnostic, LtxDiagnosticSink, LtxError, LtxSourceMap, LtxSpan, render_json_into,
    render_pretty_into,
};

fn main() -> io::Result<()> {
    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline(
        "chapter1.tex",
        "\\section{Introduction}\n\
         This uses \\nocommand and cites \\cite{unknown-key}.\n\
         \\begin{itemize}\n\
         \\item First point\n",
    );

    let source_map = Arc::new(source_map);

    let mut sink = LtxDiagnosticSink::new();

    // LTX::E100 — undefined control sequence (Error severity).
    let undefined_command_span = LtxSpan::new(33, 43, file_id);
    sink.push(LtxDiagnostic::new(
        LtxError::UndefinedControlSequence {
            name: Cow::Borrowed("nocommand"),
            span: undefined_command_span,
        },
        Arc::clone(&source_map),
    ));

    println!(
        "collected {} diagnostic(s); has_error = {}\n",
        sink.len(),
        sink.has_error()
    );

    let sorted = sink.drain_sorted(); // errors-first ordering
    let mut pretty_buf = String::new();
    render_pretty_into(&sorted, &mut pretty_buf).expect("fmt::Write to a String cannot fail");
    println!("--- pretty (miette) ---\n {pretty_buf}");

    println!("\n--- json ---");
    render_json_into(&sorted, io::stdout())?;
    println!();

    // `LtxDiagnostic::to_json` / `JsonDiagnostic` are also usable one at a
    // time, e.g. for streaming NDJSON to an LSP client:
    if let Some(first) = sorted.first() {
        let json = first.to_json();
        println!(
            "\nfirst diagnostic as JsonDiagnostic: code={}, line={:?}, column={:?}",
            json.code, json.line, json.column
        );
    }

    for diag in &sorted {
        let code = diag
            .code()
            .map_or_else(|| "?".to_string(), |c| c.to_string());
        let via_miette_attribute = miette::Diagnostic::severity(diag);
        let via_inherent_method = diag.severity();
        println!(
            "{code:<10} attribute severity = {via_miette_attribute:<18?} LtxDiagnostic::severity() = {via_inherent_method:?}"
        );
    }

    Ok(())
}
