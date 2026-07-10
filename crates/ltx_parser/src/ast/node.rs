//! the AST node for the parser's recursion unit.

use ltx_diagnostics::LtxSpan;

use crate::ast::{Command, Environment, Group, MathGroup, Text};

/// A single node in the parsed document tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'src> {
    /// Plain text, whitespace, or end-of-line content.
    Text(Text<'src>),
    /// A control sequence (`\foo`), currently argument-less.
    Command(Command<'src>),
    /// A `{ ... }` group.
    Group(Group<'src>),
    /// A `$...$` / `$$...$$` math region.
    Math(MathGroup<'src>),
    /// A `\begin{...} ... \end{...}` environment.
    Environment(Environment<'src>),
    /// A recovery placeholder for malformed input the parser chose to
    /// skip past rather than abort on.
    Error(ErrorNode),
}

impl<'src> Node<'src> {
    /// The span this node covers in the source file.
    #[must_use]
    pub const fn span(&self) -> LtxSpan {
        match self {
            Self::Text(t) => t.span,
            Self::Command(c) => c.span,
            Self::Group(g) => g.span,
            Self::Math(m) => m.span,
            Self::Environment(e) => e.span,
            Self::Error(e) => e.span,
        }
    }
}

/// Emitted in place of a "real" node when the parser can't make sense of
/// the input but wants to keep going instead of failing hard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorNode {
    /// Location of the problem.
    pub span: LtxSpan,
    /// Human-readable description (diagnostics themselves go through the
    /// error handler; this is just for tree inspection/debugging).
    pub message: String,
}
