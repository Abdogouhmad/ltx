//! Environment AST node.

use std::ops::Range;

use ltx_diagnostics::LtxSpan;
use ltx_lexer::{LtxToken, LtxTokenKind, TokenStream};

use crate::ast::body_node::DocumentBodyNode;
use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// Represents a `\begin{...} ... \end{...}` LaTeX environment.
///
/// Stores both structured child AST nodes in `body` and raw token range in `raw_range`.
/// Zero tokens cloned during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment<'src> {
    pub span: LtxSpan,
    pub name: &'src str,
    pub begin_span: LtxSpan,
    pub end_span: Option<LtxSpan>,
    pub body: Vec<DocumentBodyNode<'src>>,
    /// Token indices of the inner environment body (start inclusive, end exclusive).
    pub raw_range: Range<usize>,
}

impl<'src> Environment<'src> {
    /// Iterate over raw references to non-trivia tokens within this environment.
    #[inline]
    pub fn body_tokens<'a>(
        &self,
        stream: &'a TokenStream<'src>,
    ) -> impl Iterator<Item = &'a LtxToken<'src>> + 'a {
        self.raw_range.clone().filter_map(move |i| {
            stream.get(i).filter(|token| {
                !matches!(
                    token.kind,
                    LtxTokenKind::WhiteSpace | LtxTokenKind::EndOfLine
                )
            })
        })
    }

    fn parse_body(
        parser: &mut LtxParser<'src>,
        env_name: &'src str,
        begin_span: LtxSpan,
    ) -> (Vec<DocumentBodyNode<'src>>, Range<usize>, Option<LtxSpan>) {
        let body_start = parser.current_cursor();
        let mut nodes = Vec::new();
        let mut end_span = None;

        loop {
            parser.skip_ws();
            match parser.peek_kind() {
                None => {
                    parser.error_handler_mut().unclosed_environment(
                        env_name,
                        begin_span.start(),
                        begin_span.end(),
                    );
                    break;
                }
                Some(LtxTokenKind::EndEnv(name)) if *name == env_name => {
                    end_span = parser.bump().map(|t| t.span);
                    break;
                }
                Some(LtxTokenKind::EndEnv(found)) => {
                    let found = *found;
                    let (s, e) = parser
                        .peek_at(0)
                        .map_or((0, 0), |t| (t.span.start(), t.span.end()));
                    parser
                        .error_handler_mut()
                        .mismatched_environment(env_name, found, s, e);
                    break;
                }
                _ => {
                    nodes.push(parser.parse::<DocumentBodyNode<'src>>());
                }
            }
        }

        parser.pop_env();
        let body_end = parser.current_cursor();
        (nodes, body_start..body_end, end_span)
    }
}

impl<'src> Parse<'src> for Environment<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let (name, begin_span) = match parser.expect("environment", |kind| {
            matches!(kind, LtxTokenKind::BeginEnv(_))
        }) {
            Some(token) => {
                let name = match token.kind {
                    LtxTokenKind::BeginEnv(name) => name,
                    _ => unreachable!("expect verified kind"),
                };

                (name, token.span)
            }
            None => {
                let span = parser.dummy_span();
                return Self {
                    span,
                    name: "",
                    begin_span: span,
                    end_span: None,
                    body: Vec::new(),
                    raw_range: 0..0,
                };
            }
        };

        parser.push_env(name, begin_span);

        let (body, raw_range, end_span) = Self::parse_body(parser, name, begin_span);

        let span = LtxSpan::new(
            begin_span.start(),
            end_span.map_or(begin_span.end(), |span| span.end()),
            begin_span.file_id,
        );

        Self {
            span,
            name,
            begin_span,
            end_span,
            body,
            raw_range,
        }
    }
}
