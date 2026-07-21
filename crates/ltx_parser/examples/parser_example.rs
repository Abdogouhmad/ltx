//! Comprehensive demo: parse a LaTeX document AST and display diagnostics.
//!
//! Run with: `cargo run --example parser_example -p ltx_parser`

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, MathDelimiter, TokenStream};
use ltx_parser::{
    LtxParser,
    ast::{Arg, Document, DocumentBodyNode, PreambleItem},
    parse_document,
};

fn main() {
    let source = r"\documentclass[12pt,a4paper]{article}
\usepackage[utf8]{inputenc}
% Preamble comment
\title{Sample Document}

\begin{document}
Welcome to \textbf{LTX Parser}!
Here is inline math: $E = mc^2$ and display math:
$$\int_{0}^{1} x^2 \, dx = \frac{1}{3}$$

\begin{itemize}
  \item First item
  \item Second item
\end{itemize}

% Body comment
\end{document}";

    let mut source_map = LtxSourceMap::default();
    let file_id = source_map.add_inline("example.tex", source);
    let stream = TokenStream::new(LtxLexer::new(source, file_id, source_map));
    let mut parser = LtxParser::new(stream);

    let doc = parse_document(&mut parser);

    println!("=== PREAMBLE ({}) ===", doc.preamble.len());
    print_preamble(&doc);

    println!("\n=== DOCUMENT BODY ({}) ===", doc.body.len());
    for node in &doc.body {
        print_body_node(node, &parser, 1);
    }

    // Diagnostics summary
    let errs = parser.error_handler_mut();
    if errs.has_errors() {
        eprintln!("\nDiagnostics ({} errors):", errs.error_count());
        eprintln!("{}", errs.render_pretty());
    } else {
        println!("\nParse completed cleanly with 0 errors.");
    }
}

/// Prints the preamble of the document.
fn print_preamble(doc: &Document) {
    println!("\n=== PREAMBLE ({}) ===", doc.preamble.len());
    for item in &doc.preamble {
        match item {
            PreambleItem::DocumentClass(dc) => {
                let opts = dc.options.as_ref().map_or("none", |o| o.text);
                println!("DocumentClass: class='{}', opts='{}'", dc.class_name, opts);
            }
            PreambleItem::UsePackage(pkg) => {
                let opts = pkg.options.as_ref().map_or("none", |o| o.text);
                println!("UsePackage: pkg='{}', opts='{}'", pkg.package_name, opts);
            }
            PreambleItem::Command(cmd) => {
                println!("Preamble Command: \\{}", cmd.name);
            }
            PreambleItem::Comment(comment) => {
                println!("Preamble Comment: {:?}", comment.comment_text);
            }
            PreambleItem::Text(t) => {
                if !t.text.trim().is_empty() {
                    println!("Preamble Text: {:?}", t.text);
                }
            }
            PreambleItem::Group(_) => {
                println!("Preamble Group");
            }
        }
    }
}

/// Prints the body of the document.
fn print_body_node(node: &DocumentBodyNode, parser: &LtxParser, depth: usize) {
    let indent = "  ".repeat(depth);
    match node {
        DocumentBodyNode::Text(t) => {
            if !t.text.trim().is_empty() {
                println!("{indent}Text: {:?}", t.text);
            }
        }
        DocumentBodyNode::Command(cmd) => {
            println!("{indent}Command: \\{}", cmd.name);
            for arg in &cmd.args {
                match arg {
                    Arg::Braced(g) => {
                        let text: String = g.tokens(&parser.stream).map(|tok| tok.text).collect();
                        println!("{indent}  Braced Arg: {}", text);
                    }
                    Arg::Optional(o) => {
                        println!("{indent}  Optional Arg: [{}]", o.text);
                    }
                }
            }
        }
        DocumentBodyNode::Math(m) => {
            let mode = match m.delimiter {
                MathDelimiter::Dollar => "Inline",
                MathDelimiter::DoubleDollar => "Display",
            };
            let math_content: String = m.tokens(&parser.stream).map(|tok| tok.text).collect();
            println!("{indent}Math ({mode}): {math_content}");
        }
        DocumentBodyNode::Comment(c) => {
            println!("{indent}Comment: {:?}", c.comment_text);
        }
        DocumentBodyNode::Group(g) => {
            let content: String = g.tokens(&parser.stream).map(|tok| tok.text).collect();
            println!("{indent}Group: {content}");
        }
        DocumentBodyNode::Environment(env) => {
            let status = if env.end_span.is_some() {
                "closed"
            } else {
                "UNCLOSED"
            };
            println!("{indent}Environment: \\begin{{{}}} ({status})", env.name);
            for child in &env.body {
                print_body_node(child, parser, depth + 1);
            }
        }
    }
}
