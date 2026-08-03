//! The root module for the `ltx` configuration manifest.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::ConfigError;
use crate::{Build, Project, validate_manifest};

/// Top-level `ltx.toml` structure.
///
/// Only two sections exist: `[project]` for metadata and `[build]` for
/// compilation settings (engine, output name, arguments).
///
/// # Examples
///
/// ```rust
/// use ltx_config::{Build, CompilerEngine, LtxManifest, Project};
///
/// let manifest = LtxManifest::new(Project::new("my-paper"))
///     .with_build(Build::new("my-paper", CompilerEngine::PdfLaTeX));
/// let toml = manifest.to_toml().unwrap();
/// assert!(toml.contains("my-paper"));
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LtxManifest {
    /// Project metadata.
    pub project: Project,
    /// Optional build configuration (engine, output name, args).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

impl LtxManifest {
    /// Creates a new manifest with the given project.
    #[must_use]
    pub const fn new(project: Project) -> Self {
        Self {
            project,
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

    /// Reads, parses, and validates a manifest from a TOML file.
    ///
    /// The manifest is validated the same way as [`validate_manifest`]:
    /// `[project].main` must be set and point to an existing file, and a
    /// `[build]` section with a `name` must be present. Unknown keys and
    /// malformed TOML are rejected.
    ///
    /// Relative paths in the manifest are resolved against the directory
    /// containing the file.
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigError`] describing the first problem found (read
    /// failure, parse error, or a validation rule violation).
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let content =
            fs::read_to_string(path).map_err(|err| ConfigError::read_failed(&err, path))?;
        let project_root = path.parent().unwrap_or_else(|| Path::new("."));
        validate_manifest(&content, project_root)
    }
}
