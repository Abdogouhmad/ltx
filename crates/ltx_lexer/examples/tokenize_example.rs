//! Example of tokenizing a LaTeX source string.
#![allow(clippy::print_stdout)]

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxCatCode, LtxLexer, LtxTokenKind};
use miette::Report;

fn main() {
    let source = r"Hey %comment is here
{ and } \$ \( \) \[ \] $E=mc^2$ \documentclass{article} \begin{document} \textbf{bold} \end{documen}";

    let mut source_map = LtxSourceMap::default();
    let file_id = source_map.add_inline("example.tex", source);
    let mut lexer = LtxLexer::new(source, file_id, source_map);

    // Lex all tokens
    while !lexer.is_eof() {
        if let Some(c) = lexer.peek() {
            let cat = lexer.catcode.get(c);
            handle_token(cat, &mut lexer);
        }
    }

    // Check for errors after lexing
    if lexer.error_handler.has_errors() {
        println!("\n🔍 Found {} errors:\n", lexer.error_handler.total_count());

        // Take diagnostics from the error handler
        let diagnostics = lexer.error_handler.take_diagnostics();

        // Print each diagnostic with miette
        for diag in diagnostics {
            let report = Report::new(diag);
            println!("{report:?}");
            println!("---");
        }
    } else {
        println!("\n✅ No errors found.");
    }
}

fn handle_token(cat: LtxCatCode, lexer: &mut LtxLexer) {
    match cat {
        // Uncomment these to see all tokens
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
            let token = lexer.scan_command();
            match token.kind {
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
                LtxTokenKind::Error(msg) => {
                    // msg is Cow<'static, str>, so we can use it directly
                    println!("Error: '{}' span: {:?}", msg, token.span);

                    // If you need to check if it's borrowed or owned:
                    // match msg {
                    //     std::borrow::Cow::Borrowed(s) => {
                    //         println!("  (static error message: '{}')", s);
                    //     }
                    //     std::borrow::Cow::Owned(s) => {
                    //         println!("  (dynamic error message: '{}')", s);
                    //     }
                    // }
                }
                _ => {
                    println!("Unknown: {:?} span: {:?}", token.kind, token.span);
                }
            }
        }
        LtxCatCode::Letter | LtxCatCode::Other => {
            let token = lexer.scan_text();
            println!("Text: {:?} span: {:?}", token.text, token.span);
        }
        _ => {
            let start = lexer.current_cursor();
            let ch = lexer.bump().unwrap_or('\0');
            let span = lexer.lexer_span(start);
            println!("Unknown: '{ch}' span: {span:?}");
        }
    }
}
