//! Example demonstrating how to use `LexerErrorCore` to collect and handle lexer errors,
//! and print them using `miette::Report` and `LtxDiagnosticSink::drain_sorted`.

use ltx_diagnostics::{LtxDiagnosticSink, LtxSourceMap};
use ltx_lexer::errors_core::LexerErrorHandler;
use miette::Report;
use std::sync::Arc;

fn main() {
    // 1. Initialize the SourceMap
    let mut source_map = LtxSourceMap::new();

    // Simulate LaTeX source code with syntax errors:
    // Line 1: \documentclass{article}
    // Line 2: \begin{document}
    // Line 3: Hello } world! (Unmatched brace)
    // Line 4: Stray char: \x07
    let source = "\\documentclass{article}\n\\begin{document}\nHello } world!\nStray char: \\x07";
    let file_id = source_map.add_inline("example.tex", source);
    let source_map = Arc::new(source_map);

    // 2. Create the LexerErrorCore
    let mut error_core = LexerErrorHandler::new(file_id, source_map);

    // 3. Simulate parsing and discovering lexical errors

    // Error 1: Unexpected brace '}' on Line 3
    // "Hello } world!" -> '}' is at byte offset 46
    let error_start_1 = 46;
    let error_end_1 = 47;
    error_core.unmatched_brace('}', error_start_1, error_end_1);

    // Error 2: Invalid control character '\x07' on Line 4
    // "Stray char: \x07" -> '\x07' is at byte offset 72
    let error_start_2 = 72;
    let error_end_2 = 73;
    error_core.invalid_character('\x07', error_start_2, error_end_2);

    // 4. Retrieve raw diagnostics from error core
    let diagnostics = error_core.take_diagnostics();

    // 5. Collect them into the LtxDiagnosticSink
    let mut sink = LtxDiagnosticSink::new();
    for diag in diagnostics {
        sink.push(diag);
    }

    // 6. Pretty print them using miette::Report & LtxDiagnosticSink::drain_sorted()
    println!("🔍 Found {} issue(s):\n", sink.len());
    for diag in sink.drain_sorted() {
        let report = Report::new(diag);
        println!("{report:?}\n");
    }
}
