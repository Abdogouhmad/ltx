//! Lint configuration table for `ltx.toml`.

use serde::{Deserialize, Serialize};

/// Configuration table for lint rules (`[lints]`).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LintTable {
    /// Rules configured as errors.
    #[serde(default)]
    pub deny: Vec<String>,
    /// Rules configured as warnings.
    #[serde(default)]
    pub warn: Vec<String>,
    /// Rules configured as ignored / allowed.
    #[serde(default)]
    pub allow: Vec<String>,
}

impl LintTable {
    /// Creates a new, empty lint table.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            deny: Vec::new(),
            warn: Vec::new(),
            allow: Vec::new(),
        }
    }
}
