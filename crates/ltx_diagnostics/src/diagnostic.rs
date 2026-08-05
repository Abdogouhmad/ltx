use std::fmt;
use std::sync::Arc;

use crate::{LtxSeverity, LtxSourceMap, LtxSpan};
use miette::{Diagnostic, LabeledSpan, SourceCode, SourceSpan};

/// A source-located error that [`LtxDiagnostic`] can render.
///
/// Implement this trait for any crate-specific error enum (lexer, parser,
/// config, compiler) so it can flow through the shared diagnostic
/// infrastructure. The only requirement beyond [`miette::Diagnostic`] is that
/// the type can report the primary [`LtxSpan`] it points at.
pub trait LtxDiagnosticSource: Diagnostic + Send + Sync + 'static {
    /// The primary source span this diagnostic points at.
    fn span(&self) -> LtxSpan;
}

/// A diagnostic error, wrapping a [`LtxDiagnosticSource`] and the source map
/// needed to render it.
#[derive(Clone)]
pub struct LtxDiagnostic {
    /// The underlying language error.
    pub error: Arc<dyn LtxDiagnosticSource>,
    /// The source map used to resolve diagnostic spans.
    pub source_map: Arc<LtxSourceMap>,
}

impl LtxDiagnostic {
    /// Creates a diagnostic from an error and the source map used to render it.
    ///
    /// # Arguments
    ///
    /// * `error` - Any crate-specific error implementing [`LtxDiagnosticSource`].
    /// * `source_map` - The shared source map resolving the error's span.
    ///
    /// # Returns
    ///
    /// A new `LtxDiagnostic` wrapping `error` and `source_map`.
    #[must_use]
    #[inline]
    pub fn new<E>(error: E, source_map: Arc<LtxSourceMap>) -> Self
    where
        E: LtxDiagnosticSource,
    {
        Self {
            error: Arc::new(error),
            source_map,
        }
    }

    /// Returns the [`LtxSpan`] of the diagnostic error.
    ///
    /// # Returns
    ///
    /// The primary span the underlying error points at.
    #[must_use]
    #[inline]
    pub fn span(&self) -> LtxSpan {
        self.error.span()
    }

    /// Returns the severity of the diagnostic error.
    ///
    /// Delegates to the underlying error's miette attribute
    /// (`#[diagnostic(severity(...))]`) and maps it back to [`LtxSeverity`].
    ///
    /// # Returns
    ///
    /// The resolved [`LtxSeverity`] (defaults to `Error`).
    #[must_use]
    #[inline]
    pub fn severity(&self) -> LtxSeverity {
        self.error
            .severity()
            .map_or(LtxSeverity::Error, LtxSeverity::from_miette)
    }
}

impl Diagnostic for LtxDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.error.code()
    }
    fn severity(&self) -> Option<miette::Severity> {
        self.error.severity()
    }
    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.error.help()
    }
    fn url<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
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

impl fmt::Display for LtxDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl fmt::Debug for LtxDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LtxDiagnostic")
            .field("error", &format_args!("{}", self.error))
            .field("source_map", &self.source_map)
            .finish()
    }
}

impl std::error::Error for LtxDiagnostic {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
}

impl<E: LtxDiagnosticSource> From<(E, Arc<LtxSourceMap>)> for LtxDiagnostic {
    fn from((error, source_map): (E, Arc<LtxSourceMap>)) -> Self {
        Self::new(error, source_map)
    }
}
