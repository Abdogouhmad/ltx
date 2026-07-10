//! Math-mode AST node.

use ltx_diagnostics::LtxSpan;
use ltx_lexer::MathDelimiter;

use crate::ast::Node;

/// A `$...$` or `$$...$$` math region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathGroup<'src> {
    /// Span covering the opening through the closing delimiter.
    pub span: LtxSpan,
    /// Which delimiter opened this region (`$` vs `$$`).
    pub delimiter: MathDelimiter,
    /// Parsed children.
    pub children: Vec<Node<'src>>,
}
