use crate::ctx::{AppContext, CliCommand};
use clap::Args;
use ltx_config::{BibLayout, CompilerEngine, ScaffoldOptions, SrcLayout, scaffold};
use std::path::PathBuf;

/// Arguments for `ltx new`.
///
/// Creates a new LaTeX project with the given name, engine, and layout.
/// The project is created in a subdirectory named after the project.
#[derive(Debug, Clone, Args)]
pub struct NewArgs {
    /// Name of the new project (creates a directory with this name).
    pub name: String,

    /// LaTeX engine to use for compilation.
    ///
    /// Supported values: `pdflatex`, `xelatex`, `lualatex`, `tectonic`.
    #[arg(short, long, default_value_t = CompilerEngine::default())]
    pub engine: CompilerEngine,

    /// Include bibliography support.
    ///
    /// Creates a `bib/` directory and a starter `references.bib` file.
    #[arg(long)]
    pub bib: bool,

    /// Use `src/` layout for source files.
    ///
    /// Places `main.tex` inside a `src/` directory instead of the
    /// project root.
    #[arg(long)]
    pub src: bool,
}

impl CliCommand for NewArgs {
    fn execute(&self, _ctx: &AppContext) -> miette::Result<()> {
        let project_dir = PathBuf::from(&self.name);
        let opts = ScaffoldOptions {
            name: self.name.clone(),
            engine: self.engine,
            src: if self.src {
                SrcLayout::WithSrcDir
            } else {
                SrcLayout::Flat
            },
            bib: if self.bib {
                BibLayout::WithBibDir
            } else {
                BibLayout::Flat
            },
        };

        scaffold(&project_dir, &opts)?;

        eprintln!("Created project `{}`", self.name);
        Ok(())
    }
}
