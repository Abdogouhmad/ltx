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

### Changed
- `outputdir` is no longer supported there is a forced directory called `target/` for better consistency and efficiency
- wrapping the engine configuration inside `[build]` 
- name of toml config is changed from `config.toml` to `ltx.toml`
- `clean` command now has verbose output -v flag or for more verbose output -vv
- the engine default is changed from `pdflatex` to `tectonic`
- changed the out directory from `build/` to `target/`
- changing the way ltx approach the cli context by introducing `AppContext` and `CliCommand`

[unreleased]: https://github.com/Abdogouhmad/ltx/compare/v0.1.0...HEAD
