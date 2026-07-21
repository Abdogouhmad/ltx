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
