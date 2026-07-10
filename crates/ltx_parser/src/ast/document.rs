//! Document root AST node.

use ltx_diagnostics::LtxSpan;

use crate::ast::Node;

/// The root of a parsed `.tex` source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document<'src> {
    /// The `\documentclass{...}` name, if declared.
    pub class: Option<&'src str>,
    /// Span of the `\documentclass{...}` declaration, if present.
    pub class_span: Option<LtxSpan>,
    /// Top-level parsed nodes.
    pub children: Vec<Node<'src>>,
}
