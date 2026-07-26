//! The root module for the `ltx` configuration manifest.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

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
#[derive(Debug, Serialize, Deserialize)]
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

    /// Reads and deserializes a manifest from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ManifestError> {
        let content = fs::read_to_string(path.as_ref())?;
        let manifest = toml::from_str(&content)?;
        Ok(manifest)
    }
}

/// Errors that can occur when loading a manifest.
#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    /// An I/O error occurred while reading the file.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// The TOML content could not be deserialized.
    #[error(transparent)]
    Parse(#[from] toml::de::Error),
}
