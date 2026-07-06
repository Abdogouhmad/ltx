//! The mode of the latex {Normal, Math, Verbatim}

/// The mode of the latex {Normal, Math, Verbatim}
#[derive(Debug, Clone, PartialEq, Copy, Eq, Default)]
pub enum LtxMode {
    /// Normal mode
    #[default]
    Normal,
    /// Math mode
    Math,
    //  Verbatim mode
    // Verbatim,
}
