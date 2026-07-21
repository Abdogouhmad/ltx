//! Document body AST node representation.

use ltx_lexer::LtxTokenKind;

use crate::ast::{Command, Comment, Environment, Group, Math, Text};
use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// Enum representing any node that can appear within a document body or environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentBodyNode<'src> {
    Environment(Environment<'src>),
    Command(Command<'src>),
    Text(Text<'src>),
    Math(Math<'src>),
    Comment(Comment<'src>),
    Group(Group<'src>),
}

impl<'src> Parse<'src> for DocumentBodyNode<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        parser.skip_ws();
        match parser.peek_kind() {
            Some(LtxTokenKind::BeginEnv(_)) => Self::Environment(parser.parse()),
            Some(LtxTokenKind::Command(_)) => Self::Command(parser.parse()),
            Some(LtxTokenKind::MathStart(_)) => Self::Math(parser.parse()),
            Some(LtxTokenKind::Comment) => Self::Comment(parser.parse()),
            Some(LtxTokenKind::GroupStart) => Self::Group(parser.parse()),
            _ => Self::Text(parser.parse()),
        }
    }
}
