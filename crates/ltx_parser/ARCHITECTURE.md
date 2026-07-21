# ltx_parser — Architecture

## Dependency Map

```
┌──────────────────────────────────────────────────────────────┐
│                         ltx_parser                            │
│                                                               │
│  ┌──────────────┐    ┌──────────────┐    ┌────────────────┐  │
│  │  LtxParser   │◄───│ Parse trait  │───►│  AST nodes     │  │
│  │  (parser.rs) │    │(parser_      │    │(ast/*.rs)      │  │
│  │              │    │ traits.rs)   │    │                │  │
│  │  • new()     │    │              │    │  • Text        │  │
│  │  • peek_kind │    │  fn parse(   │    │  • Command     │  │
│  │  • bump      │    │    parser:   │    │  • Group       │  │
│  │  • accept    │    │    &mut      │    │                │  │
│  │  • expect    │    │    LtxParser │    │  (each impl    │  │
│  │  • parse<T>  │    │  ) -> Self   │    │   Parse)       │  │
│  │  • skip_ws   │    │              │    │                │  │
│  │  • checkpoint│    └──────┬───────┘    └────────┬───────┘  │
│  │  • rewind    │           │                     │          │
│  └──────┬───────┘           │                     │          │
│         │                   │                     │          │
│         │   ┌───────────────┴──────────────┐      │          │
│         └──►│        TokenStream           │◄─────┘          │
│             │        (ltx_lexer)           │                 │
│             │  • peek / peek_kind          │                 │
│             │  • bump / checkpoint / rewind│                 │
│             │  • skip_ws / at_eof          │                 │
│             │  • error_stream(_mut)?       │                 │
│             └──────────────────────────────┘                 │
└──────────────────────────────────────────────────────────────┘
```

## Crate dependencies

```
ltx_parser
 ├── ltx_lexer        (TokenStream, LtxToken, LtxTokenKind)
 └── ltx_diagnostics  (LtxSpan)
```

## File layout

```
src/
├── lib.rs            re-exports LtxParser and Parse
├── parser.rs         LtxParser struct + all methods
├── parser_traits.rs  Parse trait definition
└── ast/
    ├── mod.rs        re-exports Text, Command, Group
    ├── text.rs       Parse impl for plain text
    ├── command.rs    Parse impl for \controlsequence + args
    └── group.rs      Parse impl for { … } balanced groups
examples/
└── parser_example.rs demo: lex → token-stream → parse Text
```

## How to add a new AST node

1. Create `src/ast/your_type.rs`
2. Define a struct with `pub span: LtxSpan` plus your fields
3. `impl<'src> Parse<'src> for YourType<'src>` — call `parser.expect()` / `parser.parse::<Child>()`
4. Re-export in `src/ast/mod.rs`
