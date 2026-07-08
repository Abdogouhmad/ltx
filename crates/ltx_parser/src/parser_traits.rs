//! shared behavior within my parser

use crate::parser::LtxParser;

/// Shared trait for types that can be parsed from a token stream.
pub trait Parse<'src>: Sized {
    /// Parse an instance of `Self` from the parser, advancing the stream.
    fn parse(parser: &mut LtxParser<'src>) -> Self;
}
