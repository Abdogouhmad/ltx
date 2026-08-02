//! Configuration and scaffolding for the `ltx` CLI.
//!
//! This crate provides the data model for `ltx.toml`
//! ([`LtxManifest`], [`Project`]), the [`validate_manifest()`] function that
//! checks a manifest before it is used, and the [`scaffold()`] function that
//! generates a new project on disk.

pub mod build;
pub mod engine;
pub mod manifest;
pub mod project;
pub mod scaffold;
pub mod validate;

pub use build::{Build, CompileOptions};
pub use engine::{CompilerEngine, Engine};
pub use manifest::LtxManifest;
pub use project::Project;
pub use scaffold::{BibLayout, ScaffoldError, ScaffoldOptions, SrcLayout, scaffold};
pub use validate::{ManifestDiagnostic, validate_manifest};
