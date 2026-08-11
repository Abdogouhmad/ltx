//! Unified front-end: lex → parse → lint → diagnostics.
//!
//! This module is the single entry point the CLI uses for `ltx check`. It
//! runs the whole pipeline over a `.tex` file and reports every diagnostic —
//! lexer errors, parser errors, and lint findings — under a unified
//! `LTX::LINTER::E*` / `LTX::LINTER::W*` code namespace.

use std::fmt;
use std::path::Path;
use std::sync::Arc;

use ltx_config::LintTable;
use ltx_diagnostics::{
    LtxDiagnostic, LtxDiagnosticSink, LtxDiagnosticSource, LtxSeverity, LtxSourceMap, LtxSpan,
};
use ltx_lexer::{LtxLexer, TokenStream};
use ltx_parser::{LtxParser, parse_document};
use miette::{Diagnostic, LabeledSpan};

use crate::{LintContext, LintRegistry};

/// Maps every lexer/parser diagnostic code to its unified `LTX::LINTER::`
/// code. The lexer keeps codes `E001`–`E011`, the parser continues from
/// `E012`–`E017`, so the numbering stays stable and readable.
pub const CODE_REMAP: &[(&str, &str)] = &[
    // Lexer codes.
    ("LTX::LEXER::E001", "LTX::LINTER::E001"),
    ("LTX::LEXER::E002", "LTX::LINTER::E002"),
    ("LTX::LEXER::E003", "LTX::LINTER::E003"),
    ("LTX::LEXER::E004", "LTX::LINTER::E004"),
    ("LTX::LEXER::E005", "LTX::LINTER::E005"),
    ("LTX::LEXER::E006", "LTX::LINTER::E006"),
    ("LTX::LEXER::E007", "LTX::LINTER::E007"),
    ("LTX::LEXER::E008", "LTX::LINTER::E008"),
    ("LTX::LEXER::E009", "LTX::LINTER::E009"),
    ("LTX::LEXER::E010", "LTX::LINTER::E010"),
    ("LTX::LEXER::E011", "LTX::LINTER::E011"),
    // Parser codes.
    ("LTX::PARSER::E001", "LTX::LINTER::E012"),
    ("LTX::PARSER::E002", "LTX::LINTER::E013"),
    ("LTX::PARSER::E003", "LTX::LINTER::E014"),
    ("LTX::PARSER::E004", "LTX::LINTER::E015"),
    ("LTX::PARSER::E005", "LTX::LINTER::E016"),
    ("LTX::PARSER::E006", "LTX::LINTER::E017"),
];

/// Resolves a lexer/parser diagnostic code to its unified `LTX::LINTER::` code.
///
/// Unknown codes fall back to `LTX::LINTER::E000` so rendering never breaks.
#[must_use]
pub fn unified_code(code: &str) -> &'static str {
    for (source, unified) in CODE_REMAP {
        if *source == code {
            return unified;
        }
    }
    "LTX::LINTER::E000"
}

/// Wraps a lexer/parser diagnostic, exposing a unified `LTX::LINTER::` code
/// while preserving its message, severity, help text, and span.
#[derive(Clone)]
struct RecodedDiagnostic {
    inner: Arc<dyn LtxDiagnosticSource>,
    code: &'static str,
}

impl fmt::Display for RecodedDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl fmt::Debug for RecodedDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RecodedDiagnostic({})", self.code)
    }
}

impl std::error::Error for RecodedDiagnostic {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

impl Diagnostic for RecodedDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new(self.code))
    }

    fn severity(&self) -> Option<miette::Severity> {
        self.inner.severity()
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.inner.help()
    }

    fn url<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.inner.url()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        self.inner.labels()
    }
}

impl LtxDiagnosticSource for RecodedDiagnostic {
    fn span(&self) -> LtxSpan {
        self.inner.span()
    }
}

/// The outcome of running the full pipeline over one `.tex` file.
#[derive(Debug)]
pub struct LintResult {
    sink: LtxDiagnosticSink,
}

impl LintResult {
    /// Returns `true` when no diagnostics were produced.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sink.is_empty()
    }

    /// Returns `true` when at least one error was produced.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    /// Number of diagnostics with error severity.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.sink.get_by_severity(LtxSeverity::Error).count()
    }

    /// Number of diagnostics with warning severity.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.sink.len() - self.error_count()
    }

    /// All diagnostics produced by the pipeline.
    #[must_use]
    pub fn diagnostics(&self) -> &[LtxDiagnostic] {
        self.sink.all()
    }

    /// Renders all diagnostics with miette's pretty printer.
    ///
    /// # Errors
    ///
    /// Returns a formatting error if rendering fails.
    pub fn render_pretty(&self) -> Result<String, fmt::Error> {
        self.sink.render_pretty()
    }
}

/// Runs the full lex → parse → lint pipeline on the `.tex` file at `path`.
///
/// All diagnostics are emitted under unified `LTX::LINTER::` codes. When
/// `lint_table` is given (from `[lints]` in `ltx.toml`), lint rules are
/// filtered and re-leveled accordingly. When `run_lints` is `false`, only
/// lexing and parsing run.
///
/// # Errors
///
/// Returns an error if the file cannot be read or the lint table references
/// an unknown or contradictory rule.
pub fn lint_file(
    path: &Path,
    lint_table: Option<&LintTable>,
    run_lints: bool,
) -> miette::Result<LintResult> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| miette::miette!("Error reading `{}`: {e}", path.display()))?;
    run_pipeline(&source, path, lint_table, run_lints)
}

/// Runs the pipeline over an in-memory source (used by the CLI and tests).
pub(crate) fn run_pipeline(
    source: &str,
    name: &Path,
    lint_table: Option<&LintTable>,
    run_lints: bool,
) -> miette::Result<LintResult> {
    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline(name, source);
    let source_map = Arc::new(source_map);

    let stream = TokenStream::new(LtxLexer::new(source, file_id, source_map.as_ref().clone()));
    let mut parser = LtxParser::new(stream);
    let document = parse_document(&mut parser);

    let mut sink = LtxDiagnosticSink::new();
    // A fragment is a partial file meant to be `\input`/`\include`d into a
    // root document: it declares neither `\documentclass` nor its own
    // `\begin{document}`, so a missing document environment is expected
    // rather than an error. Files that do declare `\documentclass` are full
    // documents, and forgetting `\begin{document}` there stays an error.
    #[allow(clippy::literal_string_with_formatting_args)]
    let is_fragment = !source.contains("\\documentclass") && !source.contains("\\begin{document}");
    for diag in parser.error_handler_mut().take_diagnostics() {
        let source_code = diag
            .error
            .code()
            .map(|code| code.to_string())
            .unwrap_or_default();
        let code = unified_code(&source_code);
        if is_fragment && code == "LTX::LINTER::E017" {
            continue;
        }
        let error = RecodedDiagnostic {
            inner: diag.error.clone(),
            code,
        };
        sink.push(LtxDiagnostic::new(error, source_map.clone()));
    }

    if run_lints {
        let mut registry = LintRegistry::with_defaults(source);
        if let Some(table) = lint_table {
            registry = registry.filtered(table)?;
        }
        let ctx = LintContext::new(&document, source_map, source);
        registry.run(&ctx, &mut sink);
    }

    Ok(LintResult { sink })
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::literal_string_with_formatting_args,
        clippy::unwrap_used
    )]

    use std::path::Path;

    use ltx_config::LintTable;
    use ltx_diagnostics::LtxSeverity;

    use super::{run_pipeline, unified_code};

    fn sample(_name: &str, source: &str) -> String {
        format!("\\documentclass{{article}}\n\\begin{{document}}\n{source}\n\\end{{document}}\n")
    }

    #[test]
    fn valid_source_produces_no_diagnostics() {
        let result = run_pipeline(
            &sample("valid", "Hello world!"),
            Path::new("main.tex"),
            None,
            true,
        )
        .expect("pipeline should succeed");
        assert!(result.is_empty());
        assert!(!result.has_errors());
    }

    #[test]
    fn parse_errors_are_reported_under_unified_codes() {
        let source = "\\begin{minipage}\n\\end{minipage}\n\\end{document}\n";
        let result = run_pipeline(source, Path::new("main.tex"), None, true)
            .expect("pipeline should succeed");
        assert!(result.has_errors());
        let codes: Vec<String> = result
            .diagnostics()
            .iter()
            .map(|diag| diag.error.code().map(|c| c.to_string()).unwrap_or_default())
            .collect();
        assert!(!codes.is_empty(), "expected parse errors, got none");
        for code in &codes {
            assert!(
                code.starts_with("LTX::LINTER::E"),
                "expected a unified LTX::LINTER::E code, got {code}"
            );
        }
    }

    #[test]
    fn unified_code_maps_lexer_and_parser_codes() {
        assert_eq!(unified_code("LTX::LEXER::E011"), "LTX::LINTER::E011");
        assert_eq!(unified_code("LTX::PARSER::E005"), "LTX::LINTER::E016");
        assert_eq!(unified_code("LTX::LINTER::W001"), "LTX::LINTER::E000");
    }

    #[test]
    fn lint_findings_are_reported_as_warnings() {
        let source = sample("unused", "\\label{fig:x}");
        let result = run_pipeline(&source, Path::new("main.tex"), None, true)
            .expect("pipeline should succeed");
        assert!(!result.has_errors());
        assert_eq!(result.warning_count(), 1);
        let codes: Vec<String> = result
            .diagnostics()
            .iter()
            .map(|diag| diag.error.code().map(|c| c.to_string()).unwrap_or_default())
            .collect();
        assert!(codes.iter().any(|c| c == "LTX::LINTER::W001"));
    }

    #[test]
    fn fragment_without_document_environment_is_not_an_error() {
        let source = "\\section{Dummy}\nSome text.\n";
        let result = run_pipeline(source, Path::new("chap.tex"), None, true)
            .expect("pipeline should succeed");
        assert!(!result.has_errors());
        assert!(!result.diagnostics().iter().any(|diag| {
            diag.error
                .code()
                .is_some_and(|c| c.to_string() == "LTX::LINTER::E017")
        }));
    }

    #[test]
    fn document_without_document_environment_is_still_an_error() {
        let source = "\\documentclass{article}\n\\section{Dummy}\n";
        let result = run_pipeline(source, Path::new("main.tex"), None, true)
            .expect("pipeline should succeed");
        assert!(result.has_errors());
        assert!(result.diagnostics().iter().any(|diag| {
            diag.error
                .code()
                .is_some_and(|c| c.to_string() == "LTX::LINTER::E017")
        }));
    }

    #[test]
    fn linting_can_be_disabled() {
        let source = sample("unused", "\\label{fig:x}");
        let result = run_pipeline(&source, Path::new("main.tex"), None, false)
            .expect("pipeline should succeed");
        assert!(result.is_empty());
    }

    #[test]
    fn unknown_lint_rule_is_an_error() {
        let mut table = LintTable::new();
        table.deny.push("does-not-exist".to_string());
        let Err(err) = run_pipeline(
            "\\begin{document}\n\\end{document}",
            Path::new("main.tex"),
            Some(&table),
            true,
        ) else {
            panic!("expected an error");
        };
        assert!(err.to_string().contains("unknown lint rule"));
    }

    #[test]
    fn denied_rule_escalates_to_an_error() {
        let mut table = LintTable::new();
        table.deny.push("long-line".to_string());
        let source = sample(
            "long",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        );
        let result = run_pipeline(&source, Path::new("main.tex"), Some(&table), true)
            .expect("pipeline should succeed");
        assert!(result.has_errors());
        assert!(
            result
                .diagnostics()
                .iter()
                .any(|diag| diag.severity() == LtxSeverity::Error)
        );
    }
}
