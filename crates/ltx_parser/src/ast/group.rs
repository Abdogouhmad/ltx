//! Group node AST.

use std::marker::PhantomData;
use std::ops::Range;

use ltx_diagnostics::LtxSpan;
use ltx_lexer::{LtxToken, LtxTokenKind, TokenStream};

use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// A balanced braced group `{ … }`.
///
/// Stores token index range inside the parent [`TokenStream`].
/// Zero cloning or extra allocation during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group<'src> {
    /// Span covering the entire group from `{` to `}`.
    pub span: LtxSpan,
    /// Token indices inside the parent stream (start inclusive, end exclusive).
    pub tokens: Range<usize>,
    _marker: PhantomData<&'src ()>,
}

impl<'src> Group<'src> {
    /// Iterate over references to tokens within this group without cloning.
    #[inline]
    pub fn tokens<'a>(
        &self,
        stream: &'a TokenStream<'src>,
    ) -> impl Iterator<Item = &'a LtxToken<'src>> + 'a {
        self.tokens.clone().filter_map(move |i| stream.get(i))
    }
}

impl<'src> Parse<'src> for Group<'src> {
    /// Consume tokens from an opening `{` up to and including matching `}`.
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let start_idx = parser.current_cursor();

        let open_span = match parser.expect("`{`", |k| matches!(k, LtxTokenKind::GroupStart)) {
            Some(tok) => tok.span,
            None => {
                return Self {
                    span: parser.dummy_span(),
                    tokens: start_idx..start_idx,
                    _marker: PhantomData,
                };
            }
        };

        let mut depth = 1usize;
        let mut close_span = open_span;

        while depth > 0 {
            parser.skip_ws();
            match parser.peek_kind() {
                None => {
                    parser
                        .error_handler_mut()
                        .missing_closing_brace(open_span.start());
                    break;
                }
                Some(LtxTokenKind::GroupStart) => depth += 1,
                Some(LtxTokenKind::GroupEnd) => depth -= 1,
                Some(_) => {}
            }

            if let Some(tok) = parser.bump() {
                close_span = tok.span;
            }
        }

        let end_idx = parser.current_cursor();
        let span = LtxSpan::new(open_span.start(), close_span.end(), open_span.file_id);

        Self {
            span,
            tokens: start_idx..end_idx,
            _marker: PhantomData,
        }
    }
}
