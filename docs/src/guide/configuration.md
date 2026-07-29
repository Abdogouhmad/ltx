# Configuration

Every LTX project is described by a `config.toml` at its root. The `ltx new` command generates this file automatically.

## Full reference

```toml
[project]
name = "my-paper"              # Required. Project name.
version = "0.1.0"              # Optional. Semver version string.
author = ["Author Name"]       # Optional. List of authors.
main = "src/main.tex"          # Optional. Path to main .tex file, relative to project root.

[engine]
compiler = "pdflatex"          # pdflatex | xelatex | lualatex | tectonic (default: pdflatex)
# args = ["-interaction=nonstopmode"]  # Optional extra compiler arguments.

[build]
name = "my-paper"              # Optional. Output PDF name (without extension).
outdir = "build"               # Optional. Output directory, relative to project root.
```

## Sections

### `[project]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Project name, used as the root directory identifier. |
| `version` | string | No | Semver version string. |
| `author` | list of strings | No | Project authors. |
| `main` | string | No | Path to the main `.tex` file relative to the project root. Defaults to `main.tex`. |

### `[engine]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `compiler` | string | No | LaTeX engine. One of `pdflatex`, `xelatex`, `lualatex`, `tectonic`. Default: `pdflatex`. |
| `args` | list of strings | No | Extra command-line arguments passed to the compiler. |

### `[build]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | No | Output PDF filename (without `.pdf` extension). |
| `outdir` | string | No | Output directory path, relative to project root. Default: `build`. |

## Scaffolding options

When you run `ltx new`, the directory layout is controlled by flags that map to internal options:

### `SrcLayout` — source file placement

| Flag | Layout | Result |
|------|--------|--------|
| (none) | `Flat` | `main.tex` in project root |
| `--src` | `WithSrcDir` | `src/main.tex` + `src/sections/` |

### `BibLayout` — bibliography placement

| Flag | Layout | Result |
|------|--------|--------|
| (none) | `Flat` | `references.bib` in project root |
| `--bib` | `WithBibDir` | `bib/references.bib` |

These flags combine freely. For example, `ltx new paper --src --bib` creates:

```
paper/
├── src/
│   ├── main.tex
│   └── sections/
├── bib/
│   └── references.bib
├── config.toml
├── build/
└── .gitignore
```

## Minimal config

A project with just a name and default settings:

```toml
[project]
name = "my-paper"

[engine]
compiler = "pdflatex"
```

## Custom compiler arguments

Pass extra flags to the LaTeX engine:

```toml
[engine]
compiler = "pdflatex"
args = ["-interaction=nonstopmode", "-halt-on-error"]
```
