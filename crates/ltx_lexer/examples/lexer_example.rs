//! Basic lexer functionality -- testing EOF and spans.
#![allow(clippy::print_stdout)]

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::LtxLexer;

fn main() {
    let source = "Hello, world!";
    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline("example.tex", source);
    let mut lexer = LtxLexer::new(source, file_id, source_map);

    let mut tokens: Vec<(char, ltx_diagnostics::LtxSpan)> = Vec::with_capacity(source.len());

    while !lexer.is_eof() {
        let start = lexer.current_cursor();
        if let Some(ch) = lexer.bump() {
            let span = lexer.lexer_span(start);
            tokens.push((ch, span));
        }
    }

    println!("CHARACTERS");
    println!("{:<6} {:<10} {}", "Index", "Char", "Span");
    println!("{}", "-".repeat(45));
    for (i, (ch, span)) in tokens.iter().enumerate() {
        println!("{:<6} {:<10} {:?}", i, ch.escape_debug(), span);
    }
    println!();
    println!("Total: {} characters", tokens.len());
    println!("EOF: {}", lexer.is_eof());
}
