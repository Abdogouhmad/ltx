//! Structural rules that each need their own visitor logic:
//!
//! - `duplicate-package-import` (W003)
//! - `empty-environment` (W006)
//! - `empty-section` (W007)
//! - `redundant-braces` (W012)
//! - `missing-caption` (W014)
//! - `empty-command` (W016)

use std::borrow::Cow;
use std::collections::HashSet;

use ltx_diagnostics::{LtxDiagnosticSink, LtxSeverity, LtxSpan};
use ltx_parser::Visitor;
use ltx_parser::ast::{Command, Document, DocumentBodyNode, Environment, UsePackage};
use ltx_parser::visitor::{walk_body_node, walk_command};

use crate::context::LintContext;
use crate::error::emit;
use crate::rule::AstLintRule;
use crate::rules::{braced_inner, inner_span};

/// Flags duplicate package imports (`unused-package`, W003).
///
/// Reports both `\usepackage{a,a}` within one call and a package imported a
/// second time later in the preamble.
pub struct DuplicatePackageImport {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    seen: HashSet<String>,
    findings: Vec<(LtxSpan, Cow<'static, str>)>,
}

impl<'src> Visitor<'src> for DuplicatePackageImport {
    fn visit_use_package(&mut self, pkg: &UsePackage<'src>) {
        let mut local = HashSet::new();
        for name in pkg
            .package_name
            .split(',')
            .map(str::trim)
            .filter(|n| !n.is_empty())
        {
            if !local.insert(name) || !self.seen.insert(name.to_string()) {
                self.findings.push((
                    pkg.span,
                    Cow::Owned(format!("package `{name}` is imported more than once")),
                ));
            }
        }
    }
}

impl<'src> AstLintRule<'src> for DuplicatePackageImport {
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

/// Rule for `duplicate-package-import` (W003).
#[must_use]
pub fn duplicate_package_import() -> DuplicatePackageImport {
    DuplicatePackageImport {
        code: "LTX::LINTER::W003",
        slug: "duplicate-package-import",
        severity: LtxSeverity::Warning,
        seen: HashSet::new(),
        findings: Vec::new(),
    }
}

/// Flags environments whose body is empty or whitespace-only (`empty-environment`, W006).
pub struct EmptyEnvironment {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    findings: Vec<(LtxSpan, Cow<'static, str>)>,
}

impl<'src> Visitor<'src> for EmptyEnvironment {
    fn visit_environment(&mut self, env: &Environment<'src>) {
        let empty = env.body.iter().all(|node| match node {
            DocumentBodyNode::Text(text) => text.text.trim().is_empty(),
            _ => false,
        });
        if empty {
            self.findings.push((
                env.span,
                Cow::Owned(format!("environment `{}` is empty", env.name)),
            ));
        }
    }
}

impl<'src> AstLintRule<'src> for EmptyEnvironment {
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

/// Rule for `empty-environment` (W006).
#[must_use]
pub const fn empty_environment() -> EmptyEnvironment {
    EmptyEnvironment {
        code: "LTX::LINTER::W006",
        slug: "empty-environment",
        severity: LtxSeverity::Warning,
        findings: Vec::new(),
    }
}

/// Flags sections with no content before the next section heading (`empty-section`, W007).
///
/// Requires lookahead over the document body — a section can only be judged
/// empty once the next heading (or end of document) is seen.
pub struct EmptySection {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    pending_section: Option<LtxSpan>,
    has_content: bool,
    findings: Vec<(LtxSpan, Cow<'static, str>)>,
}

fn is_section_cmd(name: &str) -> bool {
    matches!(
        name,
        "part"
            | "chapter"
            | "section"
            | "subsection"
            | "subsubsection"
            | "paragraph"
            | "subparagraph"
    )
}

impl<'src> Visitor<'src> for EmptySection {
    fn visit_document(&mut self, doc: &Document<'src>) {
        for node in &doc.body {
            if let DocumentBodyNode::Command(cmd) = node {
                if is_section_cmd(cmd.name) {
                    self.close_pending();
                    self.pending_section = Some(cmd.span);
                    self.has_content = false;
                    continue;
                }
            }
            match node {
                DocumentBodyNode::Text(text) if text.text.trim().is_empty() => {}
                DocumentBodyNode::Comment(_) => {}
                _ => self.has_content = true,
            }
        }
    }
}

impl EmptySection {
    fn close_pending(&mut self) {
        if let Some(span) = self.pending_section.take() {
            if !self.has_content {
                self.findings.push((
                    span,
                    Cow::Borrowed("empty section — no content before the next section heading"),
                ));
            }
        }
    }
}

impl<'src> AstLintRule<'src> for EmptySection {
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
        self.close_pending();
        for (span, message) in self.findings.drain(..) {
            emit(sink, ctx, self.code, self.severity, message, span);
        }
    }
}

/// Rule for `empty-section` (W007).
#[must_use]
pub const fn empty_section() -> EmptySection {
    EmptySection {
        code: "LTX::LINTER::W007",
        slug: "empty-section",
        severity: LtxSeverity::Warning,
        pending_section: None,
        has_content: false,
        findings: Vec::new(),
    }
}

/// Flags standalone `{ ... }` groups wrapping a single node (`redundant-braces`, W012).
///
/// Only body-level groups are considered — braced command arguments are not
/// redundant by definition and are not visited here. Groups containing any
/// control sequence are left alone to avoid false positives on scoped
/// declarations like `{\large text}`.
pub struct RedundantBraces<'src> {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    source: &'src str,
    findings: Vec<(LtxSpan, Cow<'static, str>)>,
}

impl<'src> Visitor<'src> for RedundantBraces<'src> {
    fn visit_body_node(&mut self, node: &DocumentBodyNode<'src>) {
        if let DocumentBodyNode::Group(group) = node {
            let inner = braced_inner(group, self.source).trim();
            if !inner.is_empty() && !inner.contains(['{', '}', '\\']) {
                self.findings.push((
                    group.span,
                    Cow::Borrowed("redundant braces around a single node"),
                ));
            }
        }
        walk_body_node(self, node);
    }
}

impl<'src> AstLintRule<'src> for RedundantBraces<'src> {
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

/// Rule for `redundant-braces` (W012).
#[must_use]
pub const fn redundant_braces(source: &str) -> RedundantBraces<'_> {
    RedundantBraces {
        code: "LTX::LINTER::W012",
        slug: "redundant-braces",
        severity: LtxSeverity::Warning,
        source,
        findings: Vec::new(),
    }
}

/// Flags float environments without a `\caption` (`missing-caption`, W014).
pub struct MissingCaption {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    findings: Vec<(LtxSpan, Cow<'static, str>)>,
}

fn is_float(name: &str) -> bool {
    matches!(name, "figure" | "figure*" | "table" | "table*")
}

fn has_caption(body: &[DocumentBodyNode<'_>]) -> bool {
    body.iter().any(|node| match node {
        DocumentBodyNode::Command(cmd) => cmd.name == "caption",
        DocumentBodyNode::Environment(env) => has_caption(&env.body),
        _ => false,
    })
}

impl<'src> Visitor<'src> for MissingCaption {
    fn visit_environment(&mut self, env: &Environment<'src>) {
        if is_float(env.name) && !has_caption(&env.body) {
            self.findings.push((
                env.span,
                Cow::Owned(format!("float `{}` has no `\\caption`", env.name)),
            ));
        }
    }
}

impl<'src> AstLintRule<'src> for MissingCaption {
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

/// Rule for `missing-caption` (W014).
#[must_use]
pub const fn missing_caption() -> MissingCaption {
    MissingCaption {
        code: "LTX::LINTER::W014",
        slug: "missing-caption",
        severity: LtxSeverity::Warning,
        findings: Vec::new(),
    }
}

/// Commands whose braced argument is meaningless when empty or whitespace-only.
fn is_empty_sensitive(name: &str) -> bool {
    matches!(
        name,
        "textbf"
            | "textit"
            | "textsf"
            | "textrm"
            | "texttt"
            | "textsc"
            | "textsl"
            | "textmd"
            | "textup"
            | "emph"
            | "underline"
            | "title"
            | "author"
    )
}

/// Flags empty braced arguments to formatting/metadata commands (`empty-command`, W016).
pub struct EmptyCommand<'src> {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    source: &'src str,
    findings: Vec<(LtxSpan, Cow<'static, str>)>,
}

impl<'src> Visitor<'src> for EmptyCommand<'src> {
    fn visit_command(&mut self, cmd: &Command<'src>) {
        if is_empty_sensitive(cmd.name) {
            for group in cmd.braced_args() {
                if braced_inner(group, self.source).trim().is_empty() {
                    self.findings.push((
                        inner_span(group),
                        Cow::Owned(format!("`\\{}` has an empty argument", cmd.name)),
                    ));
                }
            }
        }
        walk_command(self, cmd);
    }
}

impl<'src> AstLintRule<'src> for EmptyCommand<'src> {
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

/// Rule for `empty-command` (W016).
#[must_use]
pub const fn empty_command(source: &str) -> EmptyCommand<'_> {
    EmptyCommand {
        code: "LTX::LINTER::W016",
        slug: "empty-command",
        severity: LtxSeverity::Warning,
        source,
        findings: Vec::new(),
    }
}
