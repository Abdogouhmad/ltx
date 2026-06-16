use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{LtxSeverity, LtxSpan};

#[derive(Debug, Diagnostic, Error, Clone)]
/// `LtxDiagnostic` represents a diagnostic message from the Ltx compiler.
pub enum LtxDiagnostic {
    // ------------- Parse errors -----------------
    /// Represents an unexpected token that was found during parsing.
    #[error("unexpected token `{found}`")]
    #[diagnostic(
        code(ltx::parse::E001),
        help("check LaTeX syntax near this location "),
        severity(Error)
    )]
    UnexpectedToken {
        /// The token that was found.
        found: String,
        #[label("unexpected token")]
        /// The span of the token that was found.
        span: SourceSpan,
        #[source_code]
        /// The source code that the token was found in.
        src: miette::NamedSource<String>,
    },

    /// Represents an unclosed environment that was found during parsing.
    #[error("unclosed environment `{name}`")]
    #[diagnostic(
        code(ltx::parse::E002),
        help("add \\end{{{name}}} before the next \\begin or end of file"),
        severity(Error)
    )]
    UnclosedEnvironment {
        /// The name of the environment that was not closed.
        name: String,
        #[label("opened here")]
        /// The span of the token that opened the environment.
        open_span: SourceSpan,
        #[source_code]
        /// The source code that the environment was found in.
        src: miette::NamedSource<String>,
    },

    #[error("missing \\documentclass")]
    #[diagnostic(
        code(ltx::parse::E003),
        help("every LaTeX document must start with \\documentclass and \\end of document"),
        url("https://tex.stackexchange.com/questions/10889/what-is-the-documentclass-command"),
        severity(Error)
    )]
    /// Represents a missing `\documentclass` command that was found during parsing.
    MissingDocumentClass {
        #[label("document starts here")]
        /// The span of the token that opened the environment.
        span: SourceSpan,
        /// The source code that the environment was found in.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// Represents an unknown command that was found during parsing.
    #[error("unknown command `\\{name}`")]
    #[diagnostic(
        code(ltx::parse::E004),
        severity(Error),
        help("check if the command is defined or spelled correctly"),
    )]
    UnknownCommand {
        /// The name of the unknown command.
        name: String,
        #[label("unknown command")]
        /// The span of the token that opened the environment.
        span: SourceSpan,
        /// The source code that the environment was found in.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    // ── Lint warnings ───────────────────────────────────────────────────────
    /// Represents a deprecated math delimiter that was found during xlinting.
    #[error("deprecated math delimiter `$$`")]
    #[diagnostic(
        code(ltx::lint::W001),
        help("replace $$ ... $$ with \\[ ... \\]"),
        severity(Warning)
    )]
    DeprecatedMathDelimiter {
        #[label("use \\[ ... \\] instead")]
        /// The span of the token that opened the environment.
        span: SourceSpan,
        #[source_code]
        /// The source code that the environment was found in.
        src: miette::NamedSource<String>,
    },

    /// Represents trailing whitespace that was found during linting.
    #[error("trailing whitespace")]
    #[diagnostic(
        code(ltx::lint::W002),
        severity(Warning),
        help("remove trailing whitespace"),
    )]
    TrailingWhitespace {
        #[label("trailing whitespace here")]
        /// The span of the trailing whitespace.
        span: SourceSpan,
        #[source_code]
        /// The source code that the trailing whitespace was found in.
        src: miette::NamedSource<String>,
    },

    /// Represents an unreferenced label that was found during linting.
    #[error("unreferenced label `{name}`")]
    #[diagnostic(
        code(ltx::lint::W003),
        help("use \\ref{{{name}}} or remove the label"),
        severity(Warning)
    )]
    UnreferencedLabel {
        /// The name of the label that was not referenced.
        name: String,
        #[label("defined here, never referenced")]
        /// The span of the label that was not referenced.
        span: SourceSpan,
        #[source_code]
        /// The source code that the label was found in.
        src: miette::NamedSource<String>,
    },
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
            Self::UnexpectedToken { .. }
            | Self::UnclosedEnvironment { .. }
            | Self::MissingDocumentClass { .. }
            | Self::UnknownCommand { .. } => LtxSeverity::Error,

            Self::DeprecatedMathDelimiter { .. }
            | Self::TrailingWhitespace { .. }
            | Self::UnreferencedLabel { .. } => LtxSeverity::Warning,
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
            Self::UnexpectedToken { found, .. } => Self::UnexpectedToken {
                found,
                span: source_span,
                src: named_src,
            },
            Self::UnclosedEnvironment { name, .. } => Self::UnclosedEnvironment {
                name,
                open_span: source_span,
                src: named_src,
            },
            Self::MissingDocumentClass { .. } => Self::MissingDocumentClass {
                span: source_span,
                src: named_src,
            },
            Self::UnknownCommand { name, .. } => Self::UnknownCommand {
                name,
                span: source_span,
                src: named_src,
            },
            Self::DeprecatedMathDelimiter { .. } => Self::DeprecatedMathDelimiter {
                span: source_span,
                src: named_src,
            },
            Self::TrailingWhitespace { .. } => Self::TrailingWhitespace {
                span: source_span,
                src: named_src,
            },
            Self::UnreferencedLabel { name, .. } => Self::UnreferencedLabel {
                name,
                span: source_span,
                src: named_src,
            },
        }
    }
}
