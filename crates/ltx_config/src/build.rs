//! Build output configuration for the `[build]` section of `config.toml`.

use serde::Serialize;

/// Controls where and how compiled output is written.
///
/// Both fields are optional — when omitted from `config.toml`, the toolchain
/// falls back to its built-in defaults (`output = "<project>.pdf"`,
/// `outdir = "build"`).
///
/// # Examples
///
/// ```rust
/// use ltx_config::Build;
///
/// let build = Build::new("paper.pdf", "build");
/// assert_eq!(build.name(), Some("paper.pdf"));
/// assert_eq!(build.outdir(), Some("build"));
/// ```
#[derive(Debug, Default, Serialize)]
pub struct Build {
    /// Name of the PDF output file (without extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// Path to the output directory, relative to the project root.
    #[serde(skip_serializing_if = "Option::is_none")]
    outdir: Option<String>,
}

impl Build {
    /// Creates a new build configuration.
    #[must_use]
    pub fn new(name: impl Into<String>, outdir: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            outdir: Some(outdir.into()),
        }
    }

    /// Returns the output file name, if set.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the output directory, if set.
    #[must_use]
    pub fn outdir(&self) -> Option<&str> {
        self.outdir.as_deref()
    }
}
