//! Source file storage and byte-offset → line:column resolution.
//!
//! [`LtxSourceMap`] is the central registry: every diagnostic holds an
//! `Arc<LtxSourceMap>` so that rendering can resolve a [`LtxSpan`] back
//! to human-readable line/column positions and source-text snippets.

use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use crate::span::LtxFileId;

/// A loaded source file with precomputed line-start offsets.
#[derive(Debug, Clone)]
pub struct LtxSourceFile {
    /// Unique identifier for this file within the [`LtxSourceMap`].
    id: LtxFileId,
    /// Path on disk.
    path: PathBuf,
    /// File contents.
    source: Arc<str>,
    /// Byte offset of the start of each line (line 1 → index 0).
    line_starts: Vec<usize>,
    /// Precomputed [`miette::NamedSource`] for diagnostic rendering.
    named_source: miette::NamedSource<Arc<str>>,
}

impl LtxSourceFile {
    /// Unique file identifier.
    #[must_use]
    #[inline]
    pub const fn id(&self) -> LtxFileId {
        self.id
    }

    /// Path on disk.
    #[must_use]
    #[inline]
    pub const fn path(&self) -> &PathBuf {
        &self.path
    }

    /// File contents as a shared string slice.
    #[must_use]
    #[inline]
    pub const fn source(&self) -> &Arc<str> {
        &self.source
    }

    /// Byte offsets of line starts.
    #[must_use]
    #[inline]
    pub fn line_starts(&self) -> &[usize] {
        &self.line_starts
    }

    /// Precomputed [`miette::NamedSource`] for diagnostic rendering.
    #[must_use]
    #[inline]
    pub const fn named_source(&self) -> &miette::NamedSource<Arc<str>> {
        &self.named_source
    }
}

/// Stores all loaded source files and provides span-to-line:column mapping.
#[derive(Debug, Clone)]
pub struct LtxSourceMap {
    files: Vec<LtxSourceFile>,
    path_to_id: HashMap<PathBuf, LtxFileId>,
    next_id: u32,
}

impl LtxSourceMap {
    /// Creates a new, empty source map.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            path_to_id: HashMap::new(),
            next_id: 0,
        }
    }

    /// Load a file from disk.
    pub fn add_file(&mut self, path: PathBuf) -> io::Result<LtxFileId> {
        let source = std::fs::read_to_string(&path)?;
        Ok(self.add_source(path, source))
    }

    /// Add a file directly from a string (useful for tests).
    pub fn add_inline(&mut self, name: impl Into<PathBuf>, source: impl Into<String>) -> LtxFileId {
        self.add_source(name.into(), source.into())
    }

    /// Returns the file with the given ID, if it exists.
    #[inline]
    #[must_use]
    pub fn get_file(&self, id: LtxFileId) -> Option<&LtxSourceFile> {
        self.files.get(id.0 as usize)
    }

    /// Number of loaded files.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Returns `true` if no files have been loaded.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Convert a byte offset into a 1-indexed `(line, column)` tuple.
    #[inline]
    #[must_use]
    pub fn line_col(&self, file_id: LtxFileId, byte_offset: usize) -> Option<(usize, usize)> {
        let file = self.get_file(file_id)?;
        let line_index = file
            .line_starts
            .partition_point(|&start| start <= byte_offset);

        if line_index == 0 {
            return None;
        }

        let line_index = line_index - 1;
        let line_start = file.line_starts[line_index];
        let col_offset = byte_offset - line_start;

        Some((line_index + 1, col_offset + 1))
    }

    // ── private ──────────────────────────────────────────────────────

    fn add_source(&mut self, path: PathBuf, source: String) -> LtxFileId {
        let id = LtxFileId(self.next_id);
        self.next_id += 1;

        let line_starts = Self::compute_line_starts(&source);
        let source: Arc<str> = source.into();
        let named_source = miette::NamedSource::new(path.display().to_string(), source.clone());

        self.files.push(LtxSourceFile {
            id,
            path: path.clone(),
            source,
            line_starts,
            named_source,
        });

        self.path_to_id.insert(path, id);
        id
    }

    fn compute_line_starts(source: &str) -> Vec<usize> {
        let mut starts = vec![0];
        let bytes = source.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            let b = bytes[i];
            if b == b'\r' {
                if i + 1 < len && bytes[i + 1] == b'\n' {
                    i += 2;
                } else {
                    i += 1;
                }
                starts.push(i);
            } else if b == b'\n' {
                i += 1;
                starts.push(i);
            } else {
                i += 1;
            }
        }
        starts
    }
}

impl Default for LtxSourceMap {
    fn default() -> Self {
        Self::new()
    }
}
