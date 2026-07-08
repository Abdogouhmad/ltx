//! Example of tokenizing a LaTeX source string.
#![allow(
    clippy::print_stdout,
    clippy::print_literal,
    clippy::uninlined_format_args
)]

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, LtxToken, LtxTokenKind, TokenStream};

fn main() {
    let source = r"Hey %comment is here
{ and } \$ \( \) \[ \] $E=mc^2$ \documentclass{article} \begin{document} \textbf{bold} \end{documen}";

    let mut source_map = LtxSourceMap::default();
    let file_id = source_map.add_inline("example.tex", source);
    let lexer = LtxLexer::new(source, file_id, source_map);

    // TokenStream is now the entry point — the parser (and this example)
    // never touches LtxLexer directly again after this line.
    let mut stream = TokenStream::new(lexer);

    println!("TOKENS");
    println!("{:<25} {:<12} {}", "Kind", "Text", "Span");
    println!("{}", "-".repeat(65));

    while !stream.at_eof() {
        // bump() drives the cursor forward one token at a time, same
        // observable order as the old `for token in lexer.by_ref()` loop.
        // let token = stream.bump().expect("checked by at_eof");
        let Some(token) = stream.bump() else {
            panic!("check by at eof")
        };
        let text_repr = token.text.escape_debug().to_string();

        match &token.kind {
            LtxTokenKind::WhiteSpace => {
                println!("{:<25} {:<12} {:?}", "WhiteSpace", text_repr, token.span);
            }
            LtxTokenKind::EndOfLine => {
                println!("{:<25} {:<12} {:?}", "EndOfLine", text_repr, token.span);
            }
            LtxTokenKind::Comment => {
                println!("{:<25} {:<12} {:?}", "Comment", text_repr, token.span);
            }
            LtxTokenKind::GroupStart => {
                println!("{:<25} {:<12} {:?}", "GroupStart", text_repr, token.span);
            }
            LtxTokenKind::GroupEnd => {
                println!("{:<25} {:<12} {:?}", "GroupEnd", text_repr, token.span);
            }
            LtxTokenKind::MathStart(delim) => {
                let info = format!("MathStart({:?})", delim);
                println!("{:<25} {:<12} {:?}", info, text_repr, token.span);
            }
            LtxTokenKind::MathEnd(delim) => {
                let info = format!("MathEnd({:?})", delim);
                println!("{:<25} {:<12} {:?}", info, text_repr, token.span);
            }
            LtxTokenKind::DocumentClass(name) => {
                let info = format!("DocumentClass({})", name);
                println!("{:<25} {:<12} {:?}", info, text_repr, token.span);
            }
            LtxTokenKind::BeginEnv(name) => {
                let info = format!("BeginEnv({})", name);
                println!("{:<25} {:<12} {:?}", info, text_repr, token.span);
            }
            LtxTokenKind::EndEnv(name) => {
                let info = format!("EndEnv({})", name);
                println!("{:<25} {:<12} {:?}", info, text_repr, token.span);
            }
            LtxTokenKind::Command(name) => {
                let info = format!("Command({})", name);
                println!("{:<25} {:<12} {:?}", info, text_repr, token.span);
            }
            LtxTokenKind::Text => {
                println!("{:<25} {:<12} {:?}", "Text", text_repr, token.span);
            }
            LtxTokenKind::Error(msg) => {
                println!(
                    "{:<25} {:<12} {:?}",
                    format!("Error({})", msg),
                    text_repr,
                    token.span
                );
            }
            LtxTokenKind::Parameter(name) => {
                let info = format!("Parameter({})", name);
                println!("{:<25} {:<12} {:?}", info, text_repr, token.span);
            }
            LtxTokenKind::Active(ch) => {
                let info = format!("Active({})", ch);
                println!("{:<25} {:<12} {:?}", info, text_repr, token.span);
            }
        }
    }

    println!();

    // Diagnostics survived the move into TokenStream via `by_ref()` in `new`.
    let has_error = stream.error_stream().has_errors();
    if has_error {
        println!("ERRORS ({})", stream.error_stream().total_count());
        println!("{}", "-".repeat(65));
        print!("{}", stream.error_stream_mut().render_pretty());
    } else {
        println!("No errors.");
    }
}
