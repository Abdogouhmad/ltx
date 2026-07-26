# API Overview

Reference for the LTX library crates. These are intended for Rust consumers embedding LTX as a library.

---

## ltx_diagnostics

Unified diagnostic infrastructure — error codes, source spans, and rendering.

### Core types

| Type | Description |
|------|-------------|
| `LtxSourceMap` | Registry of loaded source files. Provides byte-offset → line:column mapping. |
| `LtxSourceFile` | A single loaded source file with its content and file ID. |
| `LtxFileId` | Opaque identifier for a file in the source map. |
| `LtxSpan` | Byte-range location in a specific file (`file_id`, `start`, `end`). |
| `LtxSeverity` | Error / Warning / Hint classification. |
| `LtxError` | A single diagnosable error with code, message, span, and help text. |
| `LtxDiagnostic` | Wraps an `LtxError` with the `LtxSourceMap` needed to render it. |
| `LtxDiagnosticSink` | Accumulates diagnostics across phases for batch reporting. |
| `ErrorCode` | Metadata for a diagnostic code (code string, description, severity). |

### Error code ranges

| Range | Category |
|-------|----------|
| `LTX::E0xx` | Syntax / tokenization (braces, delimiters, escapes) |
| `LTX::E1xx` | Structural / semantic (commands, environments, references) |
| `LTX::W0xx` | Lint warnings (reserved) |

### Rendering

- `render_pretty(diagnostic)` — returns a formatted string for terminal output.
- `render_pretty_into(diagnostic, &mut writer)` — writes formatted output to a buffer.
- `render_json_into(sink, &mut writer)` — serializes diagnostics to JSON.

---

## ltx_lexer

Byte-level tokenizer for LaTeX source files.

### Core types

| Type | Description |
|------|-------------|
| `LtxLexer` | Streaming iterator — call `.next()` or `.next_token()` for one token at a time. |
| `TokenStream` | Eagerly drains `LtxLexer` and provides a cursor API for the parser. |
| `LtxToken` | A single token carrying `LtxTokenKind`, `LtxSpan`, and the source text slice. |
| `LtxTokenKind` | Token classification (command, brace, text, math delimiter, etc.). |
| `LtxMode` | Operating mode: `Normal` or `Math`. |
| `LtxCatCode` | TeX category code (letter, escape, begin-group, etc.). |
| `LtxCatCodeState` | Lookup table for current catcode assignments. |
| `LexerErrorHandler` | Collects `LtxDiagnostic` instances during lexing. |

### TokenStream cursor API

- `peek()` — look at the next token without consuming it.
- `bump()` — consume the current token and advance.
- `checkpoint()` / `rewind()` — save and restore cursor positions for speculative parsing.

---

## ltx_parser

Recursive-descent parser that consumes a `TokenStream` and produces an AST.

### Core types

| Type | Description |
|------|-------------|
| `LtxParser` | Wraps a `TokenStream` and exposes `peek`, `bump`, `checkpoint`/`rewind`, `skip_ws`. |
| `Parse` | Trait implemented by every AST node type. |

### AST nodes

| Node | Description |
|------|-------------|
| `Document` | Top-level root node containing preamble and body. |
| `PreambleItem` | Preamble items (`\documentclass`, `\usepackage`, etc.). |
| `DocumentBodyNode` | Body and environment items. |
| `DocumentClassDecl` | `\documentclass` declaration. |
| `UsePackage` | `\usepackage` declaration. |
| `Command` | Control sequence with its arguments. |
| `Arg` / `OptionalArg` | Required and optional argument variants. |
| `Environment` | `\begin{...}...\end{...}` block. |
| `Group` | Balanced `{...}` group. |
| `Math` | Math expression. |
| `Text` | Plain text run. |
| `Comment` | LaTeX comment. |

### Entry point

```rust
use ltx_parser::{LtxParser, parse_document};

let doc = parse_document(&mut parser);
```

---

## ltx_config

Configuration and project scaffolding.

### Core types

| Type | Description |
|------|-------------|
| `LtxManifest` | Top-level `config.toml` structure. |
| `Project` | `[project]` table — name, version, author, main file path. |
| `Engine` | `[engine]` table — compiler and extra args. |
| `CompilerEngine` | Enum: `PdfLaTeX`, `XeLaTeX`, `LuaLaTeX`, `Tectonic`. |
| `Build` | `[build]` table — output name and directory. |
| `ScaffoldOptions` | Options for project generation (`name`, `engine`, `src`, `bib`). |
| `SrcLayout` | `Flat` or `WithSrcDir`. |
| `BibLayout` | `Flat` or `WithBibDir`. |

### Functions

```rust
// Create a manifest and write it to disk
let manifest = LtxManifest::new(project, engine);
manifest.write("config.toml")?;

// Read a manifest from disk
let manifest = LtxManifest::from_file("config.toml")?;

// Scaffold a new project
scaffold(&project_dir, &scaffold_opts)?;
```

---

## ltx_utils

Low-level filesystem helpers.

| Function | Description |
|----------|-------------|
| `create_dir(path)` | Creates a directory, including all missing parents. |
| `create_file(path)` | Creates a file, including parent directories. |
| `write_file(path, contents)` | Writes contents to a file, creating parent dirs as needed. |
| `resolve_main_file(path)` | Returns the path to the main entry file. Defaults to `main.tex`. |
