//! Lint rule traits and the [`LintRule`] enum.
//!
//! Two kinds of rules exist:
//!
//! - [`AstLintRule`] — walks the [`Document`](ltx_parser::ast::Document) AST
//!   through `ltx_parser::Visitor`. Collects findings while
//!   visiting and emits them in [`AstLintRule::finish`].
//! - [`LineLintRule`] — scans the raw source line by line through
//!   [`LineLintRule::check_line`].
//!
//! The trait lifetime is tied to the visited source (`'src`) because the
//! parser AST borrows from the source string; rules that need to slice the
//! raw source (e.g. to read the text of a braced argument) are generic over
//! that lifetime too.

use ltx_diagnostics::{LtxDiagnosticSink, LtxSeverity};
use ltx_parser::Visitor;

use crate::context::LintContext;

/// An AST-walking lint rule.
///
/// Implementations override the [`Visitor`] hooks they care about, record
/// findings, then drain them in [`finish`](Self::finish).
#[allow(unused_variables)]
pub trait AstLintRule<'src>: Visitor<'src> {
    /// The diagnostic code of this rule (e.g. `"W001"`).
    fn code(&self) -> &'static str;

    /// The stable machine-readable slug used in `ltx.toml` (e.g. `"unused-label"`).
    fn slug(&self) -> &'static str;

    /// The severity the rule emits at unless overridden by the [`LintTable`](ltx_config::LintTable).
    fn default_severity(&self) -> LtxSeverity;

    /// Overrides the effective severity (used when a rule is `deny`-listed).
    fn set_severity(&mut self, severity: LtxSeverity);

    /// Seeds the rule with usage information collected across a whole project.
    ///
    /// `unused-label` / `unused-macro` use this to learn about references
    /// that live in *other* files (or inside math/groups the AST visitor
    /// doesn't descend into), so a definition used elsewhere isn't reported.
    /// Rules that don't track define/use ignore the seed.
    #[allow(unused_variables)]
    fn seed_uses(&mut self, uses: &crate::session::ProjectUses) {}

    /// Emits any findings collected while visiting.
    ///
    /// Called once per rule, after the document has been fully visited.
    fn finish(
        &mut self,
        ctx: &LintContext<'_, 'src>,
        sink: &mut ltx_diagnostics::LtxDiagnosticSink,
    ) {
    }
}

/// A line-scanning lint rule.
///
/// [`check_source`](Self::check_source) is the default driver: it iterates
/// the lines of the raw source and forwards each one to
/// [`check_line`](Self::check_line). Implementations that need to track
/// state across the whole file (e.g. run counting) just mutate `self` in
/// `check_line`.
#[allow(unused_variables)]
pub trait LineLintRule {
    /// The diagnostic code of this rule (e.g. `"W008"`).
    fn code(&self) -> &'static str;

    /// The stable machine-readable slug used in `ltx.toml` (e.g. `"trailing-whitespace"`).
    fn slug(&self) -> &'static str;

    /// The severity the rule emits at unless overridden by the [`LintTable`](ltx_config::LintTable).
    fn default_severity(&self) -> LtxSeverity;

    /// Overrides the effective severity (used when a rule is `deny`-listed).
    fn set_severity(&mut self, severity: LtxSeverity);

    /// Checks a single line of source.
    ///
    /// `line_no` is 1-based. `line` excludes the trailing newline. `ctx`
    /// provides the raw source and span helpers needed to build diagnostics.
    fn check_line(
        &mut self,
        line_no: u32,
        line: &str,
        ctx: &LintContext<'_, '_>,
        sink: &mut LtxDiagnosticSink,
    ) {
    }

    /// Runs this rule over every line of the source.
    fn check_source(&mut self, ctx: &LintContext<'_, '_>, sink: &mut LtxDiagnosticSink) {
        for (line_no, line) in ctx.lines() {
            self.check_line(line_no, line, ctx, sink);
        }
    }
}

/// A lint rule in one of the two supported kinds.
pub enum LintRule<'src> {
    /// An AST-walking rule.
    Ast(Box<dyn AstLintRule<'src> + 'src>),
    /// A line-scanning rule.
    Line(Box<dyn LineLintRule>),
}

impl LintRule<'_> {
    /// The diagnostic code of this rule.
    #[inline]
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::Ast(rule) => rule.code(),
            Self::Line(rule) => rule.code(),
        }
    }

    /// The stable machine-readable slug of this rule.
    #[inline]
    #[must_use]
    pub fn slug(&self) -> &'static str {
        match self {
            Self::Ast(rule) => rule.slug(),
            Self::Line(rule) => rule.slug(),
        }
    }

    /// The severity this rule emits at by default.
    #[inline]
    #[must_use]
    pub fn default_severity(&self) -> LtxSeverity {
        match self {
            Self::Ast(rule) => rule.default_severity(),
            Self::Line(rule) => rule.default_severity(),
        }
    }

    /// Overrides the effective severity of this rule.
    pub fn set_severity(&mut self, severity: LtxSeverity) {
        match self {
            Self::Ast(rule) => rule.set_severity(severity),
            Self::Line(rule) => rule.set_severity(severity),
        }
    }
}

/// Emits the four shared identity accessors (`code`, `slug`,
/// `default_severity`, `set_severity`) reading the `code`, `slug` and
/// `severity` fields that every rule struct carries.
macro_rules! rule_identity {
    ($rule:ty) => {
        #[inline]
        fn code(&self) -> &'static str {
            self.code
        }

        #[inline]
        fn slug(&self) -> &'static str {
            self.slug
        }

        #[inline]
        fn default_severity(&self) -> ltx_diagnostics::LtxSeverity {
            self.severity
        }

        #[inline]
        fn set_severity(&mut self, severity: ltx_diagnostics::LtxSeverity) {
            self.severity = severity;
        }
    };
}

/// Implements [`AstLintRule`] for a rule that records findings as
/// `findings: Vec<(LtxSpan, Cow<'static, str>)>` and emits them all in
/// `finish`. Rules with custom `finish` logic keep a manual impl and use
/// [`rule_identity!`](self::rule_identity) for the shared accessors.
///
/// Use the `src` marker for rules generic over the visited source lifetime
/// (e.g. `ast_findings_rule!(src EmptyCommand)`).
macro_rules! ast_findings_rule {
    (src $rule:ident) => {
        impl<'src> $crate::rule::AstLintRule<'src> for $rule<'src> {
            rule_identity!($rule<'src>);

            fn finish(
                &mut self,
                ctx: &$crate::context::LintContext<'_, 'src>,
                sink: &mut ltx_diagnostics::LtxDiagnosticSink,
            ) {
                for (span, message) in self.findings.drain(..) {
                    $crate::error::emit(sink, ctx, self.code, self.severity, message, span);
                }
            }
        }
    };
    ($rule:ty) => {
        impl<'src> $crate::rule::AstLintRule<'src> for $rule {
            rule_identity!($rule);

            fn finish(
                &mut self,
                ctx: &$crate::context::LintContext<'_, 'src>,
                sink: &mut ltx_diagnostics::LtxDiagnosticSink,
            ) {
                for (span, message) in self.findings.drain(..) {
                    $crate::error::emit(sink, ctx, self.code, self.severity, message, span);
                }
            }
        }
    };
}

pub(crate) use ast_findings_rule;
pub(crate) use rule_identity;
