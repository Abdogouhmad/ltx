//! Ltx compiler which compiles Ltx source code into PDF Via rust lib tectonic.
//! Or via `pdflatex` or `lualatex` using UNIX processes.

/// the build is the entry point of all compiler engines that ltx offers.
pub mod build;
/// returns the configuration for the compiler [`name`, `engine_name`, `engine_args`].
pub mod config;
/// Unified compiler errors: [`CompilerError`] and the code registry [`ALL_CODES`].
pub mod error;
/// tectonic engine
pub mod tectonic;
/// will watch for changes in the source code and recompile as needed.
pub mod watch;

pub use error::{ALL_CODES, CompilerError};
