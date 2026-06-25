//! The mode of the latex {Normal, Math, Verbatim}

/// The mode of the latex {Normal, Math, Verbatim}
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum LtxMode {
    /// Normal mode
    Normal,
    /// Math mode
    Math,
    /// Verbatim mode
    Verbatim,
}

impl Default for LtxMode {
    fn default() -> Self {
        Self::Normal
    }
}
