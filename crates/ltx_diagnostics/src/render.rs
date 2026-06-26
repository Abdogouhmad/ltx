use crate::{LtxDiagnostic, LtxSeverity, LtxSourceMap};
use miette::Diagnostic;
use serde::Serialize;

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
    ///
    /// # Arguments
    ///
    /// * `source_map` - The source map used to resolve the diagnostic's span.
    ///
    /// # Returns
    ///
    /// A `JsonDiagnostic` representing this diagnostic.
    #[must_use]
    #[inline]
    pub fn to_json(&self, source_map: &std::sync::Arc<LtxSourceMap>) -> JsonDiagnostic {
        let span = self.span();
        let (line, col) = source_map
            .line_col(span.file_id, span.start)
            .map_or((None, None), |(l, c)| (Some(l), Some(c)));

        JsonDiagnostic {
            severity: self.severity(),
            code: self
                .code()
                .map_or_else(|| "ltx::unknown".to_string(), |c| c.to_string()),
            message: self.to_string(),
            line,
            column: col,
        }
    }
}

/// Renders diagnostics as JSON
///
/// # Arguments
///
/// * `diags` - The diagnostics to render.
/// * `source_map` - The source map used to resolve the diagnostic's span.
///
/// # Returns
///
/// A JSON string representing the diagnostics.
#[must_use]
#[inline]
pub fn render_json(diags: &[LtxDiagnostic], source_map: &std::sync::Arc<LtxSourceMap>) -> String {
    let out: Vec<JsonDiagnostic> = diags.iter().map(|d| d.to_json(source_map)).collect();
    serde_json::to_string_pretty(&out).unwrap_or_else(|_| "[]".to_string())
}
