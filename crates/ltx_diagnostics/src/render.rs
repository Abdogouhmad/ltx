use crate::{LtxDiagnostic, LtxSeverity};
use miette::Diagnostic; // <--- ADD THIS LINE
use serde::Serialize;

/// A lightweight, serializable diagnostic for JSON output.
#[derive(Serialize)]
pub struct JsonDiagnostic {
    /// The severity of the diagnostic message.
    pub severity: LtxSeverity,
    /// The diagnostic code.
    pub code: String,
    /// The diagnostic message text.
    pub message: String,
}

impl From<&LtxDiagnostic> for JsonDiagnostic {
    fn from(d: &LtxDiagnostic) -> Self {
        Self {
            severity: d.severity(),
            // Now .code() works because Diagnostic trait is in scope
            code: d
                .code()
                .map_or_else(|| "ltx::unknown".to_string(), |c| c.to_string()),
            message: d.to_string(),
        }
    }
}
/// Renders a list of `LtxDiagnostic` messages as a JSON string.
pub fn render_json(diags: &[LtxDiagnostic]) -> String {
    let out: Vec<JsonDiagnostic> = diags.iter().map(JsonDiagnostic::from).collect();

    serde_json::to_string_pretty(&out).unwrap_or_else(|_| "[]".to_string())
}
