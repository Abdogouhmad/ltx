//! `new` command — creates a new ltx project.

use clap::Args;

/// Arguments for the `ltx new` subcommand.
#[derive(Debug, Clone, Args)]
pub struct NewArgs {
    /// Name of the new project.
    pub name: String,

    /// LaTeX engine to use (defaults to `pdflatex`).
    #[arg(short, long, default_value = "pdflatex")]
    pub engine: Option<String>,

    /// Include bibliography support (creates `bib/` directory).
    #[arg(long)]
    pub bib: bool,

    /// Use `src/` directory layout for source files.
    #[arg(long)]
    pub src: bool,
}

impl NewArgs {
    /// Returns the project name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the LaTeX engine, or `None` if not specified.
    #[must_use]
    pub fn engine(&self) -> Option<&str> {
        self.engine.as_deref()
    }

    /// Returns whether bibliography support is enabled.
    #[must_use]
    pub fn bib(&self) -> bool {
        self.bib
    }

    /// Returns whether the `src/` directory layout should be used.
    #[must_use]
    pub fn src(&self) -> bool {
        self.src
    }
}
