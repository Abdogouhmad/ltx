//! Example of tokenizing a LaTeX source string.
#![allow(clippy::print_stdout)]

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, LtxTokenKind};
use miette::Report;

fn main() {
    let source = r"Hey %comment is here
{ and } \$ \( \) \[ \] $E=mc^2$ \documentclass{article} \begin{document} \textbf{bold} \end{documen}";

    let mut source_map = LtxSourceMap::default();
    let file_id = source_map.add_inline("example.tex", source);

    let mut lexer = LtxLexer::new(source, file_id, source_map);

    for token in lexer.by_ref() {
        match &token.kind {
            LtxTokenKind::WhiteSpace => {
                println!("WhiteSpace: {:?} span: {:?}", token.text, token.span);
            }

            LtxTokenKind::EndOfLine => {
                println!("EOL: {:?} span: {:?}", token.text, token.span);
            }

            LtxTokenKind::Comment => {
                println!("Comment: {:?} span: {:?}", token.text, token.span);
            }

            LtxTokenKind::GroupStart => {
                println!("GroupStart: {:?} span: {:?}", token.text, token.span);
            }

            LtxTokenKind::GroupEnd => {
                println!("GroupEnd: {:?} span: {:?}", token.text, token.span);
            }

            LtxTokenKind::MathStart(delim) => {
                println!(
                    "MathStart {:?}: {:?} span: {:?}",
                    delim, token.text, token.span
                );
            }

            LtxTokenKind::MathEnd(delim) => {
                println!(
                    "MathEnd {:?}: {:?} span: {:?}",
                    delim, token.text, token.span
                );
            }

            LtxTokenKind::DocumentClass(name) => {
                println!("DocumentClass: '{}' span: {:?}", name, token.span);
            }

            LtxTokenKind::BeginEnv(name) => {
                println!("BeginEnv: '{}' span: {:?}", name, token.span);
            }

            LtxTokenKind::EndEnv(name) => {
                println!("EndEnv: '{}' span: {:?}", name, token.span);
            }

            LtxTokenKind::Command(name) => {
                println!("Command: '\\{}' span: {:?}", name, token.span);
            }

            LtxTokenKind::Text => {
                println!("Text: {:?} span: {:?}", token.text, token.span);
            }

            LtxTokenKind::Escape => {
                println!("Escape: {:?} span: {:?}", token.text, token.span);
            }

            LtxTokenKind::Error(msg) => {
                println!("Error: {} span: {:?}", msg, token.span);
            }

            other => {
                println!("{:?}: {:?} span: {:?}", other, token.text, token.span);
            }
        }
    }

    if lexer.error_handler.has_errors() {
        println!("\n🔍 Found {} errors:\n", lexer.error_handler.total_count());

        for diagnostic in lexer.error_handler.take_diagnostics() {
            println!("{:?}", Report::new(diagnostic));
            println!("---");
        }
    } else {
        println!("\n✅ No errors found.");
    }
}
