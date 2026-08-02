//! Validation of `ltx.toml` manifests with source-spanning diagnostics.

use std::fmt::Display;
use std::path::Path;

use miette::{Diagnostic, LabeledSpan, Severity, SourceCode, SourceSpan};
use thiserror::Error;

use crate::LtxManifest;

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
/// Returns a [`ManifestDiagnostic`] describing the first problem found.
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
pub fn validate_manifest(
    source: &str,
    project_root: &Path,
) -> Result<LtxManifest, ManifestDiagnostic> {
    let manifest: LtxManifest =
        toml::from_str(source).map_err(|err| ManifestDiagnostic::parse(&err, source))?;

    let main = manifest.project.get_main_project().ok_or_else(|| {
        ManifestDiagnostic::new(
            "missing `main` in the `[project]` section",
            "add `main = \"src/main.tex\"` (or the path to your main file)",
            "the `main` key is required here",
            table_span(source, "[project]").unwrap_or_else(|| whole_span(source)),
            source,
        )
    })?;

    let Some(build) = manifest.build.as_ref() else {
        return Err(ManifestDiagnostic::new(
            "missing `[build]` section",
            "add a `[build]` section with `name = \"...\"` and `engine = \"tectonic\"`",
            "the `[build]` section is required here",
            table_span(source, "[project]").unwrap_or_else(|| whole_span(source)),
            source,
        ));
    };

    if build.name().is_none() {
        return Err(ManifestDiagnostic::new(
            "missing `name` in the `[build]` section",
            "add `name = \"output\"` (without the `.pdf` extension)",
            "the `name` key is required here",
            table_span(source, "[build]").unwrap_or_else(|| whole_span(source)),
            source,
        ));
    }

    let main_path = project_root.join(main);
    if !main_path.is_file() {
        return Err(ManifestDiagnostic::new(
            format!("main file not found: `{}`", main_path.display()),
            format!(
                "create `{}` or fix the `main` value in the `[project]` section",
                main_path.display()
            ),
            "this file does not exist",
            key_span(source, "[project]", "main").unwrap_or_else(|| whole_span(source)),
            source,
        ));
    }

    Ok(manifest)
}

/// A rich error describing why an `ltx.toml` manifest is invalid.
///
/// Rendered by miette with a source snippet and a label pointing at the
/// offending table or key, plus a `help` hint describing the fix. Read
/// failures have no source snippet.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct ManifestDiagnostic {
    message: String,
    help: String,
    label: String,
    source_text: Option<String>,
    span: Option<SourceSpan>,
}

impl ManifestDiagnostic {
    /// Builds a diagnostic for a semantic validation failure.
    fn new(
        message: impl Into<String>,
        help: impl Into<String>,
        label: impl Into<String>,
        span: SourceSpan,
        source_text: &str,
    ) -> Self {
        Self {
            message: message.into(),
            help: help.into(),
            label: label.into(),
            source_text: Some(source_text.to_string()),
            span: Some(span),
        }
    }

    /// Builds a diagnostic from a TOML parse or unknown-field error.
    fn parse(err: &toml::de::Error, source_text: &str) -> Self {
        let detail = err.to_string();
        let reason = detail
            .lines()
            .last()
            .map_or_else(|| detail.as_str(), str::trim);
        let span = err
            .span()
            .map_or_else(|| whole_span(source_text), SourceSpan::from);
        Self {
            message: format!("invalid `ltx.toml`: {reason}"),
            help: detail,
            label: "this is not valid `ltx.toml`".to_string(),
            source_text: Some(source_text.to_string()),
            span: Some(span),
        }
    }

    /// Builds a diagnostic for a manifest file that could not be read.
    pub(crate) fn io(err: &std::io::Error, path: &Path) -> Self {
        Self {
            message: format!("failed to read `{}`: {err}", path.display()),
            help: "make sure the file exists and is readable".to_string(),
            label: String::new(),
            source_text: None,
            span: None,
        }
    }
}

impl Diagnostic for ManifestDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new("ltx::config::invalid"))
    }

    fn severity(&self) -> Option<Severity> {
        Some(Severity::Error)
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(&self.help))
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.source_text
            .as_ref()
            .map(|text| text as &dyn SourceCode)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        self.span.map(|span| {
            Box::new(std::iter::once(LabeledSpan::new_with_span(
                Some(self.label.clone()),
                span,
            ))) as Box<dyn Iterator<Item = LabeledSpan> + '_>
        })
    }
}

/// Returns the byte span of the line whose trimmed content equals `needle`.
fn line_span(source: &str, needle: &str) -> Option<SourceSpan> {
    let mut offset = 0usize;
    for line in source.split_inclusive('\n') {
        if line.trim() == needle {
            return Some(SourceSpan::new(offset.into(), line.trim_end().len()));
        }
        offset += line.len();
    }
    None
}

/// Returns the byte span of the `[table]` header line.
fn table_span(source: &str, table: &str) -> Option<SourceSpan> {
    line_span(source, table)
}

/// Returns the byte span of the `key = ...` line inside the given `[table]`.
fn key_span(source: &str, table: &str, key: &str) -> Option<SourceSpan> {
    let mut offset = 0usize;
    let mut in_table = false;
    for line in source.split_inclusive('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_table = trimmed == table;
        } else if in_table
            && trimmed
                .strip_prefix(key)
                .is_some_and(|rest| rest.trim_start().starts_with('='))
        {
            return Some(SourceSpan::new(offset.into(), line.trim_end().len()));
        }
        offset += line.len();
    }
    None
}

/// Span covering the entire source, used as a fallback when a table is absent.
fn whole_span(source: &str) -> SourceSpan {
    SourceSpan::new(0.into(), source.len())
}
