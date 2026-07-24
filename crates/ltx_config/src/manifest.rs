//! The root module for the `ltx` configuration manifest.

use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::{Build, Engine, Project};

/// Top-level `config.toml` structure.
///
/// # Examples
///
/// ```rust
/// use ltx_config::{CompilerEngine, Engine, LtxManifest, Project};
///
/// let manifest = LtxManifest::new(
///     Project::new("my-paper"),
///     Engine::new(CompilerEngine::PdfLaTeX),
/// );
/// let toml = manifest.to_toml().unwrap();
/// assert!(toml.contains("my-paper"));
/// ```
#[derive(Debug, Serialize)]
pub struct LtxManifest {
    /// Project metadata.
    pub project: Project,
    /// Engine configuration.
    pub engine: Engine,
    /// Optional build output configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

impl LtxManifest {
    /// Creates a new manifest from pre-built components.
    #[must_use]
    pub const fn new(project: Project, engine: Engine) -> Self {
        Self {
            project,
            engine,
            build: None,
        }
    }

    /// Attaches a [`Build`] configuration to the manifest.
    #[must_use]
    pub fn with_build(mut self, build: Build) -> Self {
        self.build = Some(build);
        self
    }

    /// Serializes the manifest into a pretty TOML string.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Writes the manifest to disk.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization or file writing fails.
    pub fn write(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let toml = self
            .to_toml()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, toml)
    }
}
