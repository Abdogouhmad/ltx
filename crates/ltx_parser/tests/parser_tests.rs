#![allow(missing_docs, clippy::print_stdout)]

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, TokenStream};
use ltx_parser::{LtxParser, parse_document};

#[test]
fn test_parse_full_document() {
    let source = r"\documentclass[11pt]{article}
\usepackage[utf8]{inputenc}

\begin{document}
Hello \textbf{world}!
$a + b = c$
\end{document}";

    let mut map = LtxSourceMap::default();
    let file_id = map.add_inline("test.tex", source);
    let stream = TokenStream::new(LtxLexer::new(source, file_id, map));
    let mut parser = LtxParser::new(stream);

    let doc = parse_document(&mut parser);

    for (i, item) in doc.preamble.iter().enumerate() {
        println!("preamble[{i}] = {item:?}");
    }

    assert_eq!(doc.preamble.len(), 2);
}

#[test]
fn test_parse_does_not_loop_on_stray_end_environment() {
    // A stray `\end{...}` before `\begin{document}` must not make the
    // preamble loop spin forever: parsing must terminate and report an error.
    let source = "\\begin{minipage}\n\\end{minipage}\n\\end{document}\n";

    let mut map = LtxSourceMap::default();
    let file_id = map.add_inline("test.tex", source);
    let stream = TokenStream::new(LtxLexer::new(source, file_id, map));
    let mut parser = LtxParser::new(stream);

    let doc = parse_document(&mut parser);

    assert!(!parser.error_handler().is_empty());
    assert!(doc.body.is_empty());
}

#[test]
fn test_parse_does_not_loop_on_stray_group_end() {
    // A stray `}` in the document body is swallowed as text instead of
    // looping forever.
    let source = "\\begin{document}\n} \n\\end{document}\n";

    let mut map = LtxSourceMap::default();
    let file_id = map.add_inline("test.tex", source);
    let stream = TokenStream::new(LtxLexer::new(source, file_id, map));
    let mut parser = LtxParser::new(stream);

    let doc = parse_document(&mut parser);

    assert_eq!(doc.body.len(), 1);
    assert!(parser.error_handler().is_empty());
}
