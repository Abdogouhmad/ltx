# Compiler Errors (LTX::COMPILER::E001-E004, W001)

The compiler crate owns every error it can produce while turning a manifest
into a compiled PDF: missing configuration, missing input files, and engine
(tectonic) failures. `CompilerError` implements `miette::Diagnostic`, so it
converts into a `miette::Report` and is returned directly from
`miette::Result` functions.

## Error Reference Table

| Code | Variant | Diagnostic Message | Remediation |
|:----:|---------|-------------------|-------------|
| `LTX::COMPILER::E001` | `MissingMain` | no main file set in ltx.toml | Add `main = "main.tex"` under `[project]`. |
| `LTX::COMPILER::E002` | `MissingBuild` | no `[build]` section in ltx.toml | Add a `[build]` section with `engine` and `name`. |
| `LTX::COMPILER::E003` | `MainFileNotFound` | main file `{path}` not found | Create the file or fix the `main` value in `[project]`. |
| `LTX::COMPILER::E004` | `TectonicError` | engine failure `{message}` | Check the engine logs; verify your network connection for bundle issues. |
| `LTX::COMPILER::W001` | `EngineNotImplemented` | `{engine}` engine is not implemented yet | Use `engine = "tectonic"`, or wait for this engine to land. |

## Error Categories

### Configuration Errors (E001-E002)

These mirror the config-layer validation but occur at compile time, when the
resolved `CompilerConfig` is missing information it needs.

**E001 - MissingMain**  
The `[project]` section has no `main` entry, so the compiler doesn't know
which file to compile.

**E002 - MissingBuild**  
The manifest has no `[build]` section, so the engine and output name are
unknown.

### Input Errors (E003)

**E003 - MainFileNotFound**  
The main input file resolved from `[project].main` does not exist on disk at
build time. Note the config layer catches this earlier during validation
(`LTX::CONFIG::E004`); this variant covers paths resolved at compile time.

### Engine Errors (E004)

**E004 - TectonicError**  
The selected engine failed to fetch its support bundle, create its processing
session, or finish the compilation. The message carries the underlying engine
error.

### Warnings (W001)

**W001 - EngineNotImplemented**  
The selected engine is not wired up yet — only `tectonic` is available. The
warning is rendered through `miette` and the build still exits successfully.

## Diagnostic Example

```bash
LTX::COMPILER::W001

  ⚠ `pdflatex` engine is not implemented yet — only `tectonic` is available
  help: use `engine = "tectonic"` in `ltx.toml`, or wait for this engine to
        land
```

## Related Topics

- [Configuration](../guide/configuration.md) — engine selection in `ltx.toml`
- [Config Errors](config.md) — manifest validation errors
