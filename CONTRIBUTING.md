# Contributing to ltx

First off, thanks for taking the time to contribute.

## Table of contents

- [Project overview](#project-overview)
- [Prerequisites](#prerequisites)
- [Quick start](#quick-start)
- [Project architecture](#project-architecture)
- [Development workflow](#development-workflow)
- [Coding standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull request process](#pull-request-process)
- [CI / CD](#ci--cd)

## Project overview

`ltx` is a fast, opinionated LaTeX toolchain written in Rust. It scaffolds projects, lints source files, compiles with multiple engines (Tectonic, pdfLaTeX, XeLaTeX, LuaLaTeX), and watches for changes.

The pipeline is: `.tex` source → `LtxLexer` → `TokenStream` → `LtxParser` → AST → diagnostics/compilation.

## Prerequisites

- **Rust** — MSRV 1.86 (stable toolchain). Install via [rustup](https://rustup.rs/).
- **A TeX distribution** (optional) — only needed if you plan to use `pdflatex`/`xelatex`/`lualatex` instead of Tectonic. [TeX Live](https://tug.org/texlive/) recommended.
- **mold** (optional, Linux) — faster linking. `apt install mold` or equivalent.
- **cargo-deny** (optional) — license/advisory checking. `cargo install cargo-deny`.
- **just** (optional) — command runner. `cargo install just`.

## Quick start

```bash
git clone https://github.com/Abdogouhmad/ltx.git
cd ltx
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Project architecture

The workspace contains seven crates under `crates/`:

| Crate | Responsibility |
|---|---|
| `ltx_cli` | Binary entrypoint + CLI commands (`new`, `check`, `code`, `clean`) |
| `ltx_config` | `ltx.toml` data model, project scaffolding |
| `ltx_compiler` | Compilation pipeline (Tectonic integration, engine dispatch) |
| `ltx_diagnostics` | Error types, source maps, diagnostic rendering (miette + JSON) |
| `ltx_lexer` | Zero-copy byte-level TeX tokenizer |
| `ltx_parser` | Recursive-descent AST parser |
| `ltx_utils` | Low-level filesystem helpers |

### Data flow

```
.tex source
    ↓
LtxLexer ── emits LtxToken<'src> (zero-copy, borrows from source)
    ↓
TokenStream ── eagerly materializes tokens, cursor API (peek/bump/checkpoint)
    ↓
LtxParser ── drives Parse trait on AST nodes, recursive-descent
    ↓
LtxDiagnosticSink ── renders via miette (pretty) or JSON
    ↓
CompilerConfig ── dispatches to Tectonic / pdflatex / xelatex / lualatex
```

Key design decisions:
- **Zero-copy** — tokens borrow from source, AST stores token index ranges.
- **Single diagnostic sink** — shared between lexer and parser via `&mut LtxDiagnosticSink`.
- **Error recovery** — parser continues after errors using `skip_to_boundary()`.
- **Eager token stream** — simplifies parser implementation, enables checkpoint/rewind.
- **Tectonic-first** — default engine is the self-contained Rust TeX engine Tectonic; traditional engines are equally supported.

## Development workflow

### Justfile

The project uses [`just`](https://github.com/casey/just) as a command runner. Run `just` to list all commands:

| Command | Alias | Action |
|---|---|---|
| `just build` | `b` | `cargo build --workspace` |
| `just test` | `t` | `cargo test --workspace` |
| `just test-crate <name>` | | Test a single crate with output |
| `just fmt` | `f` | `cargo fmt --all` |
| `just fmtck` | | Check formatting only |
| `just clippy` | | `cargo clippy --workspace --all-targets` |
| `just check` | `c` | `cargo check --workspace --all-targets` |
| `just deny` | | `cargo deny check` |
| `just doc-check` | | `cargo doc --workspace --no-deps` |
| `just qa` | | Full quality gate (fmtck + clippy + test + deny + doc-check) |
| `just doc` | `d` | Build and open docs |
| `just examples` | | Run all crate examples |
| `just release` | | Build release binary |

### Cargo aliases

Defined in `.cargo/config.toml`:

```bash
cargo qa      # full quality check
cargo ch      # cargo check --all-targets
cargo t       # cargo test --workspace
cargo fmtck   # cargo fmt --all -- --check
cargo docs    # cargo doc --workspace --no-deps
```

### Profile selection

| Profile | Use case |
|---|---|
| `dev` (default) | Fast iterative development |
| `dev-opt` | `cargo build --profile dev-opt` — opt-level 1 for slightly faster dev binaries |
| `release` | `cargo build --release` — LTO + strip + panic=abort |
| `profiling` | Release with debuginfo for perf/samply |

## Coding standards

### Formatting

- 100-character line width.
- 4-space tabs.
- Imports and modules are reordered.

Run `cargo fmt --all` before every commit.

### Linting

The workspace enables strict lints in `Cargo.toml`:

- `unsafe_code` = warn
- `unreachable_pub` = warn
- `missing_docs` = warn (every public item must be documented)
- `dbg_macro` = deny
- `wildcard_imports` = deny
- `clippy::pedantic` + `clippy::nursery` = warn
- `unwrap_used` = warn, `expect_used` = warn
- `print_stdout` = warn (use `owo-colors` + `eprintln!` in CLI layer)

Run `cargo clippy --workspace --all-targets -- -D warnings` before every commit.

### Documentation

- Every public item (struct, field, fn, enum, module) must have a doc comment.
- Use `# Panics`, `# Errors`, `# Safety` sections where applicable.
- Use backtick-quoted identifiers: `Self`, `Option<T>`, `clone()`.
- Start with a single-line summary ending with a period.
- Run `cargo doc --workspace --no-deps` to verify docs build cleanly.

### Style guidelines

- Keep functions under ~30 lines; extract helpers.
- Prefer pattern matching over `if let`/`unwrap` chains.
- Use `?` for error propagation; avoid `unwrap()` in library code.
- Use enums over boolean flags (`enum Direction { Forward, Backward }` not `forward: bool`).
- Prefer `&[T]` / `&str` slices over owned types in function signatures.
- Name functions after *what* they do, not *how*.
- Avoid `Box<dyn Error>` in public APIs — use concrete error types with `thiserror`.
- Every `unsafe` block must have a `// SAFETY:` comment.

## Testing

### Unit tests

Embed tests in `#[cfg(test)] mod tests` blocks within each source file.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_behavior() {
        // ...
    }
}
```

### Integration tests

Integration tests live in `tests/` directories inside each crate:

| Crate | Test files |
|---|---|
| `ltx_cli` | `tests/cli_tests.rs` |
| `ltx_config` | `tests/config_tests.rs` |
| `ltx_lexer` | `tests/lexer_tests.rs`, `tests/catcode_test.rs`, `tests/lexer_utils_test.rs` |
| `ltx_parser` | `tests/parser_tests.rs`, `tests/parser_ast_tests.rs` |
| `ltx_diagnostics` | `tests/diagnostics_tests.rs`, `tests/daig_t.rs` |
| `ltx_utils` | `tests/utils_tests.rs` |

### Running tests

```bash
cargo test --workspace              # all tests
cargo test -p ltx_lexer             # single crate
cargo test -p ltx_lexer -- --nocapture  # with output
```

### Property-based testing

Use `proptest` for property-based tests, especially on serialization, parsing, and algorithm correctness.

### Benchmarking

Micro-benchmarks use `criterion`:

```bash
cargo bench -p ltx_lexer
cargo bench -p ltx_parser
```

Benchmarks are in `benches/` directories with `html_reports` enabled.

## Documentation

The project uses [mdBook](https://rust-lang.github.io/mdBook/) for user-facing docs:

```bash
# Install mdBook
cargo install mdbook

# Serve locally
cd docs && mdbook serve --open
```

Documentation source lives in `docs/src/`. The generated book is at `docs/book/`.

API documentation is generated from doc comments:

```bash
cargo doc --workspace --open
```

## Pull request process

1. **Open an issue first** for significant changes so we can align on direction.
2. Create a feature branch from `main`.
3. Make your changes, following the [coding standards](#coding-standards).
4. Run the full QA gate before committing:
   ```bash
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   cargo doc --workspace --no-deps
   ```
   Or with just: `just qa`.
5. Write tests for new functionality.
6. Update documentation if your change affects the public API or user-facing behavior.
7. Keep PRs focused — one logical change per PR.
8. Update `CHANGELOG.md` under the `## [Unreleased]` section.

## CI / CD

### CI (`.github/workflows/ci.yml`)

Runs on every push/PR to `main`:
- `cargo check`
- `cargo fmt -- --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo doc -D warnings`
- `cargo deny check`

### Release (`.github/workflows/release.yml`)

Triggered by pushing a semver tag. Built with `cargo-dist` for:
- `aarch64-apple-darwin`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `x86_64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`

Generates shell/powershell/MSI installers and publishes to GitHub Releases + Homebrew.
