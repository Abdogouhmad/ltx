//! `new` command — creates a new ltx project

use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct NewArgs {
    /// Name of the new project
    pub name: String,

    /// LaTeX engine to use
    #[arg(short, long, default_value = "PdfLaTeX")]
    pub engine: Option<String>,
}

/// Arguments for the `new` command
impl NewArgs {
    /// Returns the name of the new project
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns `None` if no engine was specified.
    pub fn engine(&self) -> Option<&str> {
        self.engine.as_deref()
    }
}
