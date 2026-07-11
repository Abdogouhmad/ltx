//! Demo: parse all top-level tokens, show errors.
//!
//! Run with:  `cargo run --example parser_example -p ltx_parser`

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, LtxTokenKind, TokenStream};
use ltx_parser::{
    LtxParser,
    ast::{Command, Group, Text},
};

fn main() {
    // Try removing the closing `}` to see the error
    let source = r"Hey \textbf{hello world! 
    me";
    let mut source_map = LtxSourceMap::default();
    let file_id = source_map.add_inline("example.tex", source);
    let stream = TokenStream::new(LtxLexer::new(source, file_id, source_map));
    let mut parser = LtxParser::new(stream);

    while !parser.at_eof() {
        parser.skip_ws();
        if parser.at_eof() {
            break;
        }

        match parser.peek_kind() {
            Some(LtxTokenKind::Command(_)) => {
                let cmd: Command = parser.parse();
                println!("Command: \\{}", cmd.name);
                for arg in &cmd.args {
                    println!("  Arg:");
                    for tok in &arg.tokens {
                        println!("    {:12} {:?}", format!("{:?},", tok.kind), tok.text);
                    }
                }
            }
            Some(LtxTokenKind::Text) => {
                let text: Text = parser.parse();
                println!("Text:       {:?}", text.text);
            }
            Some(LtxTokenKind::GroupStart) => {
                let group: Group = parser.parse();
                println!("Group:");
                for tok in &group.tokens {
                    println!("    {:12} {:?}", format!("{:?},", tok.kind), tok.text);
                }
            }
            Some(LtxTokenKind::GroupEnd) => {
                let tok = parser.bump().unwrap();
                let span = tok.span;
                let kind = format!("{:?}", tok.kind);
                let text = tok.text;
                println!("{:>25} {text:?}", kind);
                parser
                    .error_handler_mut()
                    .unmatched_brace('}', span.start(), span.end());
            }
            Some(other) => {
                let debug = format!("{other:?}");
                let tok = parser.bump().unwrap();
                println!("{debug:>25} {:?}", tok.text);
            }
            _ => break,
        }
    }

    // Show any lex/parse diagnostics
    let errs = parser.error_handler_mut();
    if errs.has_errors() {
        eprintln!("\nYou got: {} Errors in total", errs.error_count());
        println!();
        eprintln!("{}", errs.render_pretty());
    } else {
        eprintln!("\nNo errors.");
    }
}
