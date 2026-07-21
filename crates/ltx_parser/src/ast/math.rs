//! Math node AST.

use std::marker::PhantomData;
use std::ops::Range;

use ltx_diagnostics::LtxSpan;
use ltx_lexer::{LtxToken, LtxTokenKind, MathDelimiter, TokenStream};

use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// A LaTeX math expression `$ ... $` or `$$ ... $$`.
///
/// Stores token index range inside the parent [`TokenStream`].
/// Zero cloning or token allocations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Math<'src> {
    pub span: LtxSpan,
    pub delimiter: MathDelimiter,
    pub tokens: Range<usize>,
    _marker: PhantomData<&'src ()>,
}

impl<'src> Math<'src> {
    /// Iterate over references to tokens within this math block without cloning.
    #[inline]
    pub fn tokens<'a>(
        &self,
        stream: &'a TokenStream<'src>,
    ) -> impl Iterator<Item = &'a LtxToken<'src>> + 'a {
        self.tokens.clone().filter_map(move |i| stream.get(i))
    }
}

impl<'src> Parse<'src> for Math<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let start_idx = parser.current_cursor();

        let (open_span, delimiter) = match parser.expect("math delimiter", |k| {
            matches!(k, LtxTokenKind::MathStart(_))
        }) {
            Some(token) => {
                let delim = match &token.kind {
                    LtxTokenKind::MathStart(d) => *d,
                    _ => unreachable!("expect verified kind"),
                };
                (token.span, delim)
            }
            None => {
                return Self {
                    span: parser.dummy_span(),
                    delimiter: MathDelimiter::Dollar,
                    tokens: start_idx..start_idx,
                    _marker: PhantomData,
                };
            }
        };

        loop {
            parser.skip_ws();
            match parser.peek_kind() {
                None => {
                    parser.error_handler_mut().unexpected_eof(
                        "math mode",
                        open_span.start(),
                        open_span.end(),
                    );
                    break;
                }
                Some(LtxTokenKind::MathEnd(d)) if *d == delimiter => {
                    break;
                }
                _ => {}
            }
            parser.bump();
        }

        let close_span = parser.bump().map(|t| t.span).unwrap_or(open_span);
        let end_idx = parser.current_cursor();
        let span = LtxSpan::new(open_span.start(), close_span.end(), open_span.file_id);

        Self {
            span,
            delimiter,
            tokens: start_idx..end_idx,
            _marker: PhantomData,
        }
    }
}
