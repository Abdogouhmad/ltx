use crate::LtxParser;
use crate::parser_traits::Parse;
use ltx_diagnostics::LtxSpan;
use ltx_lexer::LtxTokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment<'src> {
    pub span: LtxSpan,
    pub comment_text: &'src str,
}

impl<'src> Parse<'src> for Comment<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let Some(token) = parser.expect("comment token", |k| matches!(k, LtxTokenKind::Comment))
        else {
            let pos = parser.checkpoint();
            return Self {
                span: LtxSpan::new(pos, pos, ltx_diagnostics::LtxFileId(0)),
                comment_text: "",
            };
        };
        Self {
            span: token.span,
            comment_text: token.text,
        }
    }
}
