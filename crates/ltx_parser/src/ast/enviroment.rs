//! Environment (`\begin{...} ... \end{...}`) AST node.

use ltx_diagnostics::LtxSpan;

use crate::ast::Node;

/// A `\begin{name} ... \end{name}` environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment<'src> {
    /// Environment name (from `\begin{name}`).
    pub name: &'src str,
    /// Span covering `\begin{...}` through `\end{...}` (or through EOF).
    pub span: LtxSpan,
    /// Parsed children.
    pub children: Vec<Node<'src>>,
}
