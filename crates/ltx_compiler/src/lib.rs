//! Ltx compiler which compiles Ltx source code into PDF Via rust lib tectonic.
//! Or via pdflatex or LuaTeX using UNIX processes.

/// will handles the compilation of Ltx source code into HTML or PDF.
pub mod build;
/// returns the configuration for the compiler [`name`, `engine_name`, `engine_args`].
pub mod config;
/// will watch for changes in the source code and recompile as needed.
pub mod watch;
