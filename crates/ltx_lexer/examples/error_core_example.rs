//! Example demonstrating how to use `LexerErrorCore` to collect and handle lexer errors,
//! and print them using `miette::Report` and `LtxDiagnosticSink::drain_sorted`.

#![allow(clippy::print_stdout)]

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::errors_core::LexerErrorHandler;
use std::sync::Arc;

fn main() {
    let mut source_map = LtxSourceMap::new();
    let source = "\\documentclass{article}\n\\begin{document}\nHello } world!\nStray char: \\x07";
    let file_id = source_map.add_inline("example.tex", source);
    let source_map = Arc::new(source_map);

    println!("SOURCE");
    for (i, line) in source.lines().enumerate() {
        println!("  {:>2} | {}", i + 1, line);
    }

    let mut error_core = LexerErrorHandler::new(file_id, source_map);

    let error_start_1 = 46;
    let error_end_1 = 47;
    error_core.unmatched_brace('}', error_start_1, error_end_1);

    let error_start_2 = 72;
    let error_end_2 = 73;
    error_core.invalid_character('\x07', error_start_2, error_end_2);

    println!();
    println!("ERRORS ({})", error_core.error_count());
    println!("{}", "-".repeat(50));
    print!("{}", error_core.render_pretty());
}
