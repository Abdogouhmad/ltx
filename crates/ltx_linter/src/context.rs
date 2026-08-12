//! Shared context handed to every lint rule.

use std::sync::Arc;

use ltx_diagnostics::{LtxFileId, LtxSourceMap, LtxSpan};
use ltx_parser::ast::Document;

/// Everything a rule needs to inspect and report on a parsed document.
///
/// `'doc` is the lifetime of the borrow of the [`Document`], `'src` is the
/// lifetime of the source text the document (and any rule slices) borrows from.
pub struct LintContext<'doc, 'src> {
    /// The parsed AST.
    pub document: &'doc Document<'src>,
    /// The source map used to resolve diagnostic spans.
    pub source_map: Arc<LtxSourceMap>,
    /// The raw, unmodified source text (mirrors the source map's file).
    pub raw_source: &'src str,
}

impl<'doc, 'src> LintContext<'doc, 'src> {
    /// Creates a new lint context.
    ///
    /// `raw_source` must be the same text that `document` was parsed from.
    #[must_use]
    pub const fn new(
        document: &'doc Document<'src>,
        source_map: Arc<LtxSourceMap>,
        raw_source: &'src str,
    ) -> Self {
        Self {
            document,
            source_map,
            raw_source,
        }
    }

    /// The file id the document was parsed from.
    #[inline]
    #[must_use]
    pub const fn file_id(&self) -> LtxFileId {
        self.document.span.file_id
    }

    /// Byte offset at which the given 1-based line starts.
    ///
    /// Returns `0` for line numbers that don't exist or when the source map
    /// has no file registered for this document.
    #[inline]
    #[must_use]
    pub fn line_start(&self, line_no: u32) -> usize {
        let Some(file) = self.source_map.get_file(self.file_id()) else {
            return 0;
        };
        file.line_starts()
            .get(line_no as usize - 1)
            .copied()
            .unwrap_or(0)
    }

    /// Builds a span within this document's file.
    #[inline]
    #[must_use]
    pub const fn span(&self, start: usize, end: usize) -> LtxSpan {
        LtxSpan::new(start, end, self.file_id())
    }

    /// Iterates over the lines of the raw source as `(1-based line number, text)`.
    ///
    /// The text excludes the trailing newline (`\r` is kept for CRLF files).
    pub fn lines(&self) -> impl Iterator<Item = (u32, &'src str)> {
        self.raw_source
            .split('\n')
            .enumerate()
            .map(|(i, line)| ((i + 1).try_into().unwrap_or(u32::MAX), line))
    }
}
