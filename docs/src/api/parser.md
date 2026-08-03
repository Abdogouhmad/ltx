# ltx_parser

**Recursive-descent parser for LaTeX.** Consumes a `ltx_lexer::TokenStream`
and produces an AST.

## Responsibilities

- syntax parsing and AST generation
- parser-specific diagnostics (owned by this crate)

## Architecture

- `parser::LtxParser` wraps a `TokenStream` and exposes cursor methods
  (`peek`, `bump`, `checkpoint`/`rewind`, `skip_ws`).
- `parser_traits::Parse` is the trait every AST node implements.
- `ast` contains the node types: `Document`, `Command`, `Environment`,
  `Math`, `Group`, `Text`, `Comment`, `UsePackage`, `DocumentClassDecl`, `Arg`, …
- `parse_document` is the top-level convenience entry point.

## AST nodes

| Node | Description |
|------|-------------|
| `Document` | Top-level root containing preamble and body. |
| `PreambleItem` | Preamble items (`\documentclass`, `\usepackage`, …). |
| `DocumentClassDecl` | `\documentclass` declaration. |
| `UsePackage` | `\usepackage` declaration. |
| `Command` | Control sequence with its arguments. |
| `Arg` / `OptionalArg` | Required and optional argument variants. |
| `Environment` | `\begin{...}...\end{...}` block. |
| `Group` | Balanced `{...}` group. |
| `Math` | Math expression. |
| `Text` | Plain text run. |
| `Comment` | LaTeX comment. |

## Error ownership

`ParserError` (in `src/error.rs`) owns all 6 parser diagnostics
(`LTX::PARSER::E001`–`E006`):

| Code | Variant |
|------|---------|
| `E001` | `ExpectedToken` |
| `E002` | `UnexpectedEOF` |
| `E003` | `UnclosedEnvironment` |
| `E004` | `MismatchedEnvironment` |
| `E005` | `MissingClosingBrace` |
| `E006` | `UnexpectedEOFWhileParsing` |

`ParserErrorHandler` collects these. When `LtxParser::new` is constructed it
drains the lexer's diagnostics into the same handler, so a single sink
reports both phases. See the [Parser Errors](../errors/parser.md) table.

## Usage

```rust
use ltx_parser::{LtxParser, parse_document};

let mut parser = LtxParser::new(stream);
let doc = parse_document(&mut parser);

let handler = parser.error_handler_mut();
if handler.has_errors() {
    eprintln!("{}", handler.render_pretty());
}
```

## Design notes

- Zero token clones: AST nodes store `Range<usize>` token spans or
  zero-copy `&'src str` slices.
- Diagnostics flow through `ltx_diagnostics` for rendering — the parser owns
  the errors, the diagnostics crate renders them.
