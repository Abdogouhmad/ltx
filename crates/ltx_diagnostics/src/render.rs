use crate::{LtxDiagnostic, LtxSeverity};
use miette::Diagnostic;
use serde::Serialize;
use std::{fmt, io};

/// A lightweight, serializable diagnostic for JSON output.
#[derive(Serialize)]
pub struct JsonDiagnostic {
    /// The severity of the diagnostic.
    pub severity: LtxSeverity,
    /// The code of the diagnostic.
    pub code: String,
    /// The message of the diagnostic.
    pub message: String,
    /// The line number of the diagnostic.
    pub line: Option<usize>,
    /// The column number of the diagnostic.
    pub column: Option<usize>,
}

impl LtxDiagnostic {
    /// Converts this diagnostic to a `JsonDiagnostic` for JSON output.
    #[must_use]
    #[inline]
    pub fn to_json(&self) -> JsonDiagnostic {
        let span = self.span();
        let (line, column) = self
            .source_map
            .line_col(span.file_id, span.start)
            .map_or((None, None), |(l, c)| (Some(l), Some(c)));

        JsonDiagnostic {
            severity: self.severity(),
            code: self
                .code()
                .map_or_else(|| "ltx::unknown".to_string(), |c| c.to_string()),
            message: self.to_string(),
            line,
            column,
        }
    }

    /// Writes this diagnostic's pretty (miette) rendering into `writer`.
    /// No intermediate `String` is allocated — the caller supplies the buffer/destination.
    #[inline]
    pub fn render_pretty_into(&self, writer: &mut impl fmt::Write) -> fmt::Result {
        let report = miette::Report::new(self.clone());
        miette::GraphicalReportHandler::new()
            .render_report(writer, report.as_ref())
            .map_err(|_| fmt::Error)
    }
}

/// Writes all diagnostics' pretty renderings into `writer`, one after another.
#[inline]
pub fn render_pretty_into(diags: &[LtxDiagnostic], writer: &mut impl fmt::Write) -> fmt::Result {
    let handler = miette::GraphicalReportHandler::new();
    for diag in diags {
        let report = miette::Report::new(diag.clone());
        handler
            .render_report(writer, report.as_ref())
            .map_err(|_| fmt::Error)?;
        writer.write_char('\n')?;
    }
    Ok(())
}

/// Serializes diagnostics as JSON directly into an `io::Write` sink
/// (a `File`, `Stdout`, `TcpStream`, ...) — no intermediate `String`/`Vec<u8>`.
#[inline]
pub fn render_json_into(diags: &[LtxDiagnostic], writer: impl io::Write) -> serde_json::Result<()> {
    let out: Vec<JsonDiagnostic> = diags.iter().map(LtxDiagnostic::to_json).collect();
    serde_json::to_writer_pretty(writer, &out)
}

/// Renders a slice of diagnostics into a [`String`] using miette's graphical handler.
///
/// This is a convenience wrapper around [`render_pretty_into`] for callers
/// that just need a `String` (tests, CLI error output, etc.).
#[must_use]
pub fn render_pretty(diags: &[LtxDiagnostic]) -> String {
    let mut buf = String::new();
    let _ = render_pretty_into(diags, &mut buf);
    buf
}
