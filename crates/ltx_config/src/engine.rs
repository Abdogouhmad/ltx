//! LaTeX engine configuration.

use serde::Serialize;

/// Represents a LaTeX engine and its command-line arguments.
///
/// # Examples
///
/// ```rust
/// use ltx_config::Engine;
///
/// let engine = Engine::new("pdflatex");
/// assert_eq!(engine.name(), "pdflatex");
/// assert!(engine.args().is_none());
/// ```
#[derive(Serialize, Debug)]
pub struct Engine {
    /// Engine name (e.g. `"pdflatex"`, `"xelatex"`, `"lualatex"`).
    name: String,

    /// Optional extra command-line arguments passed to the engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<Vec<String>>,
}

impl Engine {
    /// Creates a new engine with the given name and no extra arguments.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            args: None,
        }
    }

    /// Returns the engine name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the extra arguments, if any have been set.
    #[must_use]
    pub fn args(&self) -> Option<&[String]> {
        self.args.as_deref()
    }

    /// Sets the extra command-line arguments.
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = Some(args);
    }
}
