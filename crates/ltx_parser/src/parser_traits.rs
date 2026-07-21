//! The [`Parse`] trait — shared interface for all AST node types.

use crate::parser::LtxParser;

/// A type that can be parsed from a token stream.
pub trait Parse<'src>: Sized {
    /// Parse an instance of `Self` from the parser, advancing the stream.
    fn parse(parser: &mut LtxParser<'src>) -> Self;
}
