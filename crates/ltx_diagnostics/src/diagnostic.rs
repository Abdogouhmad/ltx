use std::sync::Arc;

use crate::{LtxError, LtxSeverity, LtxSourceMap, LtxSpan};
use miette::{Diagnostic, LabeledSpan, SourceCode, SourceSpan};

/// A diagnostic error, wrapping an [`LtxError`] and the source map needed to render it.
#[derive(Debug, Clone)]
pub struct LtxDiagnostic {
    /// The underlying language error.
    pub error: LtxError,
    /// The source map used to resolve diagnostic spans.
    pub source_map: Arc<LtxSourceMap>,
}

impl LtxDiagnostic {
    /// create a instance of error using error code + source map
    #[must_use]
    #[inline]
    pub const fn new(error: LtxError, source_map: Arc<LtxSourceMap>) -> Self {
        Self { error, source_map }
    }

    /// Returns the span of the diagnostic error.
    /// Returns the [`LtxSpan`] of the diagnostic error.
    #[must_use]
    #[inline]
    pub const fn span(&self) -> LtxSpan {
        self.error.span()
    }

    /// Returns the severity of the diagnostic error.
    ///
    /// Delegates to the underlying [`LtxError`]'s miette attribute
    /// (`#[diagnostic(severity(...))]`) and maps it back to [`LtxSeverity`].
    #[must_use]
    #[inline]
    pub fn severity(&self) -> LtxSeverity {
        self.error
            .severity()
            .map_or(LtxSeverity::Error, LtxSeverity::from_miette)
    }
}

impl Diagnostic for LtxDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.error.code()
    }
    fn severity(&self) -> Option<miette::Severity> {
        self.error.severity()
    }
    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.error.help()
    }
    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.error.url()
    }
    fn source_code(&self) -> Option<&dyn SourceCode> {
        let file = self.source_map.get_file(self.span().file_id)?;
        Some(file.named_source())
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
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for LtxDiagnostic {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
}

impl From<(LtxError, Arc<LtxSourceMap>)> for LtxDiagnostic {
    fn from((error, source_map): (LtxError, Arc<LtxSourceMap>)) -> Self {
        Self::new(error, source_map)
    }
}
