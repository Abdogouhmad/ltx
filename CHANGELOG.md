# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Note:** LTX is pre-1.0. Minor versions may contain breaking changes.

## [Unreleased]

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

### Changed
- `outputdir` is no longer supported there is a forced directory called `target/` for better consistency and efficiency
- wrapping the engine configuration inside `[build]` 
- name of toml config is changed from `config.toml` to `ltx.toml`
- `clean` command now has verbose output -v flag or for more verbose output -vv
- the engine default is changed from `pdflatex` to `tectonic`
- changed the out directory from `build/` to `target/`
- changing the way ltx approach the cli context by introducing `AppContext` and `CliCommand`

### Fixed
- Today's date is fixed now during compilation
- `ltx build` now validates `ltx.toml` before compiling, so broken manifests (missing keys, typos, missing main file) fail fast instead of silently falling back to defaults

[unreleased]: https://github.com/Abdogouhmad/ltx/compare/v0.1.0...HEAD
