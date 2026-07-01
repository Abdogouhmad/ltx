//! Example of tokenizing a LaTeX source string.
#![allow(clippy::print_stdout)]

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxCatCode, LtxLexer};

fn main() {
    let source = r"Hey %comment is here
{ and } \$ \( \) \[ \] $E=mc^2$";

    let mut source_map = LtxSourceMap::default();
    let file_id = source_map.add_inline("example.tex", source);
    let mut lexer = LtxLexer::new(source, file_id, source_map);

    while !lexer.is_eof() {
        if let Some(c) = lexer.peek() {
            let cat = lexer.catcode.get(c);
            handle_token(cat, &mut lexer);
        }
    }
}

fn handle_token(cat: LtxCatCode, lexer: &mut LtxLexer) {
    match cat {
        LtxCatCode::WhiteSpace => {
            let token = lexer.scan_whitespace();
            println!("WhiteSpace: {:?} span: {:?}", token.text, token.span);
        }
        LtxCatCode::EndOfLine => {
            let token = lexer.scan_eol();
            println!("EOL: {:?} span: {:?}", token.text, token.span);
        }
        LtxCatCode::Comment => {
            let token = lexer.scan_comment();
            println!("Comment: {:?} span: {:?}", token.text, token.span);
        }
        LtxCatCode::GroupStart => {
            let token = lexer.scan_group_start();
            println!("GroupStart: {:?} span: {:?}", token.text, token.span);
        }
        LtxCatCode::GroupEnd => {
            let token = lexer.scan_group_end();
            println!("GroupEnd: {:?} span: {:?}", token.text, token.span);
        }
        LtxCatCode::MathShift => {
            let token = lexer.scan_math_shift();
            println!(
                "MathShift: {:?} span: {:?} -> {:?}",
                token.text, token.span, token.kind
            );
        }
        LtxCatCode::Escape => {
            let token = lexer.scan_escape();
            println!("Escape: {:?} span: {:?}", token.text, token.span);
        }
        LtxCatCode::Letter | LtxCatCode::Other => {
            let token = lexer.scan_text();
            println!("Lexer Text --> : {:?} span: {:?}", token.text, token.span);
        }
        _ => {
            let start = lexer.current_cursor();
            let ch = lexer.bump().unwrap_or('\0');
            let span = lexer.lexer_span(start);
            println!("Text: '{ch}' span: {span:?}");
        }
    }
}
