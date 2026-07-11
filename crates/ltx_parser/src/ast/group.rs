//! Group node AST

use ltx_diagnostics::LtxSpan;

use crate::parser::LtxParser;
use crate::parser_traits::Parse;
use ltx_lexer::{LtxToken, LtxTokenKind};

/// A balanced braced group `{ … }`.
///
/// Collects *all* tokens between (and including) the opening and closing
/// braces into `tokens`.  Downstream passes can walk the token slice to
/// build a more structured representation.
///
/// Allocation note: the tokens inside the group are cloned out of the
/// `TokenStream`'s internal buffer.  For a zero-copy alternative, store
/// the token indices and look them up through the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group<'src> {
    /// Span covering the entire group from `{` to `}`.
    pub span: LtxSpan,
    /// All tokens that make up this group, including `GroupStart` and `GroupEnd`.
    pub tokens: Vec<LtxToken<'src>>,
}

impl<'src> Parse<'src> for Group<'src> {
    /// Consume tokens from an opening `{` up to and including the matching `}`.
    ///
    /// Handles nested groups by tracking brace depth.  If EOF is reached
    /// before the group closes the unterminated diagnostic is emitted and
    /// parsing stops.
    ///
    /// # Panics
    ///
    /// Panics if the current token is not `GroupStart`.
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let mut tokens = Vec::new();

        // opening brace
        let open = parser.expect("`{`", |k| matches!(k, LtxTokenKind::GroupStart));
        let open_span = open.span;
        tokens.push(open.clone());

        let mut depth = 1usize;
        while depth > 0 {
            match parser.peek_kind() {
                None => {
                    parser.error_handler_mut().unterminated_argument(0, 0);
                    break;
                }
                Some(LtxTokenKind::GroupStart) => depth += 1,
                Some(LtxTokenKind::GroupEnd) => depth -= 1,
                Some(_) => {}
            }
            tokens.push(parser.bump().unwrap().clone());
        }

        let close_span = tokens.last().map(|t| t.span).unwrap_or(open_span);
        let span = LtxSpan::new(open_span.start(), close_span.end(), open_span.file_id);

        Self { span, tokens }
    }
}
