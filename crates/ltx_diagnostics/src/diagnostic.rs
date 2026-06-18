use crate::errors::{LexerError, ParserError};
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{LtxSeverity, LtxSpan};

#[derive(Debug, Diagnostic, Error, Clone)]
/// `LtxDiagnostic` represents a diagnostic message from the Ltx compiler.
pub enum LtxDiagnostic {
    /// A diagnostic message from the lexer.
    #[error(transparent)]
    Lexer(#[from] LexerError),
    /// A diagnostic message from the parser.
    #[error(transparent)]
    Parser(#[from] ParserError),
}

impl LtxDiagnostic {
    /// Returns the severity of the diagnostic.
    ///
    /// # Returns
    ///
    /// The severity of the diagnostic.
    #[must_use]
    pub const fn severity(&self) -> LtxSeverity {
        match self {
            Self::Lexer(_) => LtxSeverity::Error,
            Self::Parser(_) => LtxSeverity::Error,
        }
    }

    /// Returns the diagnostic with the source code attached.
    ///
    /// # Arguments
    ///
    /// * `span` - The span of the diagnostic in the source code.
    /// * `source` - The source code that the diagnostic was found in.
    /// * `file_name` - The name of the file that the diagnostic was found in.
    ///
    /// # Returns
    ///
    /// The diagnostic with the source code attached.
    #[must_use]
    pub fn with_source(self, span: LtxSpan, source: String, file_name: String) -> Self {
        let source_span: SourceSpan = span.into();
        let named_src = miette::NamedSource::new(file_name, source);

        match self {
            Self::Lexer(lex_err) => Self::Lexer(lex_err.with_source(source_span, named_src)),
            Self::Parser(pars_err) => Self::Parser(pars_err.with_source(source_span, named_src)),
        }
    }
}
