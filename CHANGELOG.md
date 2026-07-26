# Changelog

All notable changes to LTX will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Note:** LTX is pre-1.0. Minor versions may contain breaking changes.

## [Unreleased]

### Safety & Correctness

- Added `// SAFETY:` annotations and `debug_assert!` guards to the `unsafe` block in `ltx_parser/src/ast/arg.rs`.
- Replaced `panic!()` in `ltx_cli/src/cli.rs` with `miette::miette!()` error propagation.
- Added `#[non_exhaustive]` to `LtxError` for forward-compatible error variant additions.
- Changed `LtxSourceFile::path()` return type from `&PathBuf` to `&Path`.
- Fixed inaccurate `# Panics` doc on `LtxDiagnosticSink::drain_sorted()`.

### API Improvements

- Changed `LtxDiagnosticSink::get_by_severity()` to return `impl Iterator` instead of `Vec` (zero-allocation filtering).
- Changed `LtxDiagnosticSink::render_pretty()` to return `Result<String, fmt::Error>` instead of silently discarding errors.
- Added `Display` impls for `LtxTokenKind` and `MathDelimiter` for human-readable output.
- Added `Hash` derive to `LtxCatCode` for use as `HashMap` key.
- Made `LtxParser::env_stack` `pub(crate)` to prevent external misuse.
- Replaced `bool` fields in `ScaffoldOptions` with `SrcLayout` / `BibLayout` enums.
- Added `LtxManifest::from_file()` with `Deserialize` support on all config types.

### CLI

- Implemented full `check` command pipeline: source → lexer → parser → diagnostics rendering.
- Defined `CliError` enum with `thiserror` and structured error variants.
- Added exit code constants: `SUCCESS` (0), `ERROR` (1), `FILE_NOT_FOUND` (2), `INVALID_INPUT` (3), `DIAGNOSTICS_FOUND` (4).
- Changed all informational CLI output from `println!` to `eprintln!`.
- Added `ltx_lexer` and `ltx_parser` dependencies to `ltx_cli`.

### Tests (111 total)

- **ltx_utils (7):** `create_dir`, `create_file`, `write_file`, `resolve_main_file` edge cases.
- **ltx_diagnostics (12):** `line_col`, `span` merge/len/is_empty, `drain_sorted` ordering, `render_json_into`, source map operations.
- **ltx_config (8):** scaffold flat/src/bib layouts, manifest TOML roundtrip, `CompilerEngine` parsing, `Engine` construction.
- **ltx_lexer (25):** catcode lookups, tokenization, mode dispatch (`$`/`$$`), env scanning (`\begin`/`\end`/`\documentclass`), `TokenStream` cursor API (peek/bump/checkpoint/rewind/skip_ws/at_eof).
- **ltx_parser (22):** document structure, preamble items, commands with args, environments (nested/unclosed), math modes, text/comment nodes.
- **ltx_cli (10):** exit codes, `check` command valid/invalid/missing, `CliError` display, clap argument parsing.

### Performance

- Pre-allocated `Vec` capacity in `compute_line_starts()` based on source length.
- Added criterion benchmarks: lexer throughput (small/medium/large sources), parser full-document parse, stream creation.

### Code Quality

- Fixed 20+ clippy warnings: `unsafe` blocks, `unwrap_used`, `missing_docs`, `let...else`, `map_or_else`, wildcard matches, `const fn` promotion, redundant closures.
- Added `#![allow(clippy::expect_used, missing_docs)]` to all test/bench/example files.
- Added `cargo fmt --check` enforcement.

### Documentation

- Rewrote `docs/src/README.md` with project overview and architecture diagram.
- Completed `docs/src/guide/installation.md` with full install instructions.
- Added `docs/src/guide/cli.md` — CLI reference for all commands.
- Added `docs/src/guide/configuration.md` — `config.toml` format reference.
- Added `docs/src/api/overview.md` — API reference for all library crates.
- Created `CHANGELOG.md`.

### CI & Quality Gates

- Added GitHub Actions CI workflow with 6 parallel jobs: check, fmt, clippy, test, docs, deny.
- Added `cargo deny` check to justfile QA pipeline.
- Added `cargo doc --no-deps` verification to QA pipeline.
- Added `pre-commit` and `doc-check` justfile recipes.

---

## [0.1.0] — 2026-07-01

Initial release of the LTX workspace with six crates: `ltx_utils`, `ltx_diagnostics`, `ltx_lexer`, `ltx_parser`, `ltx_config`, `ltx_cli`.
