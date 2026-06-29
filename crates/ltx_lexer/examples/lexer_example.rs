//! Step 1: Basic lexer functionality – testing EOF and spans.

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::LtxLexer;

fn main() {
    let source = "Hello, world!";
    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline("example.tex", source);
    let mut lexer = LtxLexer::new(source, file_id, source_map);

    // 4. Tokenization loop: capture each character with its span.
    let mut tokens: Vec<(char, ltx_diagnostics::LtxSpan)> = Vec::with_capacity(source.len());

    while !lexer.is_eof() {
        let start = lexer.current_cursor(); // remember where we start
        if let Some(ch) = lexer.bump() {
            let span = lexer.lexer_span(start); // start → after bump
            tokens.push((ch, span));
        }
    }

    // 5. Print results.
    println!("Total characters: {}", tokens.len());
    for (ch, span) in &tokens {
        println!("'{}' span: {:?}", ch, span);
    }

    println!("EOF: {}", lexer.is_eof());
}
