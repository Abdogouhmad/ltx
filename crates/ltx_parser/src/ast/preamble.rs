//! Preamble AST node representation.

use ltx_lexer::LtxTokenKind;

use crate::ast::document_class::DocumentClassDecl;
use crate::ast::use_package::UsePackage;
use crate::ast::{Command, Comment, Group, Text};
use crate::parser::LtxParser;
use crate::parser_traits::Parse;

/// Items allowed in the LaTeX document preamble (before `\begin{document}`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreambleItem<'src> {
    DocumentClass(DocumentClassDecl<'src>),
    UsePackage(UsePackage<'src>),
    Command(Command<'src>),
    Text(Text<'src>),
    Comment(Comment<'src>),
    Group(Group<'src>),
}

impl<'src> Parse<'src> for PreambleItem<'src> {
    fn parse(parser: &mut LtxParser<'src>) -> Self {
        parser.skip_ws();
        match parser.peek_kind() {
            Some(LtxTokenKind::DocumentClass(_)) => Self::DocumentClass(parser.parse()),
            Some(LtxTokenKind::Command("documentclass")) => Self::DocumentClass(parser.parse()),
            Some(LtxTokenKind::Command("usepackage")) => Self::UsePackage(parser.parse()),
            Some(LtxTokenKind::Command(_)) => Self::Command(parser.parse()),
            Some(LtxTokenKind::Comment) => Self::Comment(parser.parse()),
            Some(LtxTokenKind::GroupStart) => Self::Group(parser.parse()),
            Some(LtxTokenKind::Error(_)) => {
                if let Some(tok) = parser.peek_at(0) {
                    if tok.text.starts_with("\\documentclass") {
                        return Self::DocumentClass(parser.parse());
                    }
                    if tok.text.starts_with("\\usepackage") {
                        return Self::UsePackage(parser.parse());
                    }
                }
                Self::Text(parser.parse())
            }
            _ => Self::Text(parser.parse()),
        }
    }
}
