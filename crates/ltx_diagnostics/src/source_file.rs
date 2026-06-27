//! This handles loading files, storing them, and doing `the byte-offset → line:column` math that compilers need.
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

use crate::span::LtxFileId;

use std::sync::Arc;

/// Represents a loaded source file with precomputed line starts.
#[derive(Debug, Clone)]
pub struct LtxSourceFile {
    /// A unique identifier for this file.
    pub id: LtxFileId,
    /// The path to the file on disk.
    pub path: PathBuf,
    /// The contents of the file as a string.
    pub source: Arc<str>,
    /// Byte offsets of the start of each line. Line 1 always starts at byte 0.
    pub line_starts: Vec<usize>,
    /// A precomputed `NamedSource` for use with Miette.
    pub named_source: miette::NamedSource<Arc<str>>,
}

/// Stores all loaded source files and provides span-to-line_column mapping.
#[derive(Debug, Clone)]
pub struct LtxSourceMap {
    /// The loaded source files.
    pub files: Vec<LtxSourceFile>,
    /// A mapping from file paths to their `FileId`.
    pub path_to_id: HashMap<PathBuf, LtxFileId>,
    /// The next available `FileId` to assign to a new file.
    pub next_id: u32,
}

impl LtxSourceMap {
    /// Creates a new, empty source map.
    ///
    /// # Returns
    ///
    /// A new `SourceMap` with no files loaded.
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

    /// Returns the file with the given ID, if it exists.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the file to retrieve.
    ///
    /// # Returns
    ///
    /// The file with the given ID, if it exists.
    #[inline]
    #[must_use]
    pub fn get_file(&self, id: LtxFileId) -> Option<&LtxSourceFile> {
        self.files.get(id.0 as usize)
    }

    /// Convert a byte offset into a 1-indexed (line, column) tuple.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to convert the offset for.
    /// * `byte_offset` - The byte offset to convert.
    ///
    /// # Returns
    ///
    /// The 1-indexed (line, column) tuple corresponding to the given offset, if it is valid.
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
}

/// A default implementation of [`LtxSourceMap`] that uses [`LtxSourceMap::new`].
impl Default for LtxSourceMap {
    fn default() -> Self {
        Self::new()
    }
}
