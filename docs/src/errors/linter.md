# Linter Codes (LTX::LINTER::E0xx, W0xx)

The linter runs the full lex → parse → lint pipeline and re-emits every
diagnostic under a single unified `LTX::LINTER::` namespace:

- `E001`–`E011` mirror the lexer errors.
- `E012`–`E017` mirror the parser errors.
- `E018`–`E019` are `[lints]` configuration errors.
- `W001`–`W016` are the style lint rules.

## Unified Errors (E001–E019)

These are the lexer and parser errors surfaced through `ltx check`. Their
descriptions and remediation match their source-phase twins — see the
[Lexer Errors](lexer.md) and [Parser Errors](parser.md) tables.

| Code | Description | Mirrors |
|:----:|-------------|---------|
| `LTX::LINTER::E001` | Unexpected Token | `LTX::LEXER::E001` |
| `LTX::LINTER::E002` | Unexpected End of File | `LTX::LEXER::E002` |
| `LTX::LINTER::E003` | Unmatched Brace | `LTX::LEXER::E003` |
| `LTX::LINTER::E004` | Invalid Math Delimiter | `LTX::LEXER::E004` |
| `LTX::LINTER::E005` | Unterminated Argument | `LTX::LEXER::E005` |
| `LTX::LINTER::E006` | Invalid Escape Sequence | `LTX::LEXER::E006` |
| `LTX::LINTER::E007` | Invalid Unicode | `LTX::LEXER::E007` |
| `LTX::LINTER::E008` | Illegal Parameter Character Usage | `LTX::LEXER::E008` |
| `LTX::LINTER::E009` | Unterminated Verbatim Block | `LTX::LEXER::E009` |
| `LTX::LINTER::E010` | Invalid Character | `LTX::LEXER::E010` |
| `LTX::LINTER::E011` | Mismatched Environment | `LTX::LEXER::E011` |
| `LTX::LINTER::E012` | Expected Token | `LTX::PARSER::E001` |
| `LTX::LINTER::E013` | Unexpected End of File | `LTX::PARSER::E002` |
| `LTX::LINTER::E014` | Unclosed Environment | `LTX::PARSER::E003` |
| `LTX::LINTER::E015` | Mismatched Environment | `LTX::PARSER::E004` |
| `LTX::LINTER::E016` | Missing Closing Brace | `LTX::PARSER::E005` |
| `LTX::LINTER::E017` | Unexpected End of File While Parsing | `LTX::PARSER::E006` |
| `LTX::LINTER::E018` | Unknown Lint Rule | — |
| `LTX::LINTER::E019` | Conflicting Lint Directive | — |

### `[lints]` configuration errors (E018–E019)

**E018 - UnknownLintRule**  
A slug in `[lints]` matches no registered rule (a typo like `unused-label `,
or a rule that isn't in the default set). Fix the slug in `ltx.toml`.

**E019 - ConflictingLintDirective**  
The same slug appears in both `deny` and `allow`, so the intent is ambiguous.
LTX refuses to guess — remove one of the two entries.

## Style lint rules (W001–W016)

### Reference table

| Code | Slug | Kind | On by default |
|:----:|------|------|:-------------:|
| `LTX::LINTER::W001` | `unused-label` | AST | yes |
| `LTX::LINTER::W002` | `unused-macro` | AST | yes |
| `LTX::LINTER::W003` | `duplicate-package-import` | AST | yes |
| `LTX::LINTER::W004` | `unused-package` | AST | no |
| `LTX::LINTER::W005` | `deprecated-command` | AST | yes |
| `LTX::LINTER::W006` | `empty-environment` | AST | yes |
| `LTX::LINTER::W007` | `empty-section` | AST | yes |
| `LTX::LINTER::W008` | `trailing-whitespace` | line | yes |
| `LTX::LINTER::W009` | `deprecated-package` | AST | yes |
| `LTX::LINTER::W010` | `long-line` | line | yes |
| `LTX::LINTER::W011` | `mixed-indentation` | line | yes |
| `LTX::LINTER::W012` | `redundant-braces` | AST | yes |
| `LTX::LINTER::W013` | `multiple-blank-lines` | line | yes |
| `LTX::LINTER::W014` | `missing-caption` | AST | yes |
| `LTX::LINTER::W015` | `todo-comment` | line | no (stubbed) |
| `LTX::LINTER::W016` | `empty-command` | AST | yes |

### Rule details

**W001 - unused-label**  
A `\label{...}` is defined but never referenced by `\ref` / `\pageref` /
`\eqref` (or another `\ref`-family command). References are resolved
project-wide: when `ltx check` scans the whole project, a label defined in
one file and referenced from another counts as used. Every finding includes
a `help:` hint showing how to use the label — `\ref{sec:intro}` plus the
`\cref` / `\autoref` / `\pageref` alternatives — or how to remove the
orphaned `\label{...}` line if it isn't needed.

**W002 - unused-macro**  
A macro defined with `\newcommand` / `\renewcommand` is never invoked. Usage
is collected from the token stream, so invocations inside `$...$` math or
braced groups count — but the name in `\newcommand{\R}{...}` itself does
not. Like `unused-label`, invocations in other files of the project count
when checking recursively.

**W003 - duplicate-package-import**  
A package name appears twice in one `\usepackage{...}` call (e.g.
`\usepackage{graphicx,graphicx}`).

**W004 - unused-package**  
A `\usepackage{...}` import is never exercised by any known command. Off by
default — the `PACKAGE_COMMANDS` table is not yet populated.

**W005 - deprecated-command**  
A legacy command (`\it`, `\bf`, `\rm`, …) is used where a modern alternative
exists.

**W006 - empty-environment**  
An environment whose body is empty or whitespace-only (e.g. an unused
`\begin{center} \end{center}`).

**W007 - empty-section**  
A `\section`-family command with no content following it before the next
section or the end of the document.

**W008 - trailing-whitespace**  
A line ends with spaces or tabs.

**W009 - deprecated-package**  
A deprecated package (`subfigure`, `fancybox`, …) is imported.

**W010 - long-line**  
A line exceeds 100 columns.

**W011 - mixed-indentation**  
A line's leading whitespace mixes tabs and spaces.

**W012 - redundant-braces**  
A brace group wraps a single node in a context where grouping has no effect,
e.g. `{\emph{word}}`.

**W013 - multiple-blank-lines**  
More than one consecutive blank line.

**W014 - missing-caption**  
A `figure` / `table` environment contains no `\caption` descendant.

**W015 - todo-comment**  
A `% TODO` comment is left in the source. **Stubbed** — not part of the
default registry until the lexer emits a dedicated `Comment` token (a raw
`%` string search would false-positive inside verbatim/math).

**W016 - empty-command**  
A command invoked with an empty braced argument, e.g. `\author{}` or
`\textbf{}`.

## Configuring rules

Rules are configured in `ltx.toml` under `[lints]`:

```toml
[lints]
deny = ["unused-label"]     # upgrade a rule to an error
warn = ["long-line"]        # downgrade/keep as a warning
allow = ["mixed-indentation"]  # turn a rule off
```

- A slug in both `deny` and `allow` is a conflict (`LTX::LINTER::E019`).
- An unknown slug is rejected (`LTX::LINTER::E018`).
- See the [Configuration](../guide/configuration.md) guide for the full
  `ltx.toml` reference.
