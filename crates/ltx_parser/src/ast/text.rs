//! Text node AST.

use ltx_diagnostics::LtxSpan;

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
        // Text is the catch-all fallback: any unmatched token is consumed so
        // the surrounding loops always make progress (a stray `\end{...}` or
        // `}` in the preamble/body must never cause an infinite loop).
        let Some(tok) = parser.bump() else {
            return Self {
                span: parser.dummy_span(),
                text: "",
            };
        };
        Self {
            span: tok.span,
            text: tok.text,
        }
    }
}
