# API Overview

Reference for the LTX library crates, for Rust consumers embedding LTX as a
library. The workspace is eight crates; each owns its domain and its errors.
Errors flow through the shared `ltx_diagnostics` infrastructure to render,
but are **defined in the crate that produces them**.

## Crate map

| Crate | Responsibility | Errors owned |
|-------|----------------|--------------|
| [ltx_diagnostics](diagnostics.md) | Pure infrastructure — spans, source maps, sinks, `miette` rendering, `ErrorCode` registry | *none* (infra only) |
| [ltx_lexer](lexer.md) | Byte-level tokenizer; catcode handling | `LexerError` (`LTX::LEXER::E001`–`E011`) |
| [ltx_parser](parser.md) | Recursive-descent parser; AST construction | `ParserError` (`LTX::PARSER::E001`–`E006`) |
| [ltx_config](config.md) | `ltx.toml` model, validation, scaffolding | `ConfigError` (`LTX::CONFIG::E001`–`E008`) |
| [ltx_linter](linter.md) | Style lints; unified lex→parse→lint pipeline | `LintError` + remapped `LTX::LINTER::E001`–`E019`, `W001`–`W016` |
| [ltx_compiler](compiler.md) | Compilation orchestration; engine dispatch | `CompilerError` (`LTX::COMPILER::E001`–`E006`, `W001`) |
| [ltx_utils](utils.md) | Low-level filesystem helpers | *none* |
| ltx_cli | Binary entry point; subcommand dispatch | `CliError` (CLI-only) |

## Dependency direction

Dependencies point inward toward reusable infrastructure. `ltx_diagnostics`
never depends on the other crates, so the graph stays acyclic:

```
ltx_utils ──► ltx_config ──► ltx_compiler ──► ltx_cli
                    │  ▲            │
                    │  │            ▼
                    │  └── ltx_linter ──► ltx_diagnostics (shared rendering)
                    ▼              ▲
ltx_lexer ──► ltx_parser ──────────┘
```

## The diagnostic pipeline

Any error type implements [`LtxDiagnosticSource`](diagnostics.md) and is
wrapped in an [`LtxDiagnostic`](diagnostics.md) together with the
`LtxSourceMap` needed to render it. Diagnostics accumulate in an
`LtxDiagnosticSink`, then render with `miette` or serialize to JSON — the
diagnostics crate never needs to know which phase produced an error.

```
Lexer ──► LexerError ──┐
                       ├─► LtxDiagnostic ──► LtxDiagnosticSink ──► miette renderer
Parser ──► ParserError ┘                                  └─────► JSON
Linter ──► lint findings (W0xx)
```

## Code registry

Each crate exports a `pub const ALL_CODES: &[ErrorCode]` table. The CLI
aggregates all of them for `ltx code`:

| Namespace | Count | Severities |
|-----------|-------|------------|
| `LTX::LEXER::E0xx` | 11 | errors |
| `LTX::PARSER::E0xx` | 6 | errors |
| `LTX::CONFIG::E0xx` | 8 | errors |
| `LTX::COMPILER::E0xx`, `W0xx` | 7 | errors + 1 warning |
| `LTX::LINTER::E0xx` | 19 | errors |
| `LTX::LINTER::W0xx` | 16 | warnings |

The linter's `ALL_CODES` also owns the unified namespace: by default
`ltx code` shows the `LTX::LINTER::*` codes; the phase-local codes appear
only with their phase flag or `--all`.

See the [Error codes](../errors/lexer.md) section for full tables.
