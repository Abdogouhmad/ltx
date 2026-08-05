//! Command node AST.

use ltx_diagnostics::LtxSpan;
use ltx_lexer::LtxTokenKind;

use crate::ast::Group;
use crate::ast::arg::{Arg, OptionalArg};
use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// A control sequence (e.g. `\section`, `\textbf`, `\usepackage`) and its arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command<'src> {
    /// Span of the command itself and all trailing parsed arguments.
    pub span: LtxSpan,
    /// The control-sequence name (without leading backslash).
    pub name: &'src str,
    /// Parsed command arguments in source order.
    pub args: Vec<Arg<'src>>,
}

impl<'src> Command<'src> {
    /// Iterator over braced `{...}` arguments.
    #[inline]
    pub fn braced_args(&self) -> impl Iterator<Item = &Group<'src>> {
        self.args.iter().filter_map(|arg| {
            if let Arg::Braced(g) = arg {
                Some(g)
            } else {
                None
            }
        })
    }

    /// Iterator over optional `[...]` arguments.
    #[inline]
    pub fn optional_args(&self) -> impl Iterator<Item = &OptionalArg<'src>> {
        self.args.iter().filter_map(|arg| {
            if let Arg::Optional(opt) = arg {
                Some(opt)
            } else {
                None
            }
        })
    }
}

impl<'src> Parse<'src> for Command<'src> {
    #[allow(clippy::unwrap_used)]
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let (mut span, name) = {
            let Some(token) =
                parser.expect("Command token", |k| matches!(k, LtxTokenKind::Command(_)))
            else {
                return Self {
                    span: parser.dummy_span(),
                    name: "",
                    args: Vec::new(),
                };
            };
            let LtxTokenKind::Command(name) = token.kind else {
                unreachable!("expect verified kind");
            };
            (token.span, name)
        };

        let mut args = Vec::new();
        loop {
            parser.skip_ws();
            if let Some(opt) = OptionalArg::parse_optional(parser) {
                span = LtxSpan::new(span.start(), opt.span.end(), span.file_id);
                args.push(Arg::Optional(opt));
            } else if matches!(parser.peek_kind(), Some(LtxTokenKind::GroupStart)) {
                let group = parser.parse::<Group<'src>>();
                span = LtxSpan::new(span.start(), group.span.end(), span.file_id);
                args.push(Arg::Braced(group));
            } else {
                break;
            }
        }

        Self { span, name, args }
    }
}
