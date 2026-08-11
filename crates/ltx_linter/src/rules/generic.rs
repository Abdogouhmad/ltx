//! Parameterized rules reused across multiple codes.
//!
//! - [`DefineUseTracker`] powers `unused-label` (W001), `unused-macro` (W002)
//!   and `unused-package` (W004).
//! - [`TableLookupRule`] powers `deprecated-command` (W005) and
//!   `deprecated-package` (W009).
//! - [`ConsecutiveLineRule`] powers `multiple-blank-lines` (W013).

use std::borrow::Cow;
use std::collections::HashSet;

use ltx_diagnostics::{LtxDiagnosticSink, LtxSeverity, LtxSpan};
use ltx_parser::Visitor;
use ltx_parser::ast::{Command, UsePackage};

use crate::context::LintContext;
use crate::error::emit;
use crate::rule::{AstLintRule, LineLintRule};
use crate::rules::{braced_inner, inner_span};

/// Commands each package provides, used to decide whether a loaded package is
/// actually used (`unused-package`, W004).
///
/// Intentionally empty for now — `unused-package` stays off by default until
/// this table is populated, otherwise every loaded package would be reported
/// as unused.
pub static PACKAGE_COMMANDS: &[(&str, &[&str])] = &[];

/// Deprecated commands mapped to their modern replacement.
pub static DEPRECATED_COMMANDS: &[(&str, &str)] = &[
    ("bf", "textbf"),
    ("it", "textit"),
    ("rm", "textrm"),
    ("sf", "textsf"),
    ("sc", "textsc"),
    ("sl", "textsl"),
    ("tt", "texttt"),
    ("centerline", "centering"),
];

/// Deprecated packages mapped to their modern replacement.
pub static DEPRECATED_PACKAGES: &[(&str, &str)] = &[
    ("epsfig", "graphicx"),
    ("subfigure", "subcaption"),
    ("subfig", "subcaption"),
    ("cite", "natbib"),
    ("color", "xcolor"),
    ("fullpage", "geometry"),
];

// ──────────────────────────────────────────────────────────────────────────────
// matchers
// ──────────────────────────────────────────────────────────────────────────────

/// Matcher for a define-form command: returns the defined name and its span.
type DefineMatcher = Box<dyn for<'a> Fn(&Command<'a>, &str) -> Option<(String, LtxSpan)>>;

/// Matcher for a use-form command: returns the referenced name.
type UseMatcher = Box<dyn for<'a> Fn(&Command<'a>, &str) -> Option<String>>;

/// Matcher for a command table lookup: returns the key and its span.
type CommandMatcher = Box<dyn for<'a> Fn(&Command<'a>) -> Option<(&'static str, LtxSpan)>>;

/// Matcher for a package table lookup: returns the key and its span.
type PackageMatcher = Box<dyn for<'a> Fn(&UsePackage<'a>) -> Option<(&'static str, LtxSpan)>>;

/// Matches `\label{name}` definitions.
fn label_define(cmd: &Command<'_>, source: &str) -> Option<(String, LtxSpan)> {
    if cmd.name != "label" {
        return None;
    }
    let group = cmd.braced_args().next()?;
    let name = braced_inner(group, source).trim();
    if name.is_empty() {
        None
    } else {
        Some((name.to_string(), inner_span(group)))
    }
}

/// Matches references that use a label (`\ref`, `\eqref`, ...).
fn label_use(cmd: &Command<'_>, source: &str) -> Option<String> {
    if !matches!(
        cmd.name,
        "ref" | "eqref" | "pageref" | "autoref" | "vref" | "nameref" | "cref" | "Cref"
    ) {
        return None;
    }
    let group = cmd.braced_args().next()?;
    let name = braced_inner(group, source).trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// Matches `\newcommand{\name}`-style macro definitions.
fn macro_define(cmd: &Command<'_>, source: &str) -> Option<(String, LtxSpan)> {
    if !matches!(
        cmd.name,
        "newcommand" | "renewcommand" | "providecommand" | "DeclareRobustCommand"
    ) {
        return None;
    }
    let group = cmd.braced_args().next()?;
    let name = braced_inner(group, source)
        .trim()
        .strip_prefix('\\')?
        .trim();
    if name.is_empty() {
        None
    } else {
        Some((name.to_string(), inner_span(group)))
    }
}

/// Records every command name, so macros used anywhere are counted as used.
///
/// Returns `Some` unconditionally to match the [`UseMatcher`] signature.
#[allow(clippy::unnecessary_wraps)]
fn macro_use(cmd: &Command<'_>, _source: &str) -> Option<String> {
    Some(cmd.name.to_string())
}

/// Matches `\usepackage{name}` when it appears as a command.
///
/// Note: in the preamble `\usepackage` is parsed as a
/// [`UsePackage`] node, which this define tracker does not visit — one more
/// reason `unused-package` stays off by default.
fn package_define(cmd: &Command<'_>, source: &str) -> Option<(String, LtxSpan)> {
    if cmd.name != "usepackage" {
        return None;
    }
    let group = cmd.braced_args().next()?;
    let name = braced_inner(group, source).trim();
    if name.is_empty() {
        None
    } else {
        Some((name.to_string(), inner_span(group)))
    }
}

/// Matches commands known to be provided by a loaded package.
fn package_use(cmd: &Command<'_>, _source: &str) -> Option<String> {
    PACKAGE_COMMANDS
        .iter()
        .find(|(_, commands)| commands.contains(&cmd.name))
        .map(|(package, _)| (*package).to_string())
}

// ──────────────────────────────────────────────────────────────────────────────
// DefineUseTracker
// ──────────────────────────────────────────────────────────────────────────────

/// Tracks definitions and usages of named entities and reports those that are
/// defined but never used.
pub struct DefineUseTracker<'src> {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    source: &'src str,
    defined: Vec<(String, LtxSpan)>,
    used: HashSet<String>,
    define_matcher: DefineMatcher,
    use_matcher: UseMatcher,
    message: fn(&str) -> Cow<'static, str>,
}

impl<'src> DefineUseTracker<'src> {
    fn new(
        code: &'static str,
        slug: &'static str,
        source: &'src str,
        define_matcher: DefineMatcher,
        use_matcher: UseMatcher,
        message: fn(&str) -> Cow<'static, str>,
    ) -> Self {
        Self {
            code,
            slug,
            severity: LtxSeverity::Warning,
            source,
            defined: Vec::new(),
            used: HashSet::new(),
            define_matcher,
            use_matcher,
            message,
        }
    }
}

impl<'src> Visitor<'src> for DefineUseTracker<'src> {
    fn visit_command(&mut self, cmd: &Command<'src>) {
        if let Some((name, span)) = (self.define_matcher)(cmd, self.source) {
            self.defined.push((name, span));
        }
        if let Some(name) = (self.use_matcher)(cmd, self.source) {
            self.used.insert(name);
        }
    }
}

impl<'src> AstLintRule<'src> for DefineUseTracker<'src> {
    #[inline]
    fn code(&self) -> &'static str {
        self.code
    }

    #[inline]
    fn slug(&self) -> &'static str {
        self.slug
    }

    #[inline]
    fn default_severity(&self) -> LtxSeverity {
        LtxSeverity::Warning
    }

    #[inline]
    fn set_severity(&mut self, severity: LtxSeverity) {
        self.severity = severity;
    }

    fn finish(&mut self, ctx: &LintContext<'_, 'src>, sink: &mut LtxDiagnosticSink) {
        for (name, span) in self.defined.drain(..) {
            if !self.used.contains(&name) {
                emit(
                    sink,
                    ctx,
                    self.code,
                    self.severity,
                    (self.message)(&name),
                    span,
                );
            }
        }
    }
}

/// Rule for `unused-label` (W001).
#[must_use]
pub fn unused_label(source: &str) -> DefineUseTracker<'_> {
    DefineUseTracker::new(
        "LTX::LINTER::W001",
        "unused-label",
        source,
        Box::new(label_define),
        Box::new(label_use),
        |name| Cow::Owned(format!("label `{name}` is defined but never used")),
    )
}

/// Rule for `unused-macro` (W002).
#[must_use]
pub fn unused_macro(source: &str) -> DefineUseTracker<'_> {
    DefineUseTracker::new(
        "LTX::LINTER::W002",
        "unused-macro",
        source,
        Box::new(macro_define),
        Box::new(macro_use),
        |name| Cow::Owned(format!("macro `\\{name}` is defined but never used")),
    )
}

/// Rule for `unused-package` (W004).
///
/// Off by default: [`PACKAGE_COMMANDS`] is empty, and preamble
/// `\usepackage` calls aren't visited by the define tracker.
#[must_use]
pub fn unused_package(source: &str) -> DefineUseTracker<'_> {
    DefineUseTracker::new(
        "LTX::LINTER::W004",
        "unused-package",
        source,
        Box::new(package_define),
        Box::new(package_use),
        |name| Cow::Owned(format!("package `{name}` is loaded but never used")),
    )
}

// ──────────────────────────────────────────────────────────────────────────────
// TableLookupRule
// ──────────────────────────────────────────────────────────────────────────────

/// Looks up commands/packages against a static table and reports matches.
pub struct TableLookupRule {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    table: &'static [(&'static str, &'static str)],
    command_matcher: CommandMatcher,
    package_matcher: Option<PackageMatcher>,
    message: fn(&str, &str) -> Cow<'static, str>,
    findings: Vec<(LtxSpan, Cow<'static, str>)>,
}

impl TableLookupRule {
    fn new(
        code: &'static str,
        slug: &'static str,
        table: &'static [(&'static str, &'static str)],
        command_matcher: CommandMatcher,
        package_matcher: Option<PackageMatcher>,
        message: fn(&str, &str) -> Cow<'static, str>,
    ) -> Self {
        Self {
            code,
            slug,
            severity: LtxSeverity::Warning,
            table,
            command_matcher,
            package_matcher,
            message,
            findings: Vec::new(),
        }
    }

    fn replacement(&self, key: &str) -> Option<&'static str> {
        self.table
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, replacement)| *replacement)
    }
}

impl<'src> Visitor<'src> for TableLookupRule {
    fn visit_command(&mut self, cmd: &Command<'src>) {
        if let Some((key, span)) = (self.command_matcher)(cmd) {
            if let Some(replacement) = self.replacement(key) {
                self.findings.push((span, (self.message)(key, replacement)));
            }
        }
    }

    fn visit_use_package(&mut self, pkg: &UsePackage<'src>) {
        if let Some(matcher) = &self.package_matcher {
            if let Some((key, span)) = matcher(pkg) {
                if let Some(replacement) = self.replacement(key) {
                    self.findings.push((span, (self.message)(key, replacement)));
                }
            }
        }
    }
}

impl<'src> AstLintRule<'src> for TableLookupRule {
    #[inline]
    fn code(&self) -> &'static str {
        self.code
    }

    #[inline]
    fn slug(&self) -> &'static str {
        self.slug
    }

    #[inline]
    fn default_severity(&self) -> LtxSeverity {
        LtxSeverity::Warning
    }

    #[inline]
    fn set_severity(&mut self, severity: LtxSeverity) {
        self.severity = severity;
    }

    fn finish(&mut self, ctx: &LintContext<'_, 'src>, sink: &mut LtxDiagnosticSink) {
        for (span, message) in self.findings.drain(..) {
            emit(sink, ctx, self.code, self.severity, message, span);
        }
    }
}

fn deprecated_command_matcher(cmd: &Command<'_>) -> Option<(&'static str, LtxSpan)> {
    DEPRECATED_COMMANDS
        .iter()
        .find(|(name, _)| *name == cmd.name)
        .map(|(name, _)| (*name, cmd.span))
}

fn deprecated_package_matcher(pkg: &UsePackage<'_>) -> Option<(&'static str, LtxSpan)> {
    DEPRECATED_PACKAGES
        .iter()
        .find(|(name, _)| *name == pkg.package_name)
        .map(|(name, _)| (*name, pkg.span))
}

fn deprecated_command_message(key: &str, replacement: &str) -> Cow<'static, str> {
    Cow::Owned(format!(
        "`\\{key}` is deprecated; use `\\{replacement}` instead"
    ))
}

fn deprecated_package_message(key: &str, replacement: &str) -> Cow<'static, str> {
    Cow::Owned(format!(
        "package `{key}` is deprecated; use `{replacement}` instead"
    ))
}

/// Rule for `deprecated-command` (W005).
#[must_use]
pub fn deprecated_command() -> TableLookupRule {
    TableLookupRule::new(
        "LTX::LINTER::W005",
        "deprecated-command",
        DEPRECATED_COMMANDS,
        Box::new(deprecated_command_matcher),
        None,
        deprecated_command_message,
    )
}

/// Rule for `deprecated-package` (W009).
#[must_use]
pub fn deprecated_package() -> TableLookupRule {
    TableLookupRule::new(
        "LTX::LINTER::W009",
        "deprecated-package",
        DEPRECATED_PACKAGES,
        Box::new(|_cmd| None),
        Some(Box::new(deprecated_package_matcher)),
        deprecated_package_message,
    )
}

// ──────────────────────────────────────────────────────────────────────────────
// ConsecutiveLineRule
// ──────────────────────────────────────────────────────────────────────────────

/// Flags lines where `predicate` holds for more than `max` consecutive lines.
pub struct ConsecutiveLineRule {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    max: u8,
    predicate: fn(&str) -> bool,
    run: u8,
}

impl ConsecutiveLineRule {
    fn new(code: &'static str, slug: &'static str, max: u8, predicate: fn(&str) -> bool) -> Self {
        Self {
            code,
            slug,
            severity: LtxSeverity::Warning,
            max,
            predicate,
            run: 0,
        }
    }
}

impl LineLintRule for ConsecutiveLineRule {
    #[inline]
    fn code(&self) -> &'static str {
        self.code
    }

    #[inline]
    fn slug(&self) -> &'static str {
        self.slug
    }

    #[inline]
    fn default_severity(&self) -> LtxSeverity {
        LtxSeverity::Warning
    }

    #[inline]
    fn set_severity(&mut self, severity: LtxSeverity) {
        self.severity = severity;
    }

    fn check_line(
        &mut self,
        line_no: u32,
        line: &str,
        ctx: &LintContext<'_, '_>,
        sink: &mut LtxDiagnosticSink,
    ) {
        if (self.predicate)(line) {
            self.run = self.run.saturating_add(1);
            if self.run > self.max {
                let start = ctx.line_start(line_no);
                let span = ctx.span(start, start + line.len());
                emit(
                    sink,
                    ctx,
                    self.code,
                    self.severity,
                    Cow::Borrowed("multiple consecutive blank lines"),
                    span,
                );
            }
        } else {
            self.run = 0;
        }
    }
}

fn is_blank(line: &str) -> bool {
    line.trim().is_empty()
}

/// Rule for `multiple-blank-lines` (W013).
#[must_use]
pub fn multiple_blank_lines() -> ConsecutiveLineRule {
    ConsecutiveLineRule::new("LTX::LINTER::W013", "multiple-blank-lines", 1, is_blank)
}
