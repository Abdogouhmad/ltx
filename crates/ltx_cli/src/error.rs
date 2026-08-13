//! Structured error types for the `ltx` CLI.

/// Errors that can occur during CLI operations.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum CliError {
    /// An I/O error occurred (file not found, permission denied, etc.).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The input file could not be read as valid UTF-8.
    #[error("file is not valid UTF-8: {0}")]
    NotUtf8(#[from] std::string::FromUtf8Error),

    /// The lexer produced errors that prevented parsing.
    #[error("lexer errors prevented parsing")]
    LexerErrors,

    /// The parser produced diagnostics (errors or warnings).
    #[error("check finished with diagnostics")]
    DiagnosticsFound,

    /// A self-update check or install failed.
    #[cfg(feature = "self-update")]
    #[error(transparent)]
    Update(#[from] self_update::errors::Error),
}
