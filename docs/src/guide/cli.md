# CLI Usage

LTX provides three subcommands. Run `ltx --help` for the full reference.

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
| `--engine` | `-e` | LaTeX compiler to use | `pdflatex` |
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

# Full layout: src/ + bib/ + lualatex
ltx new my-paper --src --bib --engine lualatex
```

### Scaffolded layout

Without flags:

```
my-paper/
├── main.tex
├── references.bib
├── config.toml
├── build/
└── .gitignore
```

With `--src`:

```
my-paper/
├── src/
│   ├── main.tex
│   └── sections/
├── config.toml
├── build/
└── .gitignore
```

With `--bib`:

```
my-paper/
├── main.tex
├── bib/
│   └── references.bib
├── config.toml
├── build/
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
| `2` | Input file not found |
| `4` | Diagnostics with errors were found |

### Example

```bash
$ ltx check main.tex
Check passed — no issues found.
```

If errors are found, LTX renders them with source locations and help messages:

```
LTX::E003

  × unmatched brace detected
   ╭─[main.tex:5:12]
 4 │ \section{Introduction
 5 │ % missing closing brace
   ·            ────┬────
   ·                ╰── here
 6 │
   ╰────
  help: Ensure every opening `{` has a matching closing `}`.
```

## `ltx code`

List all registered diagnostic error codes with their descriptions and severity levels.

```bash
ltx code
```

Output:

```
CODE           DESCRIPTION                                 SEVERITY
----------------------------------------------------------------------
LTX::E001      Unexpected Token                            error
LTX::E002      Unexpected End of File                      error
LTX::E003      Unmatched Brace                             error
...
LTX::E108      Command Redefined                            error

18 total codes
```

## Global flags

| Flag | Description |
|------|-------------|
| `--help` | Show help information |
| `--version` | Show version number |

## Engine options

LTX supports four LaTeX compilers, set via `--engine` or in `config.toml`:

| Engine | Binary | Notes |
|--------|--------|-------|
| `pdflatex` | `pdflatex` | Default. Most common engine. |
| `xelatex` | `xelatex` | Unicode and system-font support. |
| `lualatex` | `lualatex` | Lua-extensible engine. |
| `tectonic` | `tectonic` | Self-contained, Cargo-like LaTeX toolchain. |

## Next steps

- [Configuration](configuration.md) — customize your project with `config.toml`
