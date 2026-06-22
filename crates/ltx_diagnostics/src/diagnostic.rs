use std::sync::Arc;

use crate::errors::{LexerError, ParserError};
use crate::{LtxSeverity, LtxSourceMap, LtxSpan};
use miette::{Diagnostic, LabeledSpan, SourceCode, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone)]
/// A diagnostic error, wrapping an [`LtxDiagnosticInner`] variant and source map.
pub struct LtxDiagnostic {
    /// The inner diagnostic error variant.
    pub inner: LtxDiagnosticInner,
    /// The source map used to resolve diagnostic spans.
    pub source_map: Arc<LtxSourceMap>,
}

#[derive(Debug, Error, Diagnostic, Clone)]
/// The inner diagnostic error variant.
pub enum LtxDiagnosticInner {
    /// A lexer error, wrapping the [`LexerError`] variant.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Lexer(#[from] LexerError),

    /// A parser error, wrapping the [`ParserError`] variant.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parser(#[from] ParserError),
}

impl LtxDiagnostic {
    /// Creates a new [`LtxDiagnostic`] with the given inner variant and source map.
    /// Returns the new [`LtxDiagnostic`] instance.
    #[must_use]
    #[inline]
    pub fn new(inner: LtxDiagnosticInner, source_map: Arc<LtxSourceMap>) -> Self {
        Self { inner, source_map }
    }

    /// Returns the span of the diagnostic error.
    /// Returns the [`LtxSpan`] of the diagnostic error.
    #[must_use]
    #[inline]
    pub fn span(&self) -> LtxSpan {
        match &self.inner {
            LtxDiagnosticInner::Lexer(e) => e.span(),
            LtxDiagnosticInner::Parser(e) => e.span(),
        }
    }

    /// Returns the severity of the diagnostic error.
    /// Returns the [`LtxSeverity`] of the diagnostic error.
    #[must_use]
    #[inline]
    pub const fn severity(&self) -> LtxSeverity {
        match self.inner {
            LtxDiagnosticInner::Lexer(_) | LtxDiagnosticInner::Parser(_) => LtxSeverity::Error,
        }
    }
}

impl Diagnostic for LtxDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.inner.code()
    }

    fn severity(&self) -> Option<miette::Severity> {
        self.inner.severity()
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.inner.help()
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        let file = self.source_map.get_file(self.span().file_id)?;
        // Return a reference to the cached NamedSource!
        Some(&file.named_source)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let span: SourceSpan = self.span().into();
        Some(Box::new(std::iter::once(LabeledSpan::new_with_span(
            Some("here".into()),
            span,
        ))))
    }
}

impl std::fmt::Display for LtxDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for LtxDiagnostic {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}
