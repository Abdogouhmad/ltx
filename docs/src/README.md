# LTX

An extremely fast LaTeX project manager, written in Rust.

LTX resolves your document graph, drives `pdflatex`/`xelatex`/`lualatex` and `biber`/`bibtex` in the right order, and gets out of your way — built for people who'd rather be writing than babysitting a build.

> **Status:** pre-1.0. The CLI surface is stabilizing; expect minor breaking changes between minor versions until `1.0`.

---

## Why LTX?

Most LaTeX workflows are stitched together from shell scripts, `latexmk`, and muscle memory. LTX treats your document the way a modern build tool treats a codebase:

- **Correct by construction** — dependency-aware compilation means bibliographies, indices, and multi-pass cross-references resolve without manual re-runs.
- **Fast** — incremental builds, parallel auxiliary passes, and a Rust core mean large multi-file theses and papers compile quickly.
- **Predictable** — one `config.toml` describes your project; builds behave the same on your machine, your co-author's machine, and in CI.

## Architecture

LTX is a Rust workspace composed of six crates:

| Crate | Role |
|-------|------|
| **ltx_utils** | Low-level filesystem helpers (`create_dir`, `write_file`, `resolve_main_file`) |
| **ltx_diagnostics** | Unified diagnostic infrastructure — error codes, spans, source maps, rendering |
| **ltx_lexer** | Byte-level tokenizer — converts `.tex` source into a stream of typed tokens |
| **ltx_parser** | Recursive-descent parser — consumes the token stream and produces an AST |
| **ltx_config** | Configuration and scaffolding — reads `config.toml`, generates project layouts |
| **ltx_cli** | Binary entry point — subcommand dispatch, user-facing output |

## The Pipeline

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
  LtxDiagnosticSink   ──► Errors / Warnings / Hints
```

1. **Lexer** scans raw bytes, applies TeX catcode rules, and emits `LtxToken`s with source spans.
2. **Parser** consumes the token stream via `TokenStream` (with `peek`/`bump`/`checkpoint`/`rewind`), builds the AST, and collects diagnostics through a shared `LexerErrorHandler`.
3. **Diagnostics** are accumulated in an `LtxDiagnosticSink` and rendered with `miette` for rich terminal output or serialized to JSON.

## Quick start

```bash
# Scaffold a new project
ltx new my-paper
cd my-paper

# Check for errors
ltx check main.tex

# List diagnostic codes
ltx code
```

## Documentation pages

- [Installation](guide/installation.md) — prerequisites, cargo install, from-source
- [CLI Usage](guide/cli.md) — subcommands, flags, examples
- [Configuration](guide/configuration.md) — `config.toml` reference, scaffolding options
- [API Overview](api/overview.md) — library crate reference for Rust consumers
- [Lexer Errors](errors/lexer.md) — E0xx error codes
- [Parser Errors](errors/parser.md) — E1xx error codes
