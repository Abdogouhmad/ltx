//! Validation of `ltx.toml` manifests with source-spanning diagnostics.

use std::path::Path;

use crate::LtxManifest;
use crate::error::{ConfigError, key_span, table_span, whole_span};

/// Validates an `ltx.toml` manifest.
///
/// Parses `source` and checks the structural requirements the compiler
/// depends on:
///
/// - the `[project].main` field is set and points to an existing file,
/// - a `[build]` section is present with a non-empty `name`.
///
/// Malformed TOML and unknown keys (e.g. a typo like `[build.option]`)
/// are reported as parse errors before any semantic checks run.
///
/// # Arguments
///
/// * `source` - The raw `ltx.toml` content, used to render error spans.
/// * `project_root` - The directory relative paths in the manifest resolve against.
///
/// # Errors
///
/// Returns a [`ConfigError`] describing the first problem found.
///
/// # Examples
///
/// ```rust
/// use ltx_config::validate_manifest;
///
/// let toml = r#"
/// [project]
/// name = "demo"
/// main = "main.tex"
///
/// [build]
/// name = "demo"
/// engine = "tectonic"
/// "#;
///
/// let err = validate_manifest(toml, std::path::Path::new("/definitely-not-a-dir"))
///     .expect_err("main.tex should not exist");
/// assert!(err.to_string().contains("main file not found"));
/// ```
pub fn validate_manifest(source: &str, project_root: &Path) -> Result<LtxManifest, ConfigError> {
    let manifest: LtxManifest =
        toml::from_str(source).map_err(|err| ConfigError::invalid_toml(&err, source))?;

    let main = manifest.project.get_main_project().ok_or_else(|| {
        ConfigError::missing_main(
            source,
            table_span(source, "[project]").unwrap_or_else(|| whole_span(source)),
        )
    })?;

    let Some(build) = manifest.build.as_ref() else {
        return Err(ConfigError::missing_build(
            source,
            table_span(source, "[project]").unwrap_or_else(|| whole_span(source)),
        ));
    };

    if build.name().is_none() {
        return Err(ConfigError::missing_build_name(
            source,
            table_span(source, "[build]").unwrap_or_else(|| whole_span(source)),
        ));
    }

    let main_path = project_root.join(main);
    if !main_path.is_file() {
        return Err(ConfigError::main_file_not_found(
            main_path,
            source,
            key_span(source, "[project]", "main").unwrap_or_else(|| whole_span(source)),
        ));
    }

    Ok(manifest)
}
