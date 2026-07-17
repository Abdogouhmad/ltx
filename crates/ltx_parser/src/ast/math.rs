use crate::parser::LtxParser;
use crate::parser_traits::Parse;
use ltx_diagnostics::LtxSpan;
use ltx_lexer::{LtxToken, LtxTokenKind, MathDelimiter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Math<'src> {
    pub span: LtxSpan,
    pub delimiter: MathDelimiter,
    pub tokens: Vec<LtxToken<'src>>,
}

impl<'src> Parse<'src> for Math<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let open = match parser.expect("math delimiter", |k| {
            matches!(k, LtxTokenKind::MathStart(_))
        }) {
            Some(token) => {
                let delim = match &token.kind {
                    LtxTokenKind::MathStart(d) => *d,
                    _ => unreachable!("expect verified the kind"),
                };
                (token.span, delim)
            }
            None => {
                let pos = parser.checkpoint();
                return Self {
                    span: LtxSpan::new(pos, pos, ltx_diagnostics::LtxFileId(0)),
                    delimiter: MathDelimiter::Dollar,
                    tokens: Vec::new(),
                };
            }
        };

        let (open_span, delimiter) = open;
        let mut tokens = Vec::new();

        loop {
            parser.skip_ws();
            match parser.peek_kind() {
                None => {
                    parser.error_handler_mut().push_error(
                        ltx_diagnostics::LtxError::UnexpectedEOF {
                            found: "math mode".into(),
                            span: open_span,
                        },
                    );
                    break;
                }
                Some(LtxTokenKind::MathEnd(d)) if *d == delimiter => {
                    break;
                }
                _ => {}
            }
            tokens.push(parser.bump().unwrap().clone());
        }

        let close_span = parser.bump().map(|t| t.span).unwrap_or(open_span);

        let span = LtxSpan::new(open_span.start(), close_span.end(), open_span.file_id);

        Self {
            span,
            delimiter,
            tokens,
        }
    }
}
