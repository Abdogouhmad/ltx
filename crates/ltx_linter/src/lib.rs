//! The `ltx_linter` crate: the unified front-end for checking LaTeX documents.
//!
//! Runs the full lex → parse → lint pipeline and reports every diagnostic —
//! lexer errors, parser errors, and style findings — under a single
//! `LTX::LINTER::E*` / `LTX::LINTER::W*` code namespace.
//!
//! # Usage
//!
//! The high-level entry point is [`lint_file`]: pass a path and (optionally)
//! the `[lints]` table from `ltx.toml`, and receive a [`LintResult`] with all
//! unified diagnostics.
//!
//! ```no_run
//! # use std::path::Path;
//! use ltx_linter::lint_file;
//! let result = lint_file(Path::new("main.tex"), None, true)?;
//! if result.has_errors() {
//!     if let Ok(rendered) = result.render_pretty() {
//!         println!("{rendered}");
//!     }
//! }
//! # Ok::<(), miette::Report>(())
//! ```
//!
//! Lower-level consumers can still assemble the pipeline by hand:
//!
//! ```no_run
//! use ltx_diagnostics::{LtxDiagnosticSink, LtxSourceMap};
//! use ltx_linter::{LintContext, LintRegistry};
//! use ltx_lexer::{LtxLexer, TokenStream};
//! use ltx_parser::LtxParser;
//! # use std::sync::Arc;
//! # let source = r#"\documentclass{article}
//! # \begin{document}
//! # hello
//! # \end{document}"#;
//!
//! let mut source_map = LtxSourceMap::new();
//! let file_id = source_map.add_inline("main.tex", source);
//! let source_map = Arc::new(source_map);
//!
//! let stream = TokenStream::new(LtxLexer::new(source, file_id, source_map.as_ref().clone()));
//! let mut parser = LtxParser::new(stream);
//! let document = ltx_parser::parse_document(&mut parser);
//!
//! let ctx = LintContext::new(&document, source_map, source);
//! let mut sink = LtxDiagnosticSink::new();
//!
//! LintRegistry::with_defaults(source)
//!     .run(&ctx, &mut sink);
//! ```
//!
//! # Rules
//!
//! | Code | Slug | Kind |
//! |------|------|------|
//! | LTX::LINTER::W001 | `unused-label` | AST |
//! | LTX::LINTER::W002 | `unused-macro` | AST |
//! | LTX::LINTER::W003 | `duplicate-package-import` | AST |
//! | LTX::LINTER::W004 | `unused-package` | AST (off by default) |
//! | LTX::LINTER::W005 | `deprecated-command` | AST |
//! | LTX::LINTER::W006 | `empty-environment` | AST |
//! | LTX::LINTER::W007 | `empty-section` | AST |
//! | LTX::LINTER::W008 | `trailing-whitespace` | line |
//! | LTX::LINTER::W009 | `deprecated-package` | AST |
//! | LTX::LINTER::W010 | `long-line` | line |
//! | LTX::LINTER::W011 | `mixed-indentation` | line |
//! | LTX::LINTER::W012 | `redundant-braces` | AST |
//! | LTX::LINTER::W013 | `multiple-blank-lines` | line |
//! | LTX::LINTER::W014 | `missing-caption` | AST |
//! | LTX::LINTER::W015 | `todo-comment` | line (stubbed) |
//! | LTX::LINTER::W016 | `empty-command` | AST |

pub mod context;
pub mod error;
pub mod registry;
pub mod rule;
pub mod rules;
pub mod session;

pub use context::LintContext;
pub use error::{LintDiagnosticSource, LintError};
pub use registry::LintRegistry;
pub use rule::{AstLintRule, LineLintRule, LintRule};
pub use rules::{
    deprecated_command, deprecated_package, duplicate_package_import, empty_command,
    empty_environment, empty_section, long_line, missing_caption, mixed_indentation,
    multiple_blank_lines, redundant_braces, todo_comment, trailing_whitespace, unused_label,
    unused_macro, unused_package,
};
pub use session::{LintResult, lint_file};

use ltx_diagnostics::ErrorCode;

/// All lint rule codes mapped to their slugs, for `ltx code --lint`.
pub const ALL_RULE_CODES: &[(&str, &str)] = &[
    ("LTX::LINTER::W001", "unused-label"),
    ("LTX::LINTER::W002", "unused-macro"),
    ("LTX::LINTER::W003", "duplicate-package-import"),
    ("LTX::LINTER::W004", "unused-package"),
    ("LTX::LINTER::W005", "deprecated-command"),
    ("LTX::LINTER::W006", "empty-environment"),
    ("LTX::LINTER::W007", "empty-section"),
    ("LTX::LINTER::W008", "trailing-whitespace"),
    ("LTX::LINTER::W009", "deprecated-package"),
    ("LTX::LINTER::W010", "long-line"),
    ("LTX::LINTER::W011", "mixed-indentation"),
    ("LTX::LINTER::W012", "redundant-braces"),
    ("LTX::LINTER::W013", "multiple-blank-lines"),
    ("LTX::LINTER::W014", "missing-caption"),
    ("LTX::LINTER::W015", "todo-comment"),
    ("LTX::LINTER::W016", "empty-command"),
];

/// All diagnostic codes owned by the linter, under the unified
/// `LTX::LINTER::` namespace.
///
/// `E001`–`E011` mirror the lexer codes, `E012`–`E017` mirror the parser
/// codes, `E018`–`E019` are lint-table configuration errors, and `W001`–`W016`
/// are the style lint rules.
pub const ALL_CODES: &[ErrorCode] = &[
    // Unified lexer codes (see [`crate::session::CODE_REMAP`]).
    ErrorCode::new("LTX::LINTER::E001", "Unexpected Token", "error", "linter"),
    ErrorCode::new(
        "LTX::LINTER::E002",
        "Unexpected End of File",
        "error",
        "linter",
    ),
    ErrorCode::new("LTX::LINTER::E003", "Unmatched Brace", "error", "linter"),
    ErrorCode::new(
        "LTX::LINTER::E004",
        "Invalid Math Delimiter",
        "error",
        "linter",
    ),
    ErrorCode::new(
        "LTX::LINTER::E005",
        "Unterminated Argument",
        "error",
        "linter",
    ),
    ErrorCode::new(
        "LTX::LINTER::E006",
        "Invalid Escape Sequence",
        "error",
        "linter",
    ),
    ErrorCode::new("LTX::LINTER::E007", "Invalid Unicode", "error", "linter"),
    ErrorCode::new(
        "LTX::LINTER::E008",
        "Illegal Parameter Character Usage",
        "error",
        "linter",
    ),
    ErrorCode::new(
        "LTX::LINTER::E009",
        "Unterminated Verbatim Block",
        "error",
        "linter",
    ),
    ErrorCode::new("LTX::LINTER::E010", "Invalid Character", "error", "linter"),
    ErrorCode::new(
        "LTX::LINTER::E011",
        "Mismatched Environment",
        "error",
        "linter",
    ),
    // Unified parser codes.
    ErrorCode::new("LTX::LINTER::E012", "Expected Token", "error", "linter"),
    ErrorCode::new(
        "LTX::LINTER::E013",
        "Unexpected End of File",
        "error",
        "linter",
    ),
    ErrorCode::new(
        "LTX::LINTER::E014",
        "Unclosed Environment",
        "error",
        "linter",
    ),
    ErrorCode::new(
        "LTX::LINTER::E015",
        "Mismatched Environment",
        "error",
        "linter",
    ),
    ErrorCode::new(
        "LTX::LINTER::E016",
        "Missing Closing Brace",
        "error",
        "linter",
    ),
    ErrorCode::new(
        "LTX::LINTER::E017",
        "Unexpected End of File While Parsing",
        "error",
        "linter",
    ),
    // Lint table configuration errors.
    ErrorCode::new("LTX::LINTER::E018", "Unknown Lint Rule", "error", "linter"),
    ErrorCode::new(
        "LTX::LINTER::E019",
        "Conflicting Lint Directive",
        "error",
        "linter",
    ),
    // Style lint rules.
    ErrorCode::new("LTX::LINTER::W001", "unused-label", "warning", "linter"),
    ErrorCode::new("LTX::LINTER::W002", "unused-macro", "warning", "linter"),
    ErrorCode::new(
        "LTX::LINTER::W003",
        "duplicate-package-import",
        "warning",
        "linter",
    ),
    ErrorCode::new("LTX::LINTER::W004", "unused-package", "warning", "linter"),
    ErrorCode::new(
        "LTX::LINTER::W005",
        "deprecated-command",
        "warning",
        "linter",
    ),
    ErrorCode::new(
        "LTX::LINTER::W006",
        "empty-environment",
        "warning",
        "linter",
    ),
    ErrorCode::new("LTX::LINTER::W007", "empty-section", "warning", "linter"),
    ErrorCode::new(
        "LTX::LINTER::W008",
        "trailing-whitespace",
        "warning",
        "linter",
    ),
    ErrorCode::new(
        "LTX::LINTER::W009",
        "deprecated-package",
        "warning",
        "linter",
    ),
    ErrorCode::new("LTX::LINTER::W010", "long-line", "warning", "linter"),
    ErrorCode::new(
        "LTX::LINTER::W011",
        "mixed-indentation",
        "warning",
        "linter",
    ),
    ErrorCode::new("LTX::LINTER::W012", "redundant-braces", "warning", "linter"),
    ErrorCode::new(
        "LTX::LINTER::W013",
        "multiple-blank-lines",
        "warning",
        "linter",
    ),
    ErrorCode::new("LTX::LINTER::W014", "missing-caption", "warning", "linter"),
    ErrorCode::new("LTX::LINTER::W015", "todo-comment", "warning", "linter"),
    ErrorCode::new("LTX::LINTER::W016", "empty-command", "warning", "linter"),
];

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ltx_config::LintTable;
    use ltx_diagnostics::{LtxDiagnosticSink, LtxSourceMap};
    use ltx_lexer::{LtxLexer, TokenStream};
    use ltx_parser::{LtxParser, parse_document};

    use crate::LintContext;
    use crate::error::LintError;
    use crate::registry::LintRegistry;

    /// Parses `source`, runs `registry`, and returns `(code, message)` pairs.
    fn run_lints<'src>(
        source: &'src str,
        registry: &mut LintRegistry<'src>,
    ) -> Vec<(String, String)> {
        let mut source_map = LtxSourceMap::new();
        let file_id = source_map.add_inline("test.tex", source);
        let source_map = Arc::new(source_map);

        let stream = TokenStream::new(LtxLexer::new(source, file_id, source_map.as_ref().clone()));
        let mut parser = LtxParser::new(stream);
        let document = parse_document(&mut parser);

        let ctx = LintContext::new(&document, source_map, source);
        let mut sink = LtxDiagnosticSink::new();
        registry.run(&ctx, &mut sink);

        sink.all()
            .iter()
            .map(|diag| {
                let code = diag
                    .error
                    .code()
                    .map(|code| code.to_string())
                    .unwrap_or_default();
                (code, diag.error.to_string())
            })
            .collect()
    }

    #[test]
    #[allow(clippy::literal_string_with_formatting_args)]
    fn unused_label_is_reported_when_not_referenced() {
        let src = r"\documentclass{article}
\begin{document}
\label{fig:x}
\end{document}";
        let mut registry = LintRegistry::with_defaults(src);
        let lints = run_lints(src, &mut registry);
        assert!(
            lints
                .iter()
                .any(|(code, msg)| code == "LTX::LINTER::W001" && msg.contains("fig:x"))
        );
    }

    #[test]
    #[allow(clippy::literal_string_with_formatting_args)]
    fn used_label_is_not_reported() {
        let src = r"\documentclass{article}
\begin{document}
\label{fig:x}
See \ref{fig:x}.
\end{document}";
        let mut registry = LintRegistry::with_defaults(src);
        let lints = run_lints(src, &mut registry);
        assert!(!lints.iter().any(|(code, _)| code == "LTX::LINTER::W001"));
    }

    #[test]
    fn unused_macro_is_reported() {
        let src = r"\documentclass{article}
\begin{document}
\newcommand{\helper}[1]{#1}
\end{document}";
        let mut registry = LintRegistry::with_defaults(src);
        let lints = run_lints(src, &mut registry);
        assert!(
            lints
                .iter()
                .any(|(code, msg)| code == "LTX::LINTER::W002" && msg.contains("helper"))
        );
    }

    #[test]
    fn trailing_whitespace_and_long_line_are_reported() {
        let src = format!(
            r"\documentclass{{article}}
\begin{{document}}
hello   
{}more
\end{{document}}",
            "x".repeat(110)
        );
        let mut registry = LintRegistry::with_defaults(&src);
        let lints = run_lints(&src, &mut registry);
        assert!(lints.iter().any(|(code, _)| code == "LTX::LINTER::W008"));
        assert!(lints.iter().any(|(code, _)| code == "LTX::LINTER::W010"));
    }

    #[test]
    fn multiple_blank_lines_are_reported() {
        let src = r"\documentclass{article}
\begin{document}
hello


world
\end{document}";
        let mut registry = LintRegistry::with_defaults(src);
        let lints = run_lints(src, &mut registry);
        assert!(lints.iter().any(|(code, _)| code == "LTX::LINTER::W013"));
    }

    #[test]
    fn empty_command_is_reported() {
        let src = r"\documentclass{article}
\begin{document}
\textbf{} and \author{}
\end{document}";
        let mut registry = LintRegistry::with_defaults(src);
        let lints = run_lints(src, &mut registry);
        assert!(
            lints
                .iter()
                .filter(|(code, _)| code == "LTX::LINTER::W016")
                .count()
                >= 2
        );
    }

    #[test]
    fn non_empty_command_is_not_reported() {
        let src = r"\documentclass{article}
\begin{document}
\textbf{bold} and \author{Jane Doe}
\end{document}";
        let mut registry = LintRegistry::with_defaults(src);
        let lints = run_lints(src, &mut registry);
        assert!(!lints.iter().any(|(code, _)| code == "LTX::LINTER::W016"));
    }

    #[test]
    fn filtered_denies_upgrade_severity() {
        let long_line = "x".repeat(110);
        let src = format!(
            r"\documentclass{{article}}
\begin{{document}}
{long_line}
\end{{document}}"
        );
        let mut table = LintTable::new();
        table.deny.push("long-line".to_string());
        let Ok(mut registry) = LintRegistry::with_defaults(&src).filtered(&table) else {
            panic!("filtering default rules should not fail");
        };

        let mut source_map = LtxSourceMap::new();
        let file_id = source_map.add_inline("test.tex", &src);
        let source_map = Arc::new(source_map);
        let stream = TokenStream::new(LtxLexer::new(&src, file_id, source_map.as_ref().clone()));
        let mut parser = LtxParser::new(stream);
        let document = parse_document(&mut parser);
        let ctx = LintContext::new(&document, source_map, &src);
        let mut sink = LtxDiagnosticSink::new();
        registry.run(&ctx, &mut sink);
        assert!(sink.all().iter().any(|diag| {
            diag.error
                .code()
                .is_some_and(|c| c.to_string() == "LTX::LINTER::W010")
                && diag.severity().is_error()
        }));
    }

    #[test]
    fn conflicting_directive_is_an_error() {
        let mut table = LintTable::new();
        table.deny.push("long-line".to_string());
        table.allow.push("long-line".to_string());
        let Err(err) = LintRegistry::with_defaults("").filtered(&table) else {
            panic!("expected an error");
        };
        match err {
            LintError::ConflictingDirective { slug } => assert_eq!(slug, "long-line"),
            other @ LintError::UnknownRule { .. } => {
                panic!("expected ConflictingDirective, got {other:?}");
            }
        }
    }

    #[test]
    fn unknown_rule_is_an_error() {
        let mut table = LintTable::new();
        table.deny.push("does-not-exist".to_string());
        let Err(err) = LintRegistry::with_defaults("").filtered(&table) else {
            panic!("expected an error");
        };
        match err {
            LintError::UnknownRule { slug } => assert_eq!(slug, "does-not-exist"),
            other @ LintError::ConflictingDirective { .. } => {
                panic!("expected UnknownRule, got {other:?}");
            }
        }
    }
}
