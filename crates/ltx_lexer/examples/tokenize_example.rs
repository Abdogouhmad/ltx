//! Example of tokenizing a LaTeX source string.
use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, LtxCatCode};

fn main() {
    // init resource needed for lexing
    let source = r#"Hello,  world!
Hey Latex %small comment"#;
    let mut source_map = LtxSourceMap::default();
    let file_id = source_map.add_inline("example.tex", source);
    let mut lexer_eg = LtxLexer::new(source, file_id, source_map);

    // loop over tokens
    while !lexer_eg.is_eof() {
        // match the next character
        match lexer_eg.peek() {
            // get whitesp
            Some(c) => {
                let cat = lexer_eg.catcode.get(c);
                matcher_arm(cat, &mut lexer_eg);
            }
            None => break,
        }
    }
}


fn matcher_arm(cat: LtxCatCode, lexer_eg: &mut LtxLexer) {
    match cat {
        LtxCatCode::WhiteSpace => {
            let token = lexer_eg.scan_whitespace();
            println!("WhiteSpace: '{}' span: {:?}", token.text, token.span);
        }
        LtxCatCode::EndOfLine => {
            let token = lexer_eg.scan_eol();
            println!("EOL -> : '{}' span: {:?}", token.text, token.span);
            println!("Should be eol -> {:?} ", token.kind);
        }
        LtxCatCode::Comment => {
            let token = lexer_eg.scan_comment();
            println!("Comment: '{}' span: {:?}", token.text, token.span);
            println!("Should be comment -> {:?} ", token.kind);
        }
        _ => {
            let start = lexer_eg.current_cursor();
            let ch = lexer_eg.bump().unwrap();
            let span = lexer_eg.lexer_span(start);
            println!("Text: '{}' span: {:?}", ch, span);
        }
    }
}
