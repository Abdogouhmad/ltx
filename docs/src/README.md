# LTX

An extremely fast LaTeX project manager, written in Rust.

LTX scaffolds structured projects, lints your source, drives `tectonic`
(and `pdflatex`/`xelatex`/`lualatex`), and gets out of your way — built for
people who'd rather be writing than babysitting a build.

> **Status:** pre-1.0. The CLI surface is stabilizing; expect minor breaking
> changes between minor versions until `1.0`.

---

## Why LTX?

Most LaTeX workflows are stitched together from shell scripts, `latexmk`, and
muscle memory. LTX treats your document the way a modern build tool treats a
codebase:

- **Correct by construction** — one `ltx.toml` describes your project; builds
  behave the same on your machine, your co-author's machine, and in CI.
- **Fast** — a Rust core and a native engine mean large multi-file papers
  compile quickly, without a full TeX distribution installed.
- **Predictable** — the diagnostics pipeline gives you one consistent,
  source-spanning report for syntax and configuration problems.

## Architecture

LTX is a Rust workspace composed of seven crates. Each crate owns its domain,
and errors are defined in the crate that produces them — never centralized.

| Crate | Role |
|-------|------|
| **ltx_utils** | Low-level filesystem helpers (`create_dir`, `write_file`, `resolve_main_file`) |
| **ltx_diagnostics** | Pure diagnostic infrastructure — spans, source maps, `miette` rendering (defines **no** domain errors) |
| **ltx_lexer** | Byte-level tokenizer — converts `.tex` source into a stream of typed tokens |
| **ltx_parser** | Recursive-descent parser — consumes the token stream and produces an AST |
| **ltx_config** | Manifest model + validation + scaffolding — reads `ltx.toml`, generates project layouts |
| **ltx_compiler** | Compilation orchestration — dispatches to an engine (currently `tectonic`) |
| **ltx_cli** | Binary entry point — subcommand dispatch, user-facing output, exit codes |

## The pipeline

Source text flows through the toolchain in this order:

```
  .tex source
       │
       ▼
   LtxLexer          ──► TokenStream (LtxToken + LtxSpan)
       │
       ▼
   LtxParser          ──► AST (Document, Command, Environment, …)
       │
       ▼
   ParserErrorHandler ──► LtxDiagnosticSink (absorbs lexer + parser diagnostics)
       │
       ▼
   miette renderer    ──► Terminal output
```

1. **Lexer** scans raw bytes, applies TeX catcode rules, and emits `LtxToken`s
   with source spans. Problems are recorded by its own `LexerErrorHandler`.
2. **Parser** consumes the token stream via `TokenStream`
   (`peek`/`bump`/`checkpoint`/`rewind`), builds the AST, and reports through
   its own `ParserErrorHandler`, which absorbs the lexer's diagnostics on
   construction.
3. **Diagnostics** accumulate in an `LtxDiagnosticSink` and render with
   `miette` for rich terminal output or serialize to JSON.

## Error codes

Every diagnostic code is namespaced by the phase that owns it — 30 total:

| Namespace | Count | Range |
|-----------|-------|-------|
| `LTX::LEXER::E0xx` | 11 | Tokenization |
| `LTX::PARSER::E0xx` | 6 | Structural parsing |
| `LTX::CONFIG::E0xx` | 8 | Manifest / scaffolding |
| `LTX::COMPILER::E0xx` / `W0xx` | 5 | Compilation |

Run `ltx code` to list them, filtered by phase (`--lexer`, `--parser`,
`--config`, `--compiler`) or severity (`-e`, `-w`).

## Quick start

```bash
# Scaffold a new project
ltx new my-paper
cd my-paper

# Check for errors
ltx check main.tex

# Build a PDF (tectonic is bundled; no TeX distribution needed)
ltx build

# List diagnostic codes
ltx code
```

## Documentation pages

- [Installation](guide/installation.md) — prerequisites, cargo install, from-source
- [CLI Usage](guide/cli.md) — subcommands, flags, examples, exit codes
- [Configuration](guide/configuration.md) — `ltx.toml` reference, scaffolding options
- [API Overview](api/overview.md) — architecture + per-crate library reference
- [Lexer Errors](errors/lexer.md) — `LTX::LEXER::E0xx`
- [Parser Errors](errors/parser.md) — `LTX::PARSER::E0xx`
- [Config Errors](errors/config.md) — `LTX::CONFIG::E0xx`
- [Compiler Errors](errors/compiler.md) — `LTX::COMPILER::E0xx` / `W0xx`
