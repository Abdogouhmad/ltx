# ltx_linter

**Style linter for LaTeX documents.** Runs the full lex → parse → lint
pipeline and reports lexer errors, parser errors, and style findings under a
single unified `LTX::LINTER::E*` / `LTX::LINTER::W*` code namespace.

## Responsibilities

- orchestrating the lex → parse → lint pipeline through a single entry point
- AST and line-based style rules
- remapping lexer / parser errors into the unified `LTX::LINTER::E*` namespace
- applying the `[lints]` table from `ltx.toml`

## Key types

| Type | Role |
|------|------|
| `lint_file` | Entry point for a single `.tex` path + optional `[lints]` table, returns a `LintResult`. |
| `lint_project` | Entry point for checking a set of files as one project: define/use rules resolve references across files. |
| `ProjectUses` | Labels/macros referenced anywhere in a project, collected from the token streams and seeded into the define/use rules. |
| `LintResult` | Aggregated diagnostics (`error_count`, `warning_count`, `render_pretty`, `is_empty`). |
| `LintContext` | Shared lint input — the parsed `Document`, the `LtxSourceMap`, and the raw source text. |
| `LintRegistry` | Owns and runs every enabled rule; `with_defaults()` / `filtered()` against a `LintTable`. |
| `LintRule` | Enum over boxed `AstLintRule` / `LineLintRule` trait objects. |
| `AstLintRule` | Trait for visitor-based AST rules (record findings, emit in `finish`). |
| `LineLintRule` | Trait for line-scanning rules (`check_line`). |
| `LintError` | `UnknownRule` / `ConflictingDirective` — the `[lints]` config errors (`E018`–`E019`). |

## Rules

The default registry enables 14 rules (see the
[Linter Rules](../errors/linter.md) table for details and the full W001–W016
list). `unused-package` (W004) and `todo-comment` (W015) are deliberately
excluded from defaults.

## Usage

```rust,no_run
# use std::path::Path;
use ltx_linter::lint_file;

let result = lint_file(Path::new("main.tex"), None, true)?;
if result.has_errors() {
    if let Ok(rendered) = result.render_pretty() {
        println!("{rendered}");
    }
}
# Ok::<(), miette::Report>(())
```

Lower-level consumers can assemble the pipeline by hand:

```rust,no_run
use ltx_diagnostics::{LtxDiagnosticSink, LtxSourceMap};
use ltx_linter::{LintContext, LintRegistry};
use ltx_lexer::{LtxLexer, TokenStream};
use ltx_parser::LtxParser;
# use std::sync::Arc;
# let source = r#"\documentclass{article}
# \begin{document}
# hello
# \end{document}"#;

let mut source_map = LtxSourceMap::new();
let file_id = source_map.add_inline("main.tex", source);
let source_map = Arc::new(source_map);

let stream = TokenStream::new(LtxLexer::new(source, file_id, source_map.as_ref().clone()));
let mut parser = LtxParser::new(stream);
let document = ltx_parser::parse_document(&mut parser);

let ctx = LintContext::new(&document, source_map, source);
let mut sink = LtxDiagnosticSink::new();

LintRegistry::with_defaults(source)
    .run(&ctx, &mut sink);
```

## Error ownership

`LtxDiagnosticSource` (in `src/error.rs`) implements the diagnostics
rendering for lint findings. All 35 codes are registered in
`ALL_CODES` (E001–E019 + W001–W016); the W-codes are also listed in
`ALL_RULE_CODES` for `ltx code --lint`.

## Design notes

- Rules never assume node names — they are written against the parser
  `Visitor` trait, so a rename in `ltx_parser::ast` fails to compile instead
  of silently mis-firing.
- `todo_comment` is intentionally not implemented via raw `%` string search
  (false-positives in verbatim/math); it stays stubbed until the lexer emits
  a `Comment` token.
- `unused-label` / `unused-macro` are seeded from a token-level
  [`ProjectUses`](crate::session::ProjectUses) scan of every file: the AST
  visitor never descends into `$...$` math or braced groups, so usage is
  collected straight from the token stream (excluding macro definition names).
  `ltx check` runs this over the whole project, making define/use project-wide.
- The linter depends on `ltx_parser`, `ltx_diagnostics`, and `ltx_config`;
  it touches `ltx_lexer` directly only in the usage-collection pass.

## Related

- [Linter Rules](../errors/linter.md) — the W001–W016 table
- [Configuration](../guide/configuration.md) — the `[lints]` table
- [CLI Usage](../guide/cli.md) — `ltx check`
