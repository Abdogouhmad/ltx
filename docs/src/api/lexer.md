# ltx_lexer

**Byte-level tokenizer for LaTeX source files.** Converts raw `.tex` text
into a stream of `LtxToken`s, each carrying an `LtxTokenKind`, an `LtxSpan`,
and the source slice it was parsed from. Mode-aware (`Normal` / `Math`) with
catcode state tracking per the TeX specification.

## Responsibilities

- lexical analysis and token generation
- category code handling (`LtxCatCode`, `LtxCatCodeState`)
- lexer-specific diagnostics (owned by this crate)

## Key types

| Type | Role |
|------|------|
| `LtxLexer` | Streaming iterator — `.next()` / `.next_token()` yields one token at a time. |
| `TokenStream` | Eagerly drains `LtxLexer`; cursor API (`peek`, `bump`, `checkpoint`/`rewind`) for the parser. |
| `LtxToken` / `LtxTokenKind` | A single token and its classification. |
| `LtxMode` | Normal or Math operating mode. |
| `LtxCatCode` / `LtxCatCodeState` | TeX category codes and the current lookup table. |
| `LexerError` | All 11 lexer diagnostics (`LTX::LEXER::E001`–`E011`). |
| `LexerErrorHandler` | Collects `LtxDiagnostic`s during lexing; wraps them with the source map. |

## Error ownership

`LexerError` (in `src/error.rs`) owns every lexer diagnostic: unexpected
token, unexpected EOF, unmatched brace, invalid math delimiter, unterminated
argument, invalid escape sequence, invalid unicode, illegal parameter char,
unterminated verbatim, invalid character, and mismatched environment. Codes
are registered in `ALL_CODES` under the `LTX::LEXER::E0xx` namespace. See the
[Lexer Errors](../errors/lexer.md) table.

## Usage

```rust
use ltx_lexer::{LtxLexer, TokenStream};
use ltx_diagnostics::LtxSourceMap;

let mut source_map = LtxSourceMap::new();
let file_id = source_map.add_file("main.tex")?;
let stream = TokenStream::new(LtxLexer::new(source, file_id, source_map.into()));

// Hand the stream to the parser, or drive LtxLexer directly.
```

## Design notes

- Zero string allocations for tokens — tokens borrow from the source text.
- Most consumers create a `TokenStream` and pass it to `ltx_parser`; only
  streaming consumers need `LtxLexer` directly.
