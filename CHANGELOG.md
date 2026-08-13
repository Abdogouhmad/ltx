# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Note:** LTX is pre-1.0. Minor versions may contain breaking changes.

## [0.2.1-alpha.1]

### Added
- `ltx update` command to self-update the binary from the latest GitHub release, with `--check` to inspect without installing and `--yes` to skip the confirmation prompt (`self-update` is available as an alias)
- background update check: every command (except `ltx update`) quietly checks for a newer release on a detached thread and hints on stderr when one exists; suppress with the `LTX_NO_UPDATE_CHECK` environment variable
- `CliError::Update` variant wrapping `self_update` errors with the same `#[from]` conversion convention as the other CLI errors
- release archives now ship as `.tar.gz` on Unix (via `unix-archive` in `dist-workspace.toml`) so `self_update` can decompress them — the tool cannot read the default `.tar.xz`

## [0.2.0]

### Added
- added the following commands [`new`, 'clean', `code`, 'check']
- `AppContext` and `CliCommand` to the cli context
- `OutputFormat` Output format for status messages.
- `manifest-path`  Path to a manifest file (`ltx.toml`) for project-level configuration.
- `tectonic` as the default engine and curently it is the only engine supported
- `build` command to compile the document using the configured engine tectonic
- flexibility with naming output file pdf
- `[build.options]` table to control tectonic compilation: `keep_logs`, `keep_intermediates`, `synctex`, and `only_cached`, each defaulting to a sensible value when omitted
- manifest validation via `validate_manifest()` with pretty `miette` errors that point at the offending table or key and suggest the correct fix
- `LtxManifest::from_file()` now reads, parses, and validates a manifest in one step
- unknown keys in `ltx.toml` (e.g. a typo like `[build.option]`) are now rejected loudly instead of being silently ignored
- per-crate error ownership: `LexerError`, `ParserError`, `ConfigError`, and `CompilerError` enums, each defined in its owning crate's `error.rs` together with a `pub const ALL_CODES` registry
- 30 diagnostic codes namespaced by phase — `LTX::LEXER::E0xx` (11), `LTX::PARSER::E0xx` (6), `LTX::CONFIG::E0xx` (8), `LTX::COMPILER::E0xx`/`W0xx` (5)
- `ltx code` phase filters (`--lexer`, `--parser`, `--config`, `--compiler`, `--all`) on top of the existing `-e`/`-w` severity filters, with a new `PHASE` column
- a `README.md` for every crate documenting its responsibilities
- `ltx_compiler::watch`: `WatchConfig` + `run_watch()` — a recursive, debounced file watcher that rebuilds the project through the configured engine on every relevant change, with an initial compile on startup for immediate feedback
- watch event filtering: only `.tex`/`.sty`/`.cls`/`.bib` files trigger a rebuild, while tectonic's own output churn (`target/`, `build/`, `.git/`, `_minted`) is ignored
- new watch-mode diagnostics `LTX::COMPILER::E005` (watcher init failed) and `LTX::COMPILER::E006` (watch channel closed), registered in `ALL_CODES` (32 codes total)
- `ltx watch` command: resolves `ltx.toml`, validates the manifest, compiles once on startup, then rebuilds on every relevant save for a tight write–compile–preview loop
- script to build docs for github deployment
- `ltx_linter` crate: a unified lex → parse → lint pipeline that reports lexer errors, parser errors, and style findings under a single `LTX::LINTER::E*` / `LTX::LINTER::W*` code namespace
- parser visitor infrastructure (`ltx_parser::visitor`) for walking the AST, which the linter rules run against
- 16 built-in lint rules (`LTX::LINTER::W001`–`W016`): `unused-label`, `unused-macro`, `duplicate-package-import`, `unused-package` (off by default), `deprecated-command`, `empty-environment`, `empty-section`, `trailing-whitespace`, `deprecated-package`, `long-line`, `mixed-indentation`, `redundant-braces`, `multiple-blank-lines`, `missing-caption`, `todo-comment` (stubbed), and `empty-command`
- `[lints]` table in `ltx.toml` with `deny` / `warn` / `allow` to configure individual rules, and 2 new lint-table config error codes (`LTX::LINTER::E018`–`E019`)
- `ltx check` now checks every `.tex` file under the project recursively (like `cargo check`) and exits with code 4 when errors are found, making it suitable for CI pipelines
- `-p <file.tex>` flag to check a single file and `--no-lint` to skip the style linter (lex + parse only)
- `ltx code --lint` phase filter for the unified linter codes, which are now the default output of `ltx code`
### Changed
- `outputdir` is no longer supported there is a forced directory called `target/` for better consistency and efficiency
- wrapping the engine configuration inside `[build]` 
- name of toml config is changed from `config.toml` to `ltx.toml`
- `clean` command now has verbose output -v flag or for more verbose output -vv
- the engine default is changed from `pdflatex` to `tectonic`
- changed the out directory from `build/` to `target/`
- changing the way ltx approach the cli context by introducing `AppContext` and `CliCommand`
- `ltx_diagnostics` is now pure infrastructure: it defines no domain errors and only provides spans, source-map management, the `LtxDiagnosticSource` trait, the `ErrorCode` registry, and `miette` rendering
- removed the centralized error definitions: `ltx_diagnostics/src/errors.rs`, `ltx_lexer/src/errors_core.rs`, and `ltx_lexer/src/errors_factory.rs`
- `LtxDiagnostic` now wraps `Arc<dyn LtxDiagnosticSource>` + `Arc<LtxSourceMap>` instead of a concrete `LtxError`, so the diagnostics crate never needs to know which crate produced an error
- `LtxParser::new` absorbs the lexer's diagnostics into its own `ParserErrorHandler`, so a single sink reports both phases
- unimplemented compiler engines (`pdflatex`, `xelatex`, `lualatex`) now emit an `LTX::COMPILER::W001` warning rendered through `miette` instead of a bare `println!`, and still exit successfully
- `watch.rs` no longer shells out to a `tectonic` binary; the watch rebuild now drives `build::build` → `tectonic_compile` through the same engine path as `ltx build`
- removed the unused `LTX::COMPILER::E007` (`Spawn`) variant, since watch mode no longer spawns a subprocess
- `ltx check` no longer takes a positional file path; with no arguments it scans the whole project, and a single file is passed via `-p`
- linter rule boilerplate is now generated by `rule_identity!` and `ast_findings_rule!` macros instead of being duplicated per rule

### Fixed
- Today's date is fixed now during compilation
- `ltx build` now validates `ltx.toml` before compiling, so broken manifests (missing keys, typos, missing main file) fail fast instead of silently falling back to defaults
- source-map cloning in the error collection path now uses `Arc<LtxSourceMap>` instead of cloning the map per diagnostic
- `ltx watch` no longer loops or grows memory: watch targets are now derived from `[project].main` — `src/` for structured (`ltx new --src`) projects, the main file itself for single-structure (`ltx new`) projects — so the compiler's own `target/` output is never watched and can't feed back into a rebuild
- `ltx watch` ignores `Access`-kind events (the engine opening/reading the source during a compile), which was the real source of the self-triggering rebuild loop; only real writes (`Modify`/`Create`/`Remove`) rebuild
- `ltx watch` logs the exact paths that trigger each rebuild, making a self-triggered compile loop immediately visible
- `ltx check` recursive scan is rooted at the manifest directory (not forced to `src/`) and skips generated `target/` and `.git/` directories, so compiled output is never linted
- `unused-label` (W001) and `unused-macro` (W002) no longer fire on definitions that are actually used: `ltx check` now resolves define/use project-wide, so a label or macro defined in one file and referenced from another — or used only inside `$...$` math or braced groups (which the AST visitor never descends into) — is counted as used. Usage is collected from every file's token stream, excluding macro definition names so `\newcommand{\R}` doesn't count as using `\R`
- `unused-label` (W001) findings now carry a `help:` hint rendered by `miette` that teaches how to use the label — e.g. `\ref{sub:intro}` (plus `\cref`, `\autoref`, `\pageref`) — or how to remove the orphaned `\label{...}` line

[0.2.1-alpha.1]: https://github.com/Abdogouhmad/ltx/compare/v0.2.0...v0.2.1-alpha.1
