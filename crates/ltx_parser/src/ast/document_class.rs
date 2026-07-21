//! `\documentclass[options]{class}` AST node.

use ltx_diagnostics::LtxSpan;
use ltx_lexer::LtxTokenKind;

use crate::ast::arg::OptionalArg;
use crate::ast::Group;
use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// Represents a `\documentclass[options]{class}` preamble declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentClassDecl<'src> {
    pub span: LtxSpan,
    pub class_name: &'src str,
    pub options: Option<OptionalArg<'src>>,
}

impl<'src> Parse<'src> for DocumentClassDecl<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        parser.skip_ws();

        if matches!(
            parser.peek_kind(),
            Some(LtxTokenKind::DocumentClass(_))
        ) {
            let cur_idx = parser.current_cursor();
            let tok = parser.bump().unwrap();
            let name = match tok.kind {
                LtxTokenKind::DocumentClass(name) => name,
                _ => "",
            };

            let options = if let (Some(s_idx), Some(e_idx)) = (tok.text.find('['), tok.text.find(']')) {
                if s_idx < e_idx {
                    let opt_text = &tok.text[s_idx + 1..e_idx];
                    let opt_span = LtxSpan::new(
                        tok.span.start() + s_idx,
                        tok.span.start() + e_idx + 1,
                        tok.span.file_id,
                    );
                    Some(OptionalArg {
                        span: opt_span,
                        text: opt_text,
                        tokens: cur_idx..cur_idx + 1,
                    })
                } else {
                    None
                }
            } else {
                None
            };

            return Self {
                span: tok.span,
                class_name: name,
                options,
            };
        }

        let is_doc_class_cmd = matches!(
            parser.peek_kind(),
            Some(LtxTokenKind::Command("documentclass"))
        ) || parser
            .peek_at(0)
            .is_some_and(|t| t.text.starts_with("\\documentclass"));

        if is_doc_class_cmd {
            let start_tok = parser.bump().unwrap();
            let mut span = start_tok.span;

            let options = OptionalArg::parse_optional(parser);
            if let Some(ref opt) = options {
                span = LtxSpan::new(span.start(), opt.span.end(), span.file_id);
            }

            parser.skip_ws();
            let (class_name, group_span) =
                if matches!(parser.peek_kind(), Some(LtxTokenKind::GroupStart)) {
                    let group = parser.parse::<Group<'src>>();
                    let name = group
                        .tokens(&parser.stream)
                        .find(|t| matches!(t.kind, LtxTokenKind::Text))
                        .map_or("", |t| t.text);
                    (name, Some(group.span))
                } else {
                    ("", None)
                };

            if let Some(g_span) = group_span {
                span = LtxSpan::new(span.start(), g_span.end(), span.file_id);
            }

            return Self {
                span,
                class_name,
                options,
            };
        }

        Self {
            span: parser.dummy_span(),
            class_name: "",
            options: None,
        }
    }
}
