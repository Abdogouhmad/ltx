//! The project table schema for the `ltx` configuration manifest.

use serde::Serialize;

/// Project metadata written to `config.toml`.
#[derive(Serialize, Debug)]
pub struct Project {
    /// Project name.
    pub name: String,

    /// Optional semver version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,

    /// Optional list of authors.
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<Vec<String>>,

    /// Path to the main `.tex` file relative to the project root.
    #[serde(skip_serializing_if = "Option::is_none")]
    main: Option<String>,
}

impl Project {
    /// Creates a new project entry with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
            author: None,
            main: None,
        }
    }

    /// Returns the project name.
    #[must_use]
    pub fn get_name_project(&self) -> &str {
        &self.name
    }

    /// Returns the project version, if set.
    #[must_use]
    pub fn get_version_project(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the project authors, if set.
    #[must_use]
    pub fn get_author_project(&self) -> Option<&[String]> {
        self.author.as_deref()
    }

    /// Returns the main `.tex` file path, if set.
    #[must_use]
    pub fn get_main_project(&self) -> Option<&str> {
        self.main.as_deref()
    }

    /// Sets the main `.tex` file path.
    pub fn set_main(&mut self, path: impl Into<String>) {
        self.main = Some(path.into());
    }
}
