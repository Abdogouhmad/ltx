# AGENT.md — ltx

Rust workspace (edition 2024). LaTeX project manager/toolchain. Tectonic is
first-class alongside pdflatex/xelatex/lualatex.

## Rules

- Match existing crate conventions. Check a sibling crate before inventing a
  new pattern.
- `#[must_use]`, `#[inline]`, `const fn` where applicable.
- Errors: single enum per crate, `thiserror`, `From`-based conversion into
  the crate's error type. No ad-hoc `Result<T, String>`.
- Diagnostics: route through `ltx_diagnostics::LtxDiagnosticSink` /
  `LtxDiagnostic`. Don't `println!`/`eprintln!` errors directly.
- Rendering: write-don't-allocate — generic `_into(&self, w: impl fmt::Write)`
  functions, not `-> String` unless trivial.
- `Drop` swallows errors — use explicit `.close()` (e.g. `TempDir`) when
  cleanup failure must surface.
- No comments unless logic is non-obvious. No unnecessary allocation.
- Prefer `Cow<'static, str>` for messages that are sometimes static, sometimes
  formatted.

## Workspace crates

`ltx_lexer` → `ltx_parser` (has `Visitor`/`VisitorMut`) → `ltx_diagnostics`
→ `ltx_config` (`LintTable`, `FmtTable`) → `ltx_linter` → `ltx_compiler` →
`ltx_cli`. `ltx_linter` depends on `ltx_parser` + `ltx_diagnostics` +
`ltx_config`.

## Before coding

1. Read the target crate's existing `src/` layout — don't restructure
   without asking.
2. Check `crates/ltx_parser/src/ast/` and `visitor/` for exact type names
   (`Command`, `Group`, `Environment`, etc.) — do not assume names from specs;
   specs may predate renames.
3. If a spec (`*-feat.md`) conflicts with what's actually in the repo,
   the repo wins — flag the conflict, don't silently pick one.

## Workflow

- Implement one file/struct at a time. Don't scaffold the whole crate in one
  pass.
- Run `cargo check -p <crate>` after each file. Fix before moving on.
- Don't touch other crates unless the task requires it.
- No speculative features beyond what the feat doc specifies.
