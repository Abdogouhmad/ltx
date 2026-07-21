//! Text node AST.

use ltx_diagnostics::LtxSpan;
use ltx_lexer::LtxTokenKind;

use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// A run of plain text in the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<'src> {
    /// Span of this text run in the source.
    pub span: LtxSpan,
    /// The source text slice for this text run.
    pub text: &'src str,
}

impl<'src> Parse<'src> for Text<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        if matches!(
            parser.peek_kind(),
            Some(LtxTokenKind::Text | LtxTokenKind::Error(_))
        ) {
            let tok = parser.bump().unwrap();
            Self {
                span: tok.span,
                text: tok.text,
            }
        } else {
            Self {
                span: parser.dummy_span(),
                text: "",
            }
        }
    }
}
