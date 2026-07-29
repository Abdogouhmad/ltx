//! Commands is a wrapper around the CLI commands.
pub mod check;
pub mod clean;
pub mod code;
pub mod new;

// re-exports
pub use check::CheckArgs;
pub use clean::clean_build;
pub use code::run_code;
pub use new::NewArgs;
