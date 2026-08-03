# CLI Usage

LTX provides five subcommands. Run `ltx --help` for the full reference.

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

Check a `.tex` file for syntax errors by running the full lex → parse → diagnostics pipeline.

```bash
ltx check <path>
```

| Argument | Description |
|----------|-------------|
| `path` | Path to a `.tex` file to check |

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | Check passed (no errors; warnings are OK) |
| `1` | Failed to read the file (missing, not UTF-8, etc.) |
| `4` | Diagnostics with errors were found |

### Example

```bash
$ ltx check main.tex
Check passed — no issues found.
```

If errors are found, LTX renders them with source locations and help messages:

```
LTX::LEXER::E003

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
| `--all` | Show all codes (default) |
| `-e` / `--errors` | Show only error-severity codes |
| `-w` / `--warnings` | Show only warning-severity codes |

Output:

```
CODE                     DESCRIPTION                                SEVERITY  PHASE
--------------------------------------------------------------------------------
LTX::LEXER::E001         Unexpected Token                           error     lexer
LTX::LEXER::E002         Unexpected End of File                     error     lexer
LTX::LEXER::E003         Unmatched Brace                            error     lexer
...
LTX::COMPILER::W001      Engine Not Implemented                     warning   compiler

30 total codes
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
