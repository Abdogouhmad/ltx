//! Text node AST

use ltx_diagnostics::LtxSpan;

use crate::parser::LtxParser;
use crate::parser_traits::Parse;
use ltx_lexer::LtxTokenKind;

/// A run of plain text in the document body.
///
/// Produced from a single `LtxTokenKind::Text` token — the lexer already
/// merges consecutive text characters into one token, so no further
/// concatenation is needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<'src> {
    /// Span of this text run in the source.
    pub span: LtxSpan,
    /// The source text slice for this text run.
    pub text: &'src str,
}

impl<'src> Parse<'src> for Text<'src> {
    /// Consume one `Text` token and produce the corresponding `Text` node.
    ///
    /// On error emits a diagnostic and returns an empty node instead of panicking.
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        match parser.expect("Text token", |k| matches!(k, LtxTokenKind::Text)) {
            Some(token) => Self {
                span: token.span,
                text: token.text,
            },
            None => {
                let pos = parser.checkpoint();
                Self {
                    span: LtxSpan::new(pos, pos, ltx_diagnostics::LtxFileId(0)),
                    text: "",
                }
            }
        }
    }
}
