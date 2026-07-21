//! `\usepackage[options]{package}` AST node.

use ltx_diagnostics::LtxSpan;
use ltx_lexer::LtxTokenKind;

use crate::ast::arg::OptionalArg;
use crate::ast::Command;
use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// Represents a `\usepackage[options]{package}` preamble declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsePackage<'src> {
    pub span: LtxSpan,
    pub package_name: &'src str,
    pub options: Option<OptionalArg<'src>>,
}

impl<'src> Parse<'src> for UsePackage<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        parser.skip_ws();

        if matches!(
            parser.peek_kind(),
            Some(LtxTokenKind::Command("usepackage"))
        ) {
            let cmd = parser.parse::<Command<'src>>();
            let options = cmd.optional_args().next().cloned();
            let package_name = cmd
                .braced_args()
                .next()
                .and_then(|g| {
                    g.tokens(&parser.stream)
                        .find(|t| matches!(t.kind, LtxTokenKind::Text))
                        .map(|t| t.text)
                })
                .unwrap_or("");

            return Self {
                span: cmd.span,
                package_name,
                options,
            };
        }

        Self {
            span: parser.dummy_span(),
            package_name: "",
            options: None,
        }
    }
}
