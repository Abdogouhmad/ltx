//! Example of tokenizing a LaTeX source string.
#![allow(clippy::print_stdout)]

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, LtxTokenKind};

fn main() {
    let source = r"Hey %comment is here
{ and } \$ \( \) \[ \] $E=mc^2$ \documentclass{article} \begin{document} \textbf{bold} \end{documen}";

    let mut source_map = LtxSourceMap::default();
    let file_id = source_map.add_inline("example.tex", source);

    let mut lexer = LtxLexer::new(source, file_id, source_map);

    println!("TOKENS");
    println!("{:<25} {:<12} {}", "Kind", "Text", "Span");
    println!("{}", "-".repeat(65));

    for token in lexer.by_ref() {
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
                println!("{:<25} {:<12} {:?}", format!("Error({})", msg), text_repr, token.span);
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
    if lexer.error_handler.has_errors() {
        println!("ERRORS ({})", lexer.error_handler.total_count());
        println!("{}", "-".repeat(65));
        for diagnostic in lexer.error_handler.take_diagnostics() {
            let report = miette::Report::new(diagnostic);
            println!("{:?}", report);
            println!();
        }
    } else {
        println!("No errors.");
    }
}
