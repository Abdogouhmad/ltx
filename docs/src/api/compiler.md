# ltx_compiler

**Compilation orchestration.** Turns a validated manifest into a compiled PDF
by dispatching to a configured engine. Currently only the `tectonic` engine
is wired up; the other engines emit a warning.

## Responsibilities

- compilation pipeline (`build`)
- engine abstraction (`CompilerConfig`, engine dispatch)
- tectonic integration (`tectonic_compile`)
- file watching for rebuild-on-save (`watch` — stub)
- compiler-specific diagnostics (owned by this crate)

## Key types

| Type | Role |
|------|------|
| `CompilerConfig` | Engine, output name, main file, and compile options resolved from a manifest. |
| `CompilerError` | All compiler diagnostics (`LTX::COMPILER::E001`–`E004`, `W001`). |
| `build::build` | Entry point: resolves the main file, then dispatches to the engine. |
| `tectonic::tectonic_compile` | Drives tectonic's `ProcessingSessionBuilder` to produce a PDF. |

## Error ownership

`CompilerError` (in `src/error.rs`):

| Code | Variant |
|------|---------|
| `E001` | `MissingMain` — no `[project].main` at compile time |
| `E002` | `MissingBuild` — no `[build]` section |
| `E003` | `MainFileNotFound` — main file missing on disk |
| `E004` | `TectonicError` — bundle fetch / session creation / compilation failed |
| `W001` | `EngineNotImplemented` — engine not wired up yet (warning) |

`CompilerError` implements `miette::Diagnostic` so it converts into
`miette::Report` and can be returned directly from `miette::Result`
functions. See the [Compiler Errors](../errors/compiler.md) table.

## Engine behavior

- `tectonic` — real compilation via the `tectonic` crate.
- `pdflatex` / `xelatex` / `lualatex` — emit `LTX::COMPILER::W001` through
  `miette` to stderr and return `Ok(())` (a warning never fails the build).

## Usage

```rust
use ltx_compiler::{CompilerConfig, build};

let manifest = ltx_config::LtxManifest::from_file("ltx.toml")?;
let config = CompilerConfig::from_manifest(&manifest)?;
build::build(&config, project_root)?;
```

## Design notes

- Input is a `CompilerConfig`; output always goes to `target/`.
- `watch.rs` exists as a placeholder for a future `ltx watch` command.
