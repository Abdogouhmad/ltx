//! Configuration and scaffolding for the `ltx` CLI.
//!
//! This crate provides the data model for `ltx.toml`
//! ([`LtxManifest`], [`Project`]), the [`validate_manifest()`] function that
//! checks a manifest before it is used, and the [`scaffold()`] function that
//! generates a new project on disk.
//!
//! All errors this crate can produce are owned by the [`error::ConfigError`]
//! enum, whose codes live under the `LTX::CONFIG::E0xx` namespace.

pub mod build;
pub mod engine;
pub mod error;
pub mod manifest;
pub mod project;
pub mod scaffold;
pub mod validate;

pub use build::{Build, CompileOptions};
pub use engine::{CompilerEngine, Engine};
pub use error::{ALL_CODES, ConfigError};
pub use manifest::LtxManifest;
pub use project::Project;
pub use scaffold::{BibLayout, ScaffoldOptions, SrcLayout, scaffold};
pub use validate::validate_manifest;
