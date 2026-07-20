//! Environment AST node.

use std::ops::Range;

use crate::parser::LtxParser;
use crate::parser_traits::Parse;
use ltx_diagnostics::{LtxFileId, LtxSpan};
use ltx_lexer::{LtxToken, LtxTokenKind, TokenStream};

/// Represents a `\begin{...} ... \end{...}` environment.
///
/// The body stores only the token index range inside the parent
/// [`TokenStream`]. No tokens are cloned or allocated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment<'src> {
    pub span: LtxSpan,
    pub name: &'src str,
    pub begin_span: LtxSpan,
    pub end_span: Option<LtxSpan>,
    /// Token indices of the body (start inclusive, end exclusive).
    pub body: Range<usize>,
}

impl<'src> Environment<'src> {
    #[inline]
    pub fn body_tokens<'a>(
        &self,
        stream: &'a TokenStream<'src>,
    ) -> impl Iterator<Item = &'a LtxToken<'src>> + 'a {
        self.body.clone().filter_map(move |i| {
            stream.get(i).filter(|token| {
                !matches!(
                    token.kind,
                    LtxTokenKind::WhiteSpace | LtxTokenKind::EndOfLine
                )
            })
        })
    }

    #[inline]
    fn dummy(pos: usize) -> Self {
        let span = LtxSpan::new(pos, pos, LtxFileId(0));

        Self {
            span,
            name: "",
            begin_span: span,
            end_span: None,
            body: pos..pos,
        }
    }

    fn parse_body(
        parser: &mut LtxParser<'src>,
        env_name: &'src str,
        begin_span: LtxSpan,
    ) -> (Range<usize>, Option<LtxSpan>) {
        let body_start = parser.current_cursor();
        let mut body_end = body_start;
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
                    body_end = parser.current_cursor();
                    end_span = parser.bump().map(|t| t.span);
                    break;
                }
                Some(LtxTokenKind::EndEnv(found)) => {
                    body_end = parser.current_cursor();
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
                    parser.bump();
                }
            }
        }

        parser.pop_env();
        (body_start..body_end, end_span)
    }
}

impl<'src> Parse<'src> for Environment<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let (name, begin_span) = match parser.expect("environment", |kind| {
            matches!(kind, LtxTokenKind::BeginEnv(_))
        }) {
            Some(token) => {
                let LtxTokenKind::BeginEnv(name) = token.kind else {
                    unreachable!();
                };

                (name, token.span)
            }

            None => {
                return Self::dummy(parser.checkpoint());
            }
        };

        parser.push_env(name, begin_span);

        let (body, end_span) = Self::parse_body(parser, name, begin_span);

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
        }
    }
}
