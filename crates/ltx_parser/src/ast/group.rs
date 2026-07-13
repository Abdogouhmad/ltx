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
    /// On error (wrong token, unterminated group) emits a diagnostic and
    /// returns a partial group instead of panicking.
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let mut tokens = Vec::new();

        // opening brace — graceful if not found
        let open = match parser.expect("`{`", |k| matches!(k, LtxTokenKind::GroupStart)) {
            Some(tok) => tok.clone(),
            None => {
                let pos = parser.checkpoint();
                return Self {
                    span: LtxSpan::new(pos, pos, ltx_diagnostics::LtxFileId(0)),
                    tokens,
                };
            }
        };
        let open_span = open.span;
        tokens.push(open);

        let mut depth = 1usize;
        while depth > 0 {
            match parser.peek_kind() {
                None => {
                    parser.error_handler_mut().missing_closing_brace(open_span.start());
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
