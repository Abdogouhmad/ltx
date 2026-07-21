//! Document root AST node.

use ltx_diagnostics::LtxSpan;
use ltx_lexer::LtxTokenKind;

use crate::ast::body_node::DocumentBodyNode;
use crate::ast::preamble::PreambleItem;
use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// Top-level root AST node for a complete LaTeX document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document<'src> {
    /// Span covering the complete parsed document.
    pub span: LtxSpan,
    /// Items occurring before `\begin{document}`.
    pub preamble: Vec<PreambleItem<'src>>,
    /// Nodes occurring inside `\begin{document} ... \end{document}`.
    pub body: Vec<DocumentBodyNode<'src>>,
}

impl<'src> Parse<'src> for Document<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let start_pos = parser.checkpoint();

        let mut preamble = Vec::new();
        let mut body = Vec::new();
        let mut found_doc_begin = false;
        let mut doc_begin_span = parser.dummy_span();
        let mut doc_end_span = None;

        // 1. Preamble loop
        while !parser.at_eof() {
            parser.skip_ws();
            if parser.at_eof() {
                break;
            }

            match parser.peek_kind() {
                Some(LtxTokenKind::BeginEnv("document")) => {
                    found_doc_begin = true;
                    if let Some(tok) = parser.bump() {
                        doc_begin_span = tok.span;
                    }
                    break;
                }
                _ => {
                    preamble.push(parser.parse::<PreambleItem<'src>>());
                }
            }
        }

        // Emit diagnostic if `\begin{document}` was never found
        if !found_doc_begin {
            parser
                .error_handler_mut()
                .unexpected_eof_parsing("document environment", start_pos);
            let end_pos = parser.checkpoint();
            return Self {
                span: LtxSpan::new(start_pos, end_pos, ltx_diagnostics::LtxFileId(0)),
                preamble,
                body,
            };
        }

        parser.push_env("document", doc_begin_span);

        // 2. Document body loop
        while !parser.at_eof() {
            parser.skip_ws();
            if parser.at_eof() {
                break;
            }

            match parser.peek_kind() {
                Some(LtxTokenKind::EndEnv("document")) => {
                    doc_end_span = parser.bump().map(|t| t.span);
                    break;
                }
                Some(LtxTokenKind::EndEnv(found)) => {
                    let found = *found;
                    let (s, e) = parser
                        .peek_at(0)
                        .map_or((0, 0), |t| (t.span.start(), t.span.end()));
                    parser.error_handler_mut().mismatched_environment(
                        "document",
                        found,
                        s,
                        e,
                    );
                    break;
                }
                _ => {
                    body.push(parser.parse::<DocumentBodyNode<'src>>());
                }
            }
        }

        if doc_end_span.is_none() {
            parser.error_handler_mut().unclosed_environment(
                "document",
                doc_begin_span.start(),
                doc_begin_span.end(),
            );
        }

        parser.pop_env();

        let end_pos = doc_end_span.map_or_else(|| parser.checkpoint(), |s| s.end());

        Self {
            span: LtxSpan::new(start_pos, end_pos, doc_begin_span.file_id),
            preamble,
            body,
        }
    }
}
