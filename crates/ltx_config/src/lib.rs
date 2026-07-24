//! Configuration and scaffolding for the `ltx` CLI.
//!
//! This crate provides the data model for `config.toml`
//! ([`LtxManifest`], [`Project`], [`Engine`]) and the
//! [`scaffold()`] function that generates a new project on disk.

pub mod build;
pub mod engine;
pub mod manifest;
pub mod project;
pub mod scaffold;

pub use build::Build;
pub use engine::{CompilerEngine, Engine};
pub use manifest::LtxManifest;
pub use project::Project;
pub use scaffold::{ScaffoldError, ScaffoldOptions, scaffold};
