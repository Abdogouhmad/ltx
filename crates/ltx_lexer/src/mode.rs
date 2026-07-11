//! Lexer operating modes.

/// Lexer mode — determines which catcode dispatch table is active.
///
/// The lexer starts in [`Normal`] mode and switches to [`Math`] when it
/// encounters a `$` or `$$` token, back to [`Normal`] on the matching close.
#[derive(Debug, Clone, PartialEq, Copy, Eq, Default)]
pub enum LtxMode {
    /// Normal text mode (default).
    #[default]
    Normal,
    /// Math mode (`$…$` or `$$…$$`).
    Math,
}
