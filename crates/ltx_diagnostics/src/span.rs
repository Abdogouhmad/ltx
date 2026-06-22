use std::fmt;

/// A unique identifier for a source file in the `LtxSourceMap`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LtxFileId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a span of text in a file.
pub struct LtxSpan {
    /// The start offset of the span in the file.
    pub start: usize,
    /// The end offset of the span in the file.
    pub end: usize,
    /// The file that the span belongs to.
    pub file_id: LtxFileId,
}

impl LtxSpan {
    /// Creates a new `Span` with the given start, end, and file.
    ///
    /// # Arguments
    ///
    /// * `start` - The start offset of the span in the file.
    /// * `end` - The end offset of the span in the file.
    /// * `file` - The file that the span belongs to.
    ///
    /// # Returns
    ///
    /// A new `Span` with the given start, end, and file.
    #[must_use]
    #[inline]
    pub const fn new(start: usize, end: usize, file_id: LtxFileId) -> Self {
        Self {
            start,
            end,
            file_id,
        }
    }

    /// Returns the length of the span.
    ///
    /// # Returns
    ///
    /// The length of the span.
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns the start offset of the span.
    ///
    /// # Returns
    ///
    /// The start offset of the span.
    #[must_use]
    #[inline]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the end offset of the span.
    ///
    /// # Returns
    ///
    /// The end offset of the span.
    #[must_use]
    #[inline]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Returns whether the span is empty (i.e. `start` equals `end`).
    ///
    /// # Returns
    ///
    /// `true` if the span is empty, `false` otherwise.
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Extend this span to cover another (must be same file).
    ///
    /// # Arguments
    ///
    /// * `other` - The other span to merge with.
    ///
    /// # Returns
    ///
    /// A new `Span` that covers both spans.
    #[must_use]
    #[inline]
    pub fn merge(&self, other: &Self) -> Self {
        debug_assert_eq!(
            self.file_id, other.file_id,
            "Cannot merge spans from different files"
        );
        Self::new(
            self.start.min(other.start),
            self.end.max(other.end),
            self.file_id,
        )
    }
}

impl From<LtxSpan> for miette::SourceSpan {
    fn from(s: LtxSpan) -> Self {
        // miette expects (offset, length)
        (s.start, s.len()).into()
    }
}

impl fmt::Display for LtxFileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileId({})", self.0)
    }
}
