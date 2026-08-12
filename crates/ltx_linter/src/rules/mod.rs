//! Concrete lint rule implementations.
//!
//! - [`generic`] — parameterized rules reused across several codes
//!   (define/use tracking, table lookup, consecutive-line counting).
//! - [`ast`] — structural rules that need their own visitor logic.
//! - [`crate::rules::line`] — line-scanning rules.

pub mod ast;
pub mod generic;
pub mod line;

pub use ast::{
    duplicate_package_import, empty_command, empty_environment, empty_section, missing_caption,
    redundant_braces,
};
pub use generic::{
    deprecated_command, deprecated_package, multiple_blank_lines, unused_label, unused_macro,
    unused_package,
};
pub use line::{long_line, mixed_indentation, todo_comment, trailing_whitespace};

/// Returns the inner text of a braced group `{ ... }` by slicing `source`.
///
/// Falls back to the whole group text (without brace stripping) if the span
/// doesn't line up with braces, which should never happen for a well-formed
/// parse.
pub(crate) fn braced_inner<'s>(group: &ltx_parser::ast::Group<'_>, source: &'s str) -> &'s str {
    let start = group.span.start();
    let end = group.span.end();
    let bytes = source.as_bytes();
    let is_braced =
        bytes.get(start) == Some(&b'{') && end > start && bytes.get(end - 1) == Some(&b'}');
    if is_braced {
        &source[start + 1..end - 1]
    } else {
        &source[start..end]
    }
}

/// Span of the inner text of a braced group (excluding the braces).
#[inline]
pub(crate) const fn inner_span(group: &ltx_parser::ast::Group<'_>) -> ltx_diagnostics::LtxSpan {
    let start = group.span.start() + 1;
    let end = group.span.end().saturating_sub(1);
    ltx_diagnostics::LtxSpan::new(start, end, group.span.file_id)
}
