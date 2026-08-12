# ltx_config

**Configuration and project scaffolding.** Provides the data model for
`ltx.toml`, manifest validation, and project generation.

## Responsibilities

- manifest parsing (`LtxManifest`)
- configuration validation (`validate_manifest`)
- project scaffolding (`scaffold`)
- the `[lints]` table consumed by the linter (`LintTable`)
- config-specific diagnostics (owned by this crate)

## Key types

| Type | Role |
|------|------|
| `LtxManifest` | Top-level `ltx.toml` structure; `LtxManifest::from_file()` reads + parses + validates in one step. |
| `Project` | `[project]` table — name, version, author, main file. |
| `Build` | `[build]` table — output name, engine, compile options. |
| `CompileOptions` | `[build.options]` — `keep_logs`, `keep_intermediates`, `synctex`, `only_cached` (all with sensible defaults). |
| `CompilerEngine` | Enum: `PdfLaTeX`, `XeLaTeX`, `LuaLaTeX`, `Tectonic`. |
| `LintTable` | `[lints]` table — `deny` / `warn` / `allow` rule slugs applied by `ltx_linter`. |
| `ScaffoldOptions` | Options for `ltx new` (`name`, `engine`, `src`, `bib`). |
| `SrcLayout` / `BibLayout` | Flat vs `src/` / `bib/` directory layouts. |
| `ConfigError` | All 8 config diagnostics (`LTX::CONFIG::E001`–`E008`). |

## Error ownership

`ConfigError` (in `src/error.rs`) owns every manifest/scaffold diagnostic,
including the merged former `ManifestDiagnostic` + `ScaffoldError`:

| Code | Variant |
|------|---------|
| `E001` | `MissingMain` — missing `main` in `[project]` |
| `E002` | `MissingBuild` — missing `[build]` section |
| `E003` | `MissingBuildName` — missing `name` in `[build]` |
| `E004` | `MainFileNotFound` — `[project].main` points to a missing file |
| `E005` | `InvalidToml` — malformed TOML / unknown keys |
| `E006` | `ReadFailed` — `ltx.toml` unreadable |
| `E007` | `Io` — scaffold I/O error |
| `E008` | `AlreadyExists` — project directory already exists |

Validation errors embed the raw source text and a byte span so they render
with miette snippets pointing at the offending table or key. See the
[Config Errors](../errors/config.md) table.

## Usage

```rust
use ltx_config::{LtxManifest, Project, Build, CompilerEngine, scaffold, ScaffoldOptions};

// Read + validate in one step
let manifest = LtxManifest::from_file("ltx.toml")?;

// Scaffold a new project
scaffold(&project_dir, &ScaffoldOptions::new("my-paper", CompilerEngine::Tectonic))?;
```

## Design notes

- Unknown keys in `ltx.toml` are rejected loudly (`serde deny_unknown_fields`)
  instead of silently ignored.
- No dependency on lexer/parser — the config layer only talks to
  `ltx_diagnostics` (for `ErrorCode`) and `ltx_utils` (filesystem helpers).
