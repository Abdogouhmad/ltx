# CLI Usage

LTX provides seven subcommands. Run `ltx --help` for the full reference.

```
ltx [COMMAND]
```

## `ltx new`

Create a new LTX project with starter files.

```bash
ltx new <name> [OPTIONS]
```

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--engine` | `-e` | LaTeX compiler to use (`pdflatex`, `xelatex`, `lualatex`, `tectonic`) | `tectonic` |
| `--src` | | Use `src/` directory layout for source files | off |
| `--bib` | | Include bibliography support (`bib/` directory) | off |

### Examples

```bash
# Flat layout (main.tex in project root)
ltx new my-paper

# With src/ directory
ltx new my-paper --src

# With bib/ directory and xelatex engine
ltx new my-paper --bib --engine xelatex

# Full layout: src/ + bib/ + tectonic
ltx new my-paper --src --bib --engine tectonic
```

### Scaffolded layout

Without flags:

```
my-paper/
├── main.tex
├── references.bib
├── ltx.toml
└── .gitignore
```

With `--src`:

```
my-paper/
├── src/
│   ├── main.tex
│   └── sections/
├── references.bib
├── ltx.toml
└── .gitignore
```

With `--bib`:

```
my-paper/
├── main.tex
├── bib/
│   └── references.bib
├── ltx.toml
└── .gitignore
```

## `ltx check`

Run the full lex → parse → lint → diagnostics pipeline on `.tex` files.
With no arguments every `.tex` file under the project is checked recursively
(like `cargo check`), skipping generated `target/` and `.git/` directories.
The recursive scan treats the files as one project: `unused-label` and
`unused-macro` resolve references across files, so a label or macro defined
in one file and used in another isn't reported as unused.

```bash
ltx check [OPTIONS]
```

| Flag | Description |
|------|-------------|
| `-p <path>` | Check a single `.tex` file instead of scanning the project. |
| `--no-lint` | Skip the style linter; only lex and parse. |

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | Check passed (no errors; warnings are OK) |
| `4` | Diagnostics with errors were found |

### Examples

```bash
# Check every .tex file in the project
ltx check

# Check a single file
ltx check -p main.tex

# Lex + parse only, no style lints
ltx check --no-lint
```

```bash
$ ltx check
Check passed — no issues found across 3 file(s).
```

If errors are found, LTX renders them with source locations and help messages:

```
LTX::LINTER::E003

  × unmatched brace detected: `{`
   ╭─[main.tex:5:12]
 4 │ \section{Introduction
 5 │ % missing closing brace
   ·            ────┬────
   ·                ╰── here
 6 │
   ╰────
  help: Verify that every opening brace `{` has a matching closing brace `}`.
```

Style findings are reported under the `LTX::LINTER::W0xx` rules — see the
[Linter Rules](../errors/linter.md) table.

## `ltx build`

Compile the project described in `ltx.toml` into a PDF.

```bash
ltx build
```

The manifest is validated first — missing keys, typos, and a missing main
file fail fast with a source-spanning `LTX::CONFIG::E0xx` diagnostic. The
selected engine then compiles the document:

- `tectonic` — compiled in-process by the `tectonic` crate (the default).
- `pdflatex` / `xelatex` / `lualatex` — not wired up yet; a
  `LTX::COMPILER::W001` warning is printed and the build exits successfully.

Output is always written to the project's `target/` directory.

## `ltx clean`

Remove the `target/` directory and print a summary of deleted files and total
size, akin to `cargo clean`.

```bash
ltx clean
```

| Flag | Description |
|------|-------------|
| `-v` | Verbose output |
| `-vv` | More verbose output |

## `ltx code`

List all registered diagnostic error codes with their descriptions, severities, and owning phase.

```bash
ltx code [FILTER]
```

| Flag | Description |
|------|-------------|
| `--lexer` | Show only lexer codes (`LTX::LEXER::E0xx`) |
| `--parser` | Show only parser codes (`LTX::PARSER::E0xx`) |
| `--config` | Show only config codes (`LTX::CONFIG::E0xx`) |
| `--compiler` | Show only compiler codes (`LTX::COMPILER::E0xx` / `W0xx`) |
| `--lint` | Show only linter codes (`LTX::LINTER::E0xx` / `W0xx`) |
| `--all` | Show codes from every phase |
| `-e` / `--errors` | Show only error-severity codes |
| `-w` / `--warnings` | Show only warning-severity codes |

By default `ltx code` shows the unified linter codes, which are the global
namespace for `ltx check`. The lexer, parser, config, and compiler codes are
listed only when their phase flag or `--all` is given.

Output:

```
CODE                     DESCRIPTION                                SEVERITY  PHASE
--------------------------------------------------------------------------------
LTX::LINTER::E001        Unexpected Token                           error     linter
LTX::LINTER::E002        Unexpected End of File                     error     linter
LTX::LINTER::W001        unused-label                               warning   linter
...
LTX::LINTER::W016        empty-command                              warning   linter

35 total codes
```

## `ltx update`

Update the `ltx` binary to the latest GitHub release.

```bash
ltx update [OPTIONS]
```

| Flag | Description |
|------|-------------|
| `--check` | Check for a newer release and report without installing. |
| `-y` / `--yes` | Skip the confirmation prompt and install immediately. |

`ltx self-update` is accepted as an alias. The update is downloaded from the
GitHub Releases page for the current platform (`ltx_cli-<target>.tar.gz` on
Unix, `ltx_cli-<target>.zip` on Windows) and atomically replaces the running
binary, so the tool must have been installed from a release artifact.

```bash
# Check for a newer release
ltx update --check

# Update interactively (confirms before replacing)
ltx update

# Update without prompting
ltx update --yes
```

### Background update check

Every command except `ltx update` quietly checks the latest release on a
background thread and prints a hint to stderr when a newer version exists.
Set the `LTX_NO_UPDATE_CHECK` environment variable to disable it:

```bash
export LTX_NO_UPDATE_CHECK=1
```

## Global flags

| Flag | Description |
|------|-------------|
| `--manifest-path <path>` | Path to a manifest file (`ltx.toml`); relative paths resolve against its directory. |
| `--message-format <human\|json>` | Output format for status messages. |
| `-v` | Verbosity level (repeatable). |
| `--help` | Show help information |
| `--version` | Show version number |

## Engine options

LTX supports four LaTeX compilers, set via `ltx new --engine` or in the
`[build]` section of `ltx.toml`:

| Engine | Binary | Notes |
|--------|--------|-------|
| `tectonic` | `tectonic` | **Default.** Self-contained, Cargo-like LaTeX toolchain. |
| `pdflatex` | `pdflatex` | Most common engine. |
| `xelatex` | `xelatex` | Unicode and system-font support. |
| `lualatex` | `lualatex` | Lua-extensible engine. |

## Next steps

- [Configuration](configuration.md) — customize your project with `ltx.toml`
- [Lexer Errors](../errors/lexer.md) — the `LTX::LEXER::E0xx` code table
