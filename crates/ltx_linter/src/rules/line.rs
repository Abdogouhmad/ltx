//! Line-scanning rules:
//!
//! - `trailing-whitespace` (W008)
//! - `long-line` (W010)
//! - `mixed-indentation` (W011)
//! - `todo-comment` (W015, stubbed)

use std::borrow::Cow;

use ltx_diagnostics::{LtxDiagnosticSink, LtxSeverity};

use crate::context::LintContext;
use crate::error::emit;
use crate::rule::{LineLintRule, rule_identity};

/// Flags lines ending in spaces or tabs (`trailing-whitespace`, W008).
pub struct TrailingWhitespace {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
}

impl LineLintRule for TrailingWhitespace {
    rule_identity!(TrailingWhitespace);

    fn check_line(
        &mut self,
        line_no: u32,
        line: &str,
        ctx: &LintContext<'_, '_>,
        sink: &mut LtxDiagnosticSink,
    ) {
        let without_cr = line.trim_end_matches('\r');
        let ws_start = without_cr
            .rfind(|c| c != ' ' && c != '\t')
            .map_or(0, |i| i + 1);
        if ws_start < without_cr.len() {
            let start = ctx.line_start(line_no) + ws_start;
            let span = ctx.span(start, start + (without_cr.len() - ws_start));
            emit(
                sink,
                ctx,
                self.code,
                self.severity,
                Cow::Borrowed("line has trailing whitespace"),
                span,
            );
        }
    }
}

/// Rule for `trailing-whitespace` (W008).
#[must_use]
pub const fn trailing_whitespace() -> TrailingWhitespace {
    TrailingWhitespace {
        code: "LTX::LINTER::W008",
        slug: "trailing-whitespace",
        severity: LtxSeverity::Warning,
    }
}

/// Flags lines longer than a configurable limit (`long-line`, W010).
pub struct LongLine {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
    max_len: u16,
}

impl LineLintRule for LongLine {
    rule_identity!(LongLine);

    fn check_line(
        &mut self,
        line_no: u32,
        line: &str,
        ctx: &LintContext<'_, '_>,
        sink: &mut LtxDiagnosticSink,
    ) {
        let len = line.chars().count();
        if len > usize::from(self.max_len) {
            let start = ctx.line_start(line_no);
            let span = ctx.span(start, start + line.len());
            emit(
                sink,
                ctx,
                self.code,
                self.severity,
                Cow::Owned(format!(
                    "line is {len} characters long (limit {})",
                    self.max_len
                )),
                span,
            );
        }
    }
}

/// Rule for `long-line` (W010), default limit of 100 characters.
#[must_use]
pub const fn long_line() -> LongLine {
    LongLine {
        code: "LTX::LINTER::W010",
        slug: "long-line",
        severity: LtxSeverity::Warning,
        max_len: 100,
    }
}

/// Flags indentation that mixes spaces and tabs (`mixed-indentation`, W011).
pub struct MixedIndentation {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
}

impl LineLintRule for MixedIndentation {
    rule_identity!(MixedIndentation);

    fn check_line(
        &mut self,
        line_no: u32,
        line: &str,
        ctx: &LintContext<'_, '_>,
        sink: &mut LtxDiagnosticSink,
    ) {
        let content = line.trim_start_matches([' ', '\t']);
        let indent_len = line.len() - content.len();
        if indent_len > 0 && line[..indent_len].contains(' ') && line[..indent_len].contains('\t') {
            let start = ctx.line_start(line_no);
            let span = ctx.span(start, start + indent_len);
            emit(
                sink,
                ctx,
                self.code,
                self.severity,
                Cow::Borrowed("indentation mixes spaces and tabs"),
                span,
            );
        }
    }
}

/// Rule for `mixed-indentation` (W011).
#[must_use]
pub const fn mixed_indentation() -> MixedIndentation {
    MixedIndentation {
        code: "LTX::LINTER::W011",
        slug: "mixed-indentation",
        severity: LtxSeverity::Warning,
    }
}

/// Rule for `todo-comment` (W015) — **stubbed**.
///
/// Intentionally not wired into [`crate::LintRegistry::with_defaults`]. The
/// linter-feat doc blocks this rule until comment handling lands in the
/// linter; it must not be implemented as a raw `%` string search, which would
/// produce false positives inside verbatim and math.
pub struct TodoComment {
    code: &'static str,
    slug: &'static str,
    severity: LtxSeverity,
}

impl LineLintRule for TodoComment {
    rule_identity!(TodoComment);

    // check_line intentionally left as the default no-op: W015 is stubbed.
}

/// Rule for `todo-comment` (W015) — stubbed, not part of the default registry.
#[must_use]
pub const fn todo_comment() -> TodoComment {
    TodoComment {
        code: "LTX::LINTER::W015",
        slug: "todo-comment",
        severity: LtxSeverity::Warning,
    }
}
