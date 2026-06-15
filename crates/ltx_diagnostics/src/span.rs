use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a span of text in a file.
pub struct Span {
    /// The start offset of the span in the file.
    pub start: usize,
    /// The end offset of the span in the file.
    pub end: usize,
    /// The file that the span belongs to.
    pub file: Arc<str>,
}

impl Span {
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
    pub const fn new(start: usize, end: usize, file: Arc<str>) -> Self {
        Self { start, end, file }
    }

    /// Returns the length of the span.
    ///
    /// # Returns
    ///
    /// The length of the span.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns the start offset of the span.
    ///
    /// # Returns
    ///
    /// The start offset of the span.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the end offset of the span.
    ///
    /// # Returns
    ///
    /// The end offset of the span.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Returns the file that the span belongs to.
    ///
    /// # Returns
    ///
    /// The file that the span belongs to.
    #[must_use]
    pub const fn file(&self) -> &Arc<str> {
        &self.file
    }

    /// Returns whether the span is empty (i.e. `start` equals `end`).
    ///
    /// # Returns
    ///
    /// `true` if the span is empty, `false` otherwise.
    #[must_use]
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
    pub fn merge(&self, other: &Self) -> Self {
        debug_assert_eq!(self.file, other.file);
        Self::new(
            self.start.min(other.start),
            self.end.max(other.end),
            self.file.clone(),
        )
    }
}

impl From<Span> for miette::SourceSpan {
    fn from(s: Span) -> Self {
        // miette expects (offset, length)
        (s.start, s.len()).into()
    }
}
