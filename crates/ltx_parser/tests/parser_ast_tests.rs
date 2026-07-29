#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic, missing_docs)]

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, MathDelimiter, TokenStream};
use ltx_parser::ast::*;
use ltx_parser::{LtxParser, parse_document};
use pretty_assertions::assert_eq;

fn parse(source: &str) -> Document<'_> {
    let mut map = LtxSourceMap::default();
    let file_id = map.add_inline("test.tex", source);
    let stream = TokenStream::new(LtxLexer::new(source, file_id, map));
    let mut parser = LtxParser::new(stream);
    parse_document(&mut parser)
}

// ===== Document parsing =====

#[test]
fn test_parse_empty_document() {
    let source = r"\begin{document}\end{document}";
    let doc = parse(source);
    assert!(doc.preamble.is_empty());
    assert!(doc.body.is_empty());
}

#[test]
fn test_parse_document_no_begin() {
    let source = r"\documentclass{article}";
    let doc = parse(source);
    assert_eq!(doc.preamble.len(), 1);
    assert!(doc.body.is_empty());
}

#[test]
fn test_parse_minimal_document() {
    let source = r"\documentclass{article}
\begin{document}
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.preamble.len(), 1);
    if let PreambleItem::DocumentClass(dc) = &doc.preamble[0] {
        assert_eq!(dc.class_name, "article");
    } else {
        panic!("expected DocumentClass preamble item");
    }
    assert!(doc.body.is_empty());
}

#[test]
fn test_parse_document_with_body_text() {
    let source = r"\begin{document}
Hello world
\end{document}";
    let doc = parse(source);
    let texts: Vec<&str> = doc
        .body
        .iter()
        .filter_map(|n| match n {
            DocumentBodyNode::Text(t) => Some(t.text),
            _ => None,
        })
        .collect();
    assert!(!texts.is_empty());
    assert_eq!(texts.join(" "), "Hello world");
}

#[test]
fn test_parse_document_with_comment() {
    let source = r"\begin{document}
% this is a comment
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Comment(comment) = &doc.body[0] {
        assert_eq!(comment.comment_text, "% this is a comment");
    } else {
        panic!("expected Comment node in body");
    }
}

// ===== Preamble items =====

#[test]
fn test_parse_documentclass() {
    let source = r"\documentclass{article}
\begin{document}\end{document}";
    let doc = parse(source);
    assert_eq!(doc.preamble.len(), 1);
    if let PreambleItem::DocumentClass(dc) = &doc.preamble[0] {
        assert_eq!(dc.class_name, "article");
        assert!(dc.options.is_none());
    } else {
        panic!("expected DocumentClass preamble item");
    }
}

#[test]
fn test_parse_documentclass_with_options() {
    let source = r"\documentclass[12pt]{article}
\begin{document}\end{document}";
    let doc = parse(source);
    assert_eq!(doc.preamble.len(), 1);
    if let PreambleItem::DocumentClass(dc) = &doc.preamble[0] {
        assert_eq!(dc.class_name, "article");
        let opt = dc.options.as_ref().expect("expected options");
        assert_eq!(opt.text, "12pt");
    } else {
        panic!("expected DocumentClass preamble item");
    }
}

#[test]
fn test_parse_usepackage() {
    let source = r"\documentclass{article}
\usepackage{amsmath}
\begin{document}\end{document}";
    let doc = parse(source);
    assert_eq!(doc.preamble.len(), 2);
    if let PreambleItem::UsePackage(up) = &doc.preamble[1] {
        assert_eq!(up.package_name, "amsmath");
        assert!(up.options.is_none());
    } else {
        panic!("expected UsePackage preamble item");
    }
}

#[test]
fn test_parse_usepackage_with_options() {
    let source = r"\documentclass{article}
\usepackage[utf8]{inputenc}
\begin{document}\end{document}";
    let doc = parse(source);
    assert_eq!(doc.preamble.len(), 2);
    if let PreambleItem::UsePackage(up) = &doc.preamble[1] {
        assert_eq!(up.package_name, "inputenc");
        let opt = up.options.as_ref().expect("expected options");
        assert_eq!(opt.text, "utf8");
    } else {
        panic!("expected UsePackage preamble item");
    }
}

// ===== Command parsing =====

#[test]
fn test_parse_simple_command() {
    let source = r"\begin{document}
\textbf
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Command(cmd) = &doc.body[0] {
        assert_eq!(cmd.name, "textbf");
        assert!(cmd.args.is_empty());
    } else {
        panic!("expected Command node in body");
    }
}

#[test]
fn test_parse_command_with_braced_arg() {
    let source = r"\begin{document}
\textbf{hello}
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Command(cmd) = &doc.body[0] {
        assert_eq!(cmd.name, "textbf");
        assert_eq!(cmd.args.len(), 1);
        assert!(matches!(&cmd.args[0], Arg::Braced(_)));
    } else {
        panic!("expected Command node in body");
    }
}

#[test]
fn test_parse_command_with_optional_arg() {
    let source = r"\begin{document}
\section[Intro]{Introduction}
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Command(cmd) = &doc.body[0] {
        assert_eq!(cmd.name, "section");
        assert_eq!(cmd.args.len(), 2);
        assert!(matches!(&cmd.args[0], Arg::Optional(_)));
        assert!(matches!(&cmd.args[1], Arg::Braced(_)));
    } else {
        panic!("expected Command node in body");
    }
}

#[test]
fn test_parse_command_no_args() {
    let source = r"\begin{document}
\par
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Command(cmd) = &doc.body[0] {
        assert_eq!(cmd.name, "par");
        assert!(cmd.args.is_empty());
    } else {
        panic!("expected Command node in body");
    }
}

// ===== Environment parsing =====

#[test]
fn test_parse_simple_environment() {
    let source = r"\begin{document}
\begin{itemize}
\end{itemize}
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Environment(env) = &doc.body[0] {
        assert_eq!(env.name, "itemize");
        assert!(env.end_span.is_some());
        assert!(env.body.is_empty());
    } else {
        panic!("expected Environment node in body");
    }
}

#[test]
fn test_parse_nested_environments() {
    let source = r"\begin{document}
\begin{itemize}
\begin{itemize}
\end{itemize}
\end{itemize}
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Environment(outer) = &doc.body[0] {
        assert_eq!(outer.name, "itemize");
        assert_eq!(outer.body.len(), 1);
        if let DocumentBodyNode::Environment(inner) = &outer.body[0] {
            assert_eq!(inner.name, "itemize");
            assert!(inner.body.is_empty());
        } else {
            panic!("expected nested Environment node");
        }
    } else {
        panic!("expected Environment node in body");
    }
}

#[test]
fn test_parse_environment_with_text_body() {
    let source = r"\begin{document}
\begin{quote}
Some quoted text here
\end{quote}
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Environment(env) = &doc.body[0] {
        assert_eq!(env.name, "quote");
        let texts: Vec<&str> = env
            .body
            .iter()
            .filter_map(|n| match n {
                DocumentBodyNode::Text(t) => Some(t.text),
                _ => None,
            })
            .collect();
        assert!(!texts.is_empty());
        assert_eq!(texts.join(" "), "Some quoted text here");
    } else {
        panic!("expected Environment node in body");
    }
}

#[test]
fn test_parse_unclosed_environment() {
    let source = r"\begin{document}
\begin{itemize}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Environment(env) = &doc.body[0] {
        assert_eq!(env.name, "itemize");
        assert!(env.end_span.is_none());
    } else {
        panic!("expected Environment node in body");
    }
}

// ===== Math parsing =====

#[test]
fn test_parse_inline_math() {
    let source = r"\begin{document}
$x + 1$
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Math(math) = &doc.body[0] {
        assert_eq!(math.delimiter, MathDelimiter::Dollar);
    } else {
        panic!("expected Math node in body");
    }
}

#[test]
fn test_parse_display_math() {
    let source = r"\begin{document}
$$\int$$
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Math(math) = &doc.body[0] {
        assert_eq!(math.delimiter, MathDelimiter::DoubleDollar);
    } else {
        panic!("expected Math node in body");
    }
}

// ===== Text and Comment =====

#[test]
fn test_parse_text_node() {
    let source = r"\begin{document}
some text
\end{document}";
    let doc = parse(source);
    let texts: Vec<&str> = doc
        .body
        .iter()
        .filter_map(|n| match n {
            DocumentBodyNode::Text(t) => Some(t.text),
            _ => None,
        })
        .collect();
    assert!(!texts.is_empty());
    assert_eq!(texts.join(" "), "some text");
}

#[test]
fn test_parse_comment_node() {
    let source = r"\begin{document}
% a LaTeX comment
\end{document}";
    let doc = parse(source);
    assert_eq!(doc.body.len(), 1);
    if let DocumentBodyNode::Comment(comment) = &doc.body[0] {
        assert_eq!(comment.comment_text, "% a LaTeX comment");
    } else {
        panic!("expected Comment node in body");
    }
}
