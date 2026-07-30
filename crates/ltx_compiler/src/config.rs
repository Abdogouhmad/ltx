//! Configuration for the Ltx compiler.
use ltx_config::Build;
use ltx_config::engine::CompilerEngine;

/// Configuration for the Ltx compiler.
pub struct CompilerConfig {
    /// The compiler engine to use.
    pub engine: CompilerEngine,
    /// Additional arguments to pass to the compiler.
    pub engine_args: Vec<String>,
    /// The name of the output file.
    pub output_name: String,
}

impl CompilerConfig {
    /// Creates a new [`CompilerConfig`] from a [`Build`] configuration.
    pub fn from_build(build: &Build) -> Self {
        Self {
            engine: build.engine(),
            engine_args: build.engine_args().map_or(Vec::new(), |a| a.to_vec()),
            output_name: build.name().unwrap_or("output").to_string(),
        }
    }

    /// function that returns the name of the engine.
    ///
    /// Returns the engine name as a `&str` (e.g. `"pdflatex"`).
    #[inline]
    #[must_use]
    pub fn engine_name(&self) -> &str {
        match self.engine {
            CompilerEngine::PdfLaTeX => "pdflatex",
            CompilerEngine::XeLaTeX => "xelatex",
            CompilerEngine::LuaLaTeX => "lualatex",
            CompilerEngine::Tectonic => "tectonic",
        }
    }

    /// Function that returns the name of the output file.
    ///
    /// Returns the output name as a `&str` (e.g. `"output"`).
    #[inline]
    #[must_use]
    pub fn output_name(&self) -> &str {
        &self.output_name
    }

    // pub fn command(&self) -> (String, Vec<String>) {
    //     let name = &self.output_name;
    //     match self.engine {
    //         CompilerEngine::PdfLaTeX => {
    //             let mut args = vec!["-interaction=nonstopmode".into(), format!("{name}.tex")];
    //             args.extend(self.engine_args.clone());
    //             ("pdflatex".into(), args)
    //         }
    //         CompilerEngine::XeLaTeX => {
    //             let mut args = vec!["-interaction=nonstopmode".into(), format!("{name}.tex")];
    //             args.extend(self.engine_args.clone());
    //             ("xelatex".into(), args)
    //         }
    //         CompilerEngine::LuaLaTeX => {
    //             let mut args = vec!["-interaction=nonstopmode".into(), format!("{name}.tex")];
    //             args.extend(self.engine_args.clone());
    //             ("lualatex".into(), args)
    //         }
    //         CompilerEngine::Tectonic => {
    //             let mut args = vec![format!("{name}.tex")];
    //             args.extend(self.engine_args.clone());
    //             ("tectonic".into(), args)
    //         }
    //     }
    // }
}
