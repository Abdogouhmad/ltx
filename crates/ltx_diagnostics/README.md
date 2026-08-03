# ltx_diagnostics

**Pure diagnostic infrastructure for the LTX toolchain.** This crate defines **no
domain errors** — it only provides the building blocks other crates plug into
to report, render, and serialize diagnostics.

## Responsibilities

- source file management (`LtxSourceMap`, `LtxSourceFile`)
- byte-range span utilities (`LtxSpan`, `LtxFileId`)
- severity classification (`LtxSeverity`: Error / Warning / Hint)
- `miette` integration and rendering
- batch diagnostic reporting (`LtxDiagnosticSink`)
- the `LtxDiagnosticSource` helper trait that every crate's error enum implements
- the `ErrorCode` registry metadata struct

## Key types

| Type | Role |
|------|------|
| `LtxDiagnostic` | Wraps `Arc<dyn LtxDiagnosticSource>` + `Arc<LtxSourceMap>` so any phase error can be rendered. |
| `LtxDiagnosticSink` | Accumulates diagnostics across phases for batch reporting (never panics). |
| `LtxSourceMap` / `LtxSourceFile` | Source-text registry; byte-offset → line:column resolution. |
| `LtxSpan` / `LtxFileId` | Byte-range location in a specific file. |
| `LtxSeverity` | Error / Warning / Hint classification. |
| `ErrorCode` | Registry entry describing one diagnostic code (code, description, severity, phase). |
| `LtxDiagnosticSource` | Trait implemented by every phase-owned error; exposes its primary span. |

## Error ownership

Domain errors live in the crates that produce them, **not** here:

| Crate | Error enum | Codes |
|-------|-----------|-------|
| `ltx_lexer` | `LexerError` | `LTX::LEXER::E001`–`E011` |
| `ltx_parser` | `ParserError` | `LTX::PARSER::E001`–`E006` |
| `ltx_config` | `ConfigError` | `LTX::CONFIG::E001`–`E008` |
| `ltx_compiler` | `CompilerError` | `LTX::COMPILER::E001`–`E004`, `W001` |

Each of those crates exports a `pub const ALL_CODES: &[ErrorCode]` registry that
the CLI aggregates for `ltx code`.

## Rendering

- `render_pretty(diagnostic)` / `render_pretty_into(...)` — miette graphical output
- `render_json_into(sink, writer)` — JSON-serializable diagnostics for tooling

## Usage

```rust
use ltx_diagnostics::{LtxSourceMap, LtxDiagnosticSink, LtxDiagnostic, LtxDiagnosticSource};

let mut source_map = LtxSourceMap::new();
let file_id = source_map.add_file("main.tex")?;
// ... produce an error implementing LtxDiagnosticSource ...
let diagnostic = LtxDiagnostic::new(error, source_map.into());
```

## Design constraints (see `AGENT.md`)

- Never define domain errors here.
- Never depend on lexer / parser / config / compiler crates (keeps the graph acyclic).
