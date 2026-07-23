//! The root module for the `ltx` configuration manifest.

use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::engine::Engine;
use crate::project::Project;

/// Top-level `config.toml` structure.
#[derive(Debug, Serialize)]
pub struct LtxManifest {
    /// Project metadata.
    pub project: Project,
    /// Engine configuration.
    pub engine: Engine,
}

impl LtxManifest {
    /// Creates a new manifest from pre-built [`Project`] and [`Engine`].
    #[must_use]
    pub const fn new(project: Project, engine: Engine) -> Self {
        Self { project, engine }
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
