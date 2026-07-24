//! Project scaffolding — generates the directory layout and starter files.

use std::path::Path;

use ltx_utils::{create_dir, write_file};

use crate::build::Build;
use crate::engine::{CompilerEngine, Engine};
use crate::manifest::LtxManifest;
use crate::project::Project;

/// Options that control the generated project structure.
///
/// Pass this to [`scaffold`] to create a new project on disk.
#[derive(Debug, Clone)]
pub struct ScaffoldOptions {
    /// Project name (used as the root directory and in `config.toml`).
    pub name: String,
    /// LaTeX compiler to use.
    pub engine: CompilerEngine,
    /// Place source files under `src/` instead of the project root.
    pub src: bool,
    /// Create a `bib/` directory with a starter `references.bib`.
    pub bib: bool,
}

/// Errors that can occur during scaffolding.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ScaffoldError {
    /// An I/O error occurred while creating directories or files.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// The project directory already exists and is non-empty.
    #[error("project directory `{0}` already exists")]
    #[diagnostic(code(ltx::scaffold::already_exists))]
    AlreadyExists(String),
}

/// Creates a full LTX project under `base`.
///
/// Generated layout (flags adjust sub-directories):
///
/// ```text
/// projectname/
/// ├── src/main.tex + src/sections/   (--src)
/// ├── main.tex                       (default)
/// ├── bib/references.bib             (--bib)
/// ├── references.bib                 (default)
/// ├── config.toml
/// ├── build/
/// └── .gitignore
/// ```
///
/// # Errors
///
/// Returns [`ScaffoldError::AlreadyExists`] if the directory is non-empty,
/// or [`ScaffoldError::Io`] on filesystem failures.
pub fn scaffold(base: &Path, opts: &ScaffoldOptions) -> Result<(), ScaffoldError> {
    if base.exists() && base.read_dir()?.next().is_some() {
        return Err(ScaffoldError::AlreadyExists(base.display().to_string()));
    }

    create_dir(base)?;

    let main_path = if opts.src {
        let src = base.join("src");
        create_dir(&src.join("sections"))?;
        src.join("main.tex")
    } else {
        base.join("main.tex")
    };

    let bib_path = if opts.bib {
        let bib = base.join("bib");
        create_dir(&bib)?;
        bib.join("references.bib")
    } else {
        base.join("references.bib")
    };

    let main_rel = main_path
        .strip_prefix(base)
        .unwrap_or_else(|_| Path::new("main.tex"))
        .to_string_lossy()
        .into_owned();

    let mut project = Project::new(&opts.name);
    project.set_main(main_rel);

    let manifest = LtxManifest::new(project, Engine::new(opts.engine))
        .with_build(Build::new(&opts.name, "build"));

    write_file(&main_path, RENDER_MAIN_TEX)?;
    write_file(&bib_path, RENDER_REFERENCES_BIB)?;
    write_file(&base.join(".gitignore"), RENDER_GITIGNORE)?;
    manifest.write(base.join("config.toml"))?;

    Ok(())
}

/// Starter `main.tex` template.
const RENDER_MAIN_TEX: &str = r"\documentclass{article}

\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{amsmath,amssymb}
\usepackage{graphicx}
\usepackage{hyperref}
\usepackage{cleveref}

\title{Project}
\author{}
\date{\today}

\begin{document}

\maketitle

\section{Introduction}

% TODO: write content here

\end{document}
";

/// Starter `references.bib` template.
const RENDER_REFERENCES_BIB: &str = r"@article{example2024,
  author  = {Author, A.},
  title   = {An Example Entry},
  journal = {Journal of Examples},
  year    = {2024},
  volume  = {1},
  pages   = {1--10},
}
";

/// Default `.gitignore` content (ignores `build/`).
const RENDER_GITIGNORE: &str = "build/\n";
