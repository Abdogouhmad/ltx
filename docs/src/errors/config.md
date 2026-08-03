# Config Errors (LTX::CONFIG::E001-E008)

The config crate owns every error it can produce while reading or validating
`ltx.toml` and while scaffolding a new project. Manifest-validation errors
embed the raw source text and a byte span so they render with a miette snippet
pointing at the offending table or key; read failures and scaffold I/O errors
have no source snippet.

## Error Reference Table

| Code | Variant | Diagnostic Message | Remediation |
|:----:|---------|-------------------|-------------|
| `LTX::CONFIG::E001` | `MissingMain` | missing `main` in the `[project]` section | Add `main = "src/main.tex"` (or the path to your main file). |
| `LTX::CONFIG::E002` | `MissingBuild` | missing `[build]` section | Add a `[build]` section with `name = "..."` and `engine = "tectonic"`. |
| `LTX::CONFIG::E003` | `MissingBuildName` | missing `name` in the `[build]` section | Add `name = "output"` (without the `.pdf` extension). |
| `LTX::CONFIG::E004` | `MainFileNotFound` | main file not found: `{path}` | Create the file or fix the `main` value in the `[project]` section. |
| `LTX::CONFIG::E005` | `InvalidToml` | invalid `ltx.toml`: `{reason}` | Fix the TOML syntax or remove the unknown key. |
| `LTX::CONFIG::E006` | `ReadFailed` | failed to read `{path}`: `{error}` | Make sure the file exists and is readable. |
| `LTX::CONFIG::E007` | `Io` | (transparent I/O error) | Fix the underlying filesystem failure. |
| `LTX::CONFIG::E008` | `AlreadyExists` | project directory `{0}` already exists | Remove the directory or choose a different project name. |

## Validation rules

`validate_manifest()` (called by `ltx build` and `LtxManifest::from_file()`)
rejects a manifest when:

1. `[project].main` is missing (`E001`) or points to a file that does not
   exist (`E004`).
2. The `[build]` section is absent (`E002`).
3. `[build].name` is missing (`E003`).

Malformed TOML and unknown keys (e.g. a typo like `[build.option]`) are
reported as `E005` before any semantic checks run.

## Diagnostic Example

```bash
LTX::CONFIG::E001

  × missing `main` in the `[project]` section
   ╭─[ltx.toml:1:1]
 1 │ [project]
   · ────┬────
   ·     ╰── the `main` key is required here
 2 │ name = "demo"
   ╰────
  help: add `main = "src/main.tex"` (or the path to your main file)
```

## Related Topics

- [Configuration](../guide/configuration.md) — the `ltx.toml` reference
- [Compiler Errors](compiler.md) — errors produced during compilation
