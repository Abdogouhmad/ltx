//! Linter errors and the diagnostic source used to report rule findings.

use std::borrow::Cow;
use std::fmt;

use ltx_diagnostics::{
    LtxDiagnostic, LtxDiagnosticSink, LtxDiagnosticSource, LtxSeverity, LtxSpan,
};
use miette::{Diagnostic, LabeledSpan, SourceSpan};
use thiserror::Error;

use crate::context::LintContext;

/// Errors raised while assembling a [`crate::LintRegistry`] from a
/// [`LintTable`](ltx_config::LintTable).
#[derive(Debug, Error, Diagnostic)]
pub enum LintError {
    /// A slug in the lint table doesn't match any registered rule.
    #[error("unknown lint rule `{slug}`")]
    #[diagnostic(
        code(LTX::LINTER::E018),
        severity(Error),
        help("check the slug against `ltx code --lint` and fix the `[lints]` table")
    )]
    UnknownRule {
        /// The offending slug.
        slug: Cow<'static, str>,
    },

    /// A slug appears in both `deny` and `allow`, which is contradictory.
    #[error("lint rule `{slug}` is configured in both `deny` and `allow`")]
    #[diagnostic(
        code(LTX::LINTER::E019),
        severity(Error),
        help("remove the rule from one of the two lists — the linter refuses to guess")
    )]
    ConflictingDirective {
        /// The conflicting slug.
        slug: Cow<'static, str>,
    },
}

/// A source-located lint finding, emitted into the [`LtxDiagnosticSink`].
///
/// The code and message are dynamic (each rule supplies them), so this is a
/// manual [`Diagnostic`] implementation rather than a derived enum.
#[derive(Debug, Clone)]
pub struct LintDiagnosticSource {
    /// The diagnostic code (e.g. `"W001"`).
    code: &'static str,
    /// The human-readable message.
    message: Cow<'static, str>,
    /// The effective severity.
    severity: LtxSeverity,
    /// The location this finding points at.
    span: LtxSpan,
}

impl LintDiagnosticSource {
    /// Creates a new lint diagnostic source.
    #[must_use]
    pub const fn new(
        code: &'static str,
        message: Cow<'static, str>,
        severity: LtxSeverity,
        span: LtxSpan,
    ) -> Self {
        Self {
            code,
            message,
            severity,
            span,
        }
    }
}

impl fmt::Display for LintDiagnosticSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for LintDiagnosticSource {}

impl Diagnostic for LintDiagnosticSource {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new(self.code))
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(self.severity.to_miette())
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        Some(Box::new(std::iter::once(LabeledSpan::new_with_span(
            Some("here".into()),
            SourceSpan::from(self.span),
        ))))
    }
}

impl LtxDiagnosticSource for LintDiagnosticSource {
    fn span(&self) -> LtxSpan {
        self.span
    }
}

/// Emits a lint finding into `sink`, resolving the span through `ctx`.
pub(crate) fn emit(
    sink: &mut LtxDiagnosticSink,
    ctx: &LintContext<'_, '_>,
    code: &'static str,
    severity: LtxSeverity,
    message: Cow<'static, str>,
    span: LtxSpan,
) {
    let source = LintDiagnosticSource::new(code, message, severity, span);
    sink.push(LtxDiagnostic::new(source, ctx.source_map.clone()));
}
