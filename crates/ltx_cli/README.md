# ltx_cli

**The `ltx` command-line interface.** The only binary crate in the workspace —
parses arguments with `clap`, builds the shared `AppContext`, dispatches to
subcommands, and maps results to process exit codes.

## Subcommands

| Command | Purpose | Exit codes |
|---------|---------|------------|
| `new` | Scaffold a new LaTeX project (`--engine`, `--src`, `--bib`). | 0 |
| `check` | Lex → parse → diagnose a `.tex` file without producing output. | 0 clean / 4 diagnostics |
| `code` | List all registered diagnostic codes; filter by phase (`--lexer`/`--parser`/`--config`/`--compiler`/`--all`) and severity (`-e`/`-w`). | 0 |
| `clean` | Remove `target/` artifacts with a summary; `-v`/`-vv` for verbose output. | 0 |
| `build` | Compile the project from `ltx.toml` using the configured engine. | 0 success / 1 error |

## Architecture

- `cli.rs` — `Cli` struct: global flags (`--manifest-path`, `--message-format`,
  `-v`) plus the `Command` subcommand enum; `run()` dispatches.
- `ctx.rs` — `AppContext` (manifest path, output format, verbosity) and the
  `CliCommand` trait every subcommand implements via `execute(&AppContext)`.
- `commands/` — one module per subcommand (`new`, `check`, `code`, `clean`, `build`).
- `error.rs` — `CliError` (I/O, non-UTF-8, lexer/parser failures). `DiagnosticsFound`
  is downcast by name in `main.rs` to produce exit code 4.
- `exit_code.rs` — documented exit-code constants.

## Global flags

| Flag | Description |
|------|-------------|
| `--manifest-path <path>` | Path to `ltx.toml`; file paths resolve relative to it. |
| `--message-format <human|json>` | Output format for status messages. |
| `-v` | Verbosity (repeatable: `-v`, `-vv`, `-vvv`). |

## Example

```bash
ltx new my-paper --engine tectonic --src
cd my-paper
ltx check src/main.tex
ltx build
ltx code --lexer
ltx clean
```

## Design notes

- Status output goes to stderr; exit codes are script-friendly for CI.
- Renders diagnostics through `ltx_diagnostics` + `miette`; it never owns
  domain errors itself.
