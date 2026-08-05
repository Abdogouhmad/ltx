//! Argument AST nodes: optional `[...]` and braced `{...}` arguments.

use std::ops::Range;

use ltx_diagnostics::LtxSpan;
use ltx_lexer::LtxTokenKind;

use crate::ast::Group;
use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// An optional bracketed argument `[options]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalArg<'src> {
    /// Span covering from `[` to `]`.
    pub span: LtxSpan,
    /// Inner text within brackets.
    pub text: &'src str,
    /// Token range inside the parent [`TokenStream`](ltx_lexer::TokenStream).
    pub tokens: Range<usize>,
}

impl<'src> OptionalArg<'src> {
    /// Parse an optional `[...]` argument if present at the current parser position.
    /// Returns `None` without advancing the cursor if `[` is not present.
    #[allow(unsafe_code, clippy::unwrap_used)]
    pub fn parse_optional(parser: &mut LtxParser<'src>) -> Option<Self> {
        parser.skip_ws();
        let tok = parser.peek_kind()?;
        if !matches!(tok, LtxTokenKind::Text) {
            return None;
        }

        let peeked_tok = parser.stream.peek()?;
        if !peeked_tok.text.starts_with('[') {
            return None;
        }

        let start_pos = parser.current_cursor();
        let first_tok = parser.bump().unwrap();
        let start_span = first_tok.span;
        let start_ptr = first_tok.text.as_ptr();

        let mut end_span = start_span;

        if !(first_tok.text.starts_with('[')
            && first_tok.text.ends_with(']')
            && first_tok.text.len() >= 2)
        {
            while !parser.at_eof() {
                let Some(t) = parser.peek_at(0) else {
                    break;
                };

                end_span = t.span;
                let contains_close = t.text.contains(']');
                parser.bump();
                if contains_close {
                    break;
                }
            }
        }

        let end_pos = parser.current_cursor();
        let span = LtxSpan::new(start_span.start(), end_span.end(), start_span.file_id);

        let slice_len = end_span.end() - start_span.start();

        // SAFETY: `start_ptr` is derived from `first_tok.text.as_ptr()`, which points
        // into the original `&'src str` source string. All tokens in the stream borrow
        // from the same contiguous source, so `start_ptr + slice_len` addresses a valid
        // region within that source. We validate the invariant with debug assertions.
        debug_assert!(
            !start_ptr.is_null(),
            "start_ptr must be non-null (derived from source string)"
        );
        debug_assert!(
            slice_len > 0,
            "slice_len must be positive for a valid bracket argument"
        );
        debug_assert!(
            slice_len <= 10_000,
            "slice_len suspiciously large ({slice_len}), possible span arithmetic bug"
        );

        let raw_str = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(start_ptr, slice_len))
        };
        let inner = raw_str
            .strip_prefix('[')
            .unwrap_or(raw_str)
            .strip_suffix(']')
            .unwrap_or(raw_str);

        Some(Self {
            span,
            text: inner,
            tokens: start_pos..end_pos,
        })
    }
}

/// Unified command argument representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg<'src> {
    /// Standard braced `{...}` group argument.
    Braced(Group<'src>),
    /// Optional `[...]` bracket argument.
    Optional(OptionalArg<'src>),
}

impl<'src> Parse<'src> for Arg<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        parser.skip_ws();
        OptionalArg::parse_optional(parser).map_or_else(
            || Self::Braced(parser.parse::<Group<'src>>()),
            Self::Optional,
        )
    }
}
