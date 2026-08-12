# Configuration

Every LTX project is described by an `ltx.toml` at its root. The `ltx new`
command generates this file automatically. Three sections exist:
`[project]` for metadata, `[build]` for compilation settings, and `[lints]`
for linter rule configuration.

## Full reference

```toml
[project]
name = "my-paper"          # Required. Project name.
version = "0.1.0"          # Optional. Semver version string.
author = ["Author Name"]   # Optional. List of authors.
main = "src/main.tex"      # Required (validation). Path to the main .tex file.

[build]
name = "my-paper"          # Required (validation). Output PDF name, no extension.
engine = "tectonic"        # Required. pdflatex | xelatex | lualatex | tectonic
engine_args = ["-synctex=1"]   # Optional. Extra compiler arguments.

[build.options]            # Optional. Tectonic compilation options.
keep_logs = true           # Optional. Keep the .log file.   Default: true
keep_intermediates = false # Optional. Keep .aux/.synctex.gz. Default: false
synctex = true             # Optional. Emit SyncTeX data.     Default: true
only_cached = false        # Optional. Never hit the network. Default: false

[lints]                    # Optional. Linter rule configuration.
deny = ["unused-label"]    # Upgrade rules to errors.
warn = ["long-line"]       # Keep rules as warnings.
allow = ["todo-comment"]   # Turn rules off.
```

The compiled PDF is always written to the project's `target/` directory —
there is no output-directory option.

## Sections

### `[project]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Project name. |
| `version` | string | No | Semver version string. |
| `author` | list of strings | No | Project authors. |
| `main` | string | Yes* | Path to the main `.tex` file relative to the project root. |

\* `main` is optional in the data model but **required by validation** —
`ltx build` and `ltx check` refuse to run without it. See
`LTX::CONFIG::E001` in the [Config Errors](../errors/config.md) table.

### `[build]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes* | Output PDF filename (without `.pdf` extension). |
| `engine` | string | Yes | LaTeX engine: `pdflatex`, `xelatex`, `lualatex`, or `tectonic`. |
| `engine_args` | list of strings | No | Extra command-line arguments passed to the compiler. |

\* `name` is required by validation (`LTX::CONFIG::E003`).

### `[build.options]`

Tectonic compilation options. Every field defaults to a sensible value when
omitted, so a project only sets the options it wants to override.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keep_logs` | bool | `true` | Keep the `.log` file produced by the compiler. |
| `keep_intermediates` | bool | `false` | Keep intermediate build artifacts (`.aux`, `.synctex.gz`). |
| `synctex` | bool | `true` | Emit `SyncTeX` data for editor / PDF synchronization. |
| `only_cached` | bool | `false` | If `true`, never hit the network — fail if the bundle isn't cached. |

### `[lints]`

Linter rule configuration for `ltx check`. Each list holds rule slugs; the
union of the three lists must match registered rules (see the
[Linter Rules](../errors/linter.md) table for the full W001–W016 list).

| Field | Type | Description |
|-------|------|-------------|
| `deny` | list of strings | Rules upgraded from warnings to errors. |
| `warn` | list of strings | Rules kept as warnings (the default severity). |
| `allow` | list of strings | Rules turned off entirely. |

Precedence is `deny` > `warn` > `allow`. Errors:

- An unknown slug is rejected with `LTX::LINTER::E018` (e.g. a typo).
- The same slug in both `deny` and `allow` is rejected with
  `LTX::LINTER::E019` — LTX refuses to guess the intent.

## Validation rules

`ltx build` validates the manifest before compiling. Malformed TOML and
unknown keys (e.g. a typo like `[build.option]`) are rejected loudly instead
of silently ignored. The structural rules enforced by `validate_manifest()`:

1. `[project].main` is set **and** points to an existing file on disk
   (`LTX::CONFIG::E001`, `LTX::CONFIG::E004`).
2. A `[build]` section is present (`LTX::CONFIG::E002`).
3. `[build].name` is non-empty (`LTX::CONFIG::E003`).

## Scaffolding options

When you run `ltx new`, the directory layout is controlled by flags that map
to internal options:

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
├── ltx.toml
└── .gitignore
```

## Minimal config

A project with just the essentials (generated by `ltx new my-paper`):

```toml
[project]
name = "my-paper"
main = "main.tex"

[build]
name = "my-paper"
engine = "tectonic"
```

## Example: bibliography project

```toml
[project]
name = "thesis"
version = "0.1.0"
author = ["Jane Doe <jane@example.com>"]
main = "src/main.tex"

[build]
name = "thesis"
engine = "tectonic"

[build.options]
keep_logs = false
only_cached = true
```

## Related

- [CLI Usage](cli.md) — the `build` / `check` / `code` commands
- [Config Errors](../errors/config.md) — the `LTX::CONFIG::E0xx` code table
- [Linter Rules](../errors/linter.md) — the `[lints]` rule slugs
