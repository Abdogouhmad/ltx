//! The [`LintRegistry`], which owns and runs every enabled lint rule.

use std::collections::HashSet;

use ltx_config::LintTable;
use ltx_diagnostics::{LtxDiagnosticSink, LtxSeverity};

use crate::context::LintContext;
use crate::error::LintError;
use crate::rule::LintRule;
use crate::rules::{
    deprecated_command, deprecated_package, duplicate_package_import, empty_command,
    empty_environment, empty_section, long_line, missing_caption, mixed_indentation,
    multiple_blank_lines, redundant_braces, trailing_whitespace, unused_label, unused_macro,
};

/// The set of lint rules enabled for one lint run.
pub struct LintRegistry<'src> {
    rules: Vec<LintRule<'src>>,
}

impl<'src> LintRegistry<'src> {
    /// Builds a registry with the default rule set for `raw_source`.
    ///
    /// `raw_source` must be the same text the document being linted was parsed
    /// from; rules that inspect braced-argument text slice it directly.
    ///
    /// [`todo_comment`](crate::rules::todo_comment) and
    /// [`unused_package`](crate::rules::unused_package) are deliberately
    /// excluded — the former is stubbed, the latter needs a populated
    /// `PACKAGE_COMMANDS` table first.
    #[must_use]
    pub fn with_defaults(raw_source: &'src str) -> Self {
        Self {
            rules: vec![
                LintRule::Ast(Box::new(unused_label(raw_source))),
                LintRule::Ast(Box::new(unused_macro(raw_source))),
                LintRule::Ast(Box::new(deprecated_command())),
                LintRule::Ast(Box::new(deprecated_package())),
                LintRule::Ast(Box::new(duplicate_package_import())),
                LintRule::Ast(Box::new(empty_environment())),
                LintRule::Ast(Box::new(empty_section())),
                LintRule::Ast(Box::new(empty_command(raw_source))),
                LintRule::Ast(Box::new(redundant_braces(raw_source))),
                LintRule::Ast(Box::new(missing_caption())),
                LintRule::Line(Box::new(trailing_whitespace())),
                LintRule::Line(Box::new(long_line())),
                LintRule::Line(Box::new(mixed_indentation())),
                LintRule::Line(Box::new(multiple_blank_lines())),
            ],
        }
    }

    /// Filters the registry against a [`LintTable`], dropping `allow`ed rules
    /// and upgrading `deny`ed rules to errors.
    ///
    /// # Errors
    ///
    /// - [`LintError::UnknownRule`] if a table slug matches no registered rule.
    /// - [`LintError::ConflictingDirective`] if a slug is in both `deny` and
    ///   `allow`.
    pub fn filtered(mut self, table: &LintTable) -> Result<Self, LintError> {
        let known: HashSet<&str> = self.rules.iter().map(LintRule::slug).collect();

        for slug in table.deny.iter().chain(&table.warn).chain(&table.allow) {
            if !known.contains(slug.as_str()) {
                return Err(LintError::UnknownRule {
                    slug: slug.clone().into(),
                });
            }
        }

        for rule in &mut self.rules {
            let slug = rule.slug();
            let in_deny = table.deny.iter().any(|s| s == slug);
            let in_allow = table.allow.iter().any(|s| s == slug);

            if in_deny && in_allow {
                return Err(LintError::ConflictingDirective {
                    slug: slug.to_owned().into(),
                });
            }
            if in_deny {
                rule.set_severity(LtxSeverity::Error);
            } else if table.warn.iter().any(|s| s == slug) {
                rule.set_severity(LtxSeverity::Warning);
            }
        }

        self.rules
            .retain(|rule| !table.allow.iter().any(|s| s == rule.slug()));
        Ok(self)
    }

    /// Runs every rule against `ctx`, pushing findings into `sink`.
    pub fn run(&mut self, ctx: &LintContext<'_, 'src>, sink: &mut LtxDiagnosticSink) {
        for rule in &mut self.rules {
            match rule {
                LintRule::Ast(ast) => {
                    ast.visit_document(ctx.document);
                    ast.finish(ctx, sink);
                }
                LintRule::Line(line) => line.check_source(ctx, sink),
            }
        }
    }
}
