//! Command node AST

use ltx_diagnostics::LtxSpan;

use crate::ast::Group;
use crate::parser::LtxParser;
use crate::parser_traits::Parse;
use ltx_lexer::LtxTokenKind;

/// A control sequence (e.g. `\section`, `\textbf`, `\LaTeX`) and its arguments.
///
/// The `name` field stores the control-sequence name without the leading
/// backslash (e.g. `"section"`, `"textbf"`, `"LaTeX"`).  Arguments are
/// parsed immediately after the command, consuming any braced groups that
/// follow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command<'src> {
    /// Span of the control-sequence token itself (not including arguments).
    pub span: LtxSpan,
    /// The control-sequence name (without the backslash).
    pub name: &'src str,
    /// Braced arguments following the command, in source order.
    pub args: Vec<Group<'src>>,
}

impl<'src> Parse<'src> for Command<'src> {
    /// Consume one `Command` token and any immediately following braced groups.
    ///
    /// On error emits a diagnostic and returns a partial node instead of panicking.
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        let (span, name) = match parser.expect("Command token", |k| matches!(k, LtxTokenKind::Command(_))) {
            Some(token) => {
                let name = match &token.kind {
                    LtxTokenKind::Command(name) => *name,
                    _ => unreachable!("expect verified the kind"),
                };
                (token.span, name)
            }
            None => {
                let pos = parser.checkpoint();
                return Self {
                    span: LtxSpan::new(pos, pos, ltx_diagnostics::LtxFileId(0)),
                    name: "",
                    args: Vec::new(),
                };
            }
        };

        // parse consecutive {…} arguments (optional whitespace allowed between them)
        let mut args = Vec::new();
        loop {
            parser.skip_ws();
            if !matches!(parser.peek_kind(), Some(LtxTokenKind::GroupStart)) {
                break;
            }
            args.push(parser.parse::<Group<'src>>());
        }

        Self { span, name, args }
    }
}
