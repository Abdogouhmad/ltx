//! The main parser struct — wraps a [`TokenStream`] and drives [`Parse`] impls.

use crate::ast::Document;
use crate::parser_traits::Parse;
use ltx_diagnostics::LtxSpan;
use ltx_lexer::{LtxToken, LtxTokenKind, TokenStream};

/// Parser state machine over a fully-tokenized source.
///
/// Wraps a [`TokenStream`] (not `Peekable`) because the stream already
/// provides lookahead via `peek()`/`peek_at()`, backtracking via
/// `checkpoint()`/`rewind()`, and trivia-skipping via `skip_ws()`.
pub struct LtxParser<'src> {
    /// The underlying token stream.
    pub stream: TokenStream<'src>,

    /// Parser level env stack.
    pub env_stack: Vec<(&'src str, LtxSpan)>,
}

// ===== Convenience helpers that reduce boilerplate in `Parse` impls =====
impl<'src> LtxParser<'src> {
    /// Create a new parser that drains the given `TokenStream`.
    #[inline]
    #[must_use]
    pub const fn new(stream: TokenStream<'src>) -> Self {
        Self {
            stream,
            env_stack: Vec::new(),
        }
    }

    /// Convenience: parse any `T: Parse` from the current position.
    #[inline]
    pub fn parse<T: Parse<'src>>(&mut self) -> T {
        T::parse(self)
    }

    /// If the current token satisfies the predicate `f`, consume it and return `true`.
    #[inline]
    pub fn accept(&mut self, f: impl FnOnce(&LtxTokenKind<'src>) -> bool) -> bool {
        if self.peek_kind().is_some_and(f) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Expect the current token to satisfy `f`, consume it, and return it.
    ///
    /// On mismatch: emits an `LTX::E001` diagnostic and skips forward until a boundary.
    pub fn expect(
        &mut self,
        ctx: &str,
        f: impl FnOnce(&LtxTokenKind<'src>) -> bool,
    ) -> Option<&LtxToken<'src>> {
        if self.peek_kind().is_some_and(f) {
            self.bump()
        } else {
            let (found, span_start, span_end) = match self.stream.peek() {
                Some(tok) => {
                    let name = format!("{:?}", tok.kind);
                    (name, tok.span.start(), tok.span.end())
                }
                None => (
                    "EOF".to_string(),
                    self.stream.checkpoint(),
                    self.stream.checkpoint(),
                ),
            };
            self.error_handler_mut()
                .expected_token(ctx, &found, span_start, span_end);
            self.skip_to_boundary();
            None
        }
    }

    /// Skip tokens until a recovery point: `Command`, `GroupStart`, `GroupEnd`, `BeginEnv`, `EndEnv`, or `EOF`.
    pub fn skip_to_boundary(&mut self) {
        loop {
            match self.peek_kind() {
                None
                | Some(
                    LtxTokenKind::Command(_)
                    | LtxTokenKind::GroupStart
                    | LtxTokenKind::GroupEnd
                    | LtxTokenKind::BeginEnv(_)
                    | LtxTokenKind::EndEnv(_),
                ) => break,
                _ => {
                    self.bump();
                }
            }
        }
    }
}

/// Convenience entry point to parse a complete LaTeX [`Document`].
#[inline]
pub fn parse_document<'src>(parser: &mut LtxParser<'src>) -> Document<'src> {
    parser.parse::<Document<'src>>()
}
