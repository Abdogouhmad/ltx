//! Lexer operating modes.

/// Lexer mode — determines which catcode dispatch table is active.
///
/// The lexer starts in [`LtxMode::Normal`] and switches to [`LtxMode::Math`] when it
/// encounters a `$` or `$$` token, back to [`LtxMode::Normal`] on the matching close.
#[derive(Debug, Clone, PartialEq, Copy, Eq, Default)]
pub enum LtxMode {
    /// Normal text mode (default).
    #[default]
    Normal,
    /// Math mode (`$…$` or `$$…$$`).
    Math,
}
