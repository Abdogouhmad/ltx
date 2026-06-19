//! `LtxDiagnostic` represents a unified diagnostic message from the Ltx compiler.
use crate::errors::{LexerError, ParserError};
use crate::{LtxSeverity, LtxSpan};
use miette::Diagnostic;
use thiserror::Error;

/// `LtxDiagnostic` represents a unified diagnostic message from the Ltx compiler.
#[derive(Debug, Error, Diagnostic, Clone)]
pub enum LtxDiagnostic {
    /// A diagnostic message originating from the lexer phase.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Lexer(#[from] LexerError),

    /// A diagnostic message originating from the parser phase.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parser(#[from] ParserError),
}

impl LtxDiagnostic {
    /// If you still need this for internal non-miette logic, keep it.
    /// Otherwise, miette reads severity directly from the inner errors now!
    #[must_use]
    pub const fn severity(&self) -> LtxSeverity {
        match self {
            Self::Lexer(_) | Self::Parser(_) => LtxSeverity::Error,
        }
    }

    #[must_use]
    /// Attaches source code context to the diagnostic for rich terminal rendering.
    pub fn with_source(self, span: LtxSpan, source: String, file_name: String) -> Self {
        let source_span = span.into();
        let named_src = miette::NamedSource::new(file_name, source);

        match self {
            Self::Lexer(e) => Self::Lexer(e.with_source(source_span, named_src)),
            Self::Parser(e) => Self::Parser(e.with_source(source_span, named_src)),
        }
    }
}
