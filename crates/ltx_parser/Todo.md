# ltx_parser — Implementation Todo

## Status Legend
- `[x]` done
- `[~]` partially done
- `[ ]` not started

---

## 1. Infrastructure (parser core)

- [x] `LtxParser` struct wrapping `TokenStream` (`parser.rs`)
- [x] `Parse` trait definition (`parser_traits.rs`)
- [x] Cursor methods: `peek_kind`, `peek_at`, `bump`, `checkpoint`/`rewind`, `skip_ws`, `at_eof`
- [x] Convenience helpers: `parse::<T>`, `accept`, `expect`
- [x] Error recovery: `skip_to_boundary`
- [x] Error handler access: `error_handler`, `error_handler_mut`
- [x] `utils.rs` — shared helpers (`dummy_span`, `tokens_in_range`, env stack operations)

---

## 2. Existing AST Nodes (basic, low-level)

- [x] **Text** (`ast/text.rs`) — consumes `LtxTokenKind::Text` and error fallbacks
- [x] **Command** (`ast/command.rs`) — consumes `LtxTokenKind::Command` + optional `{...}` and `[...]` args via unified `Arg`
- [x] **Group** (`ast/group.rs`) — consumes balanced `{...}` storing token range (zero clone)

---

## 3. Document-Level AST Nodes

### 3a. Document root

- [x] `Document` — top-level root node
  - Fields: `span`, `preamble: Vec<PreambleItem>`, `body: Vec<DocumentBodyNode>`
  - Parse logic: loops over all tokens; preamble before `\begin{document}`, body after
  - Emits `LTX::E002` if `\begin{document}` is missing
  - Emits `LTX::E102` if `\end{document}` is missing

### 3b. Preamble items

- [x] `PreambleItem` — enum covering preamble constructs:
  - `DocumentClass` — `\documentclass[opts]{class}`
  - `UsePackage` — `\usepackage[opts]{pkg}`
  - `Command` — any other `\command{...}` in the preamble
  - `Text` — stray text in the preamble
  - `Comment` — `%...` comment lines
  - `Group` — `{...}` group

### 3c. Document body nodes

- [x] `DocumentBodyNode` — enum covering body contents:
  - `Environment(...)` — `\begin{env}...\end{env}`
  - `Command(...)` — standalone `\command` or `\command{...}`
  - `Text(...)` — plain text run
  - `Math(...)` — inline `$...$` or display `$$...$$` math
  - `Comment(...)` — `%...` comment
  - `Group(...)` — stray `{...}` group

---

## 4. Environment Node

- [x] `Environment` struct
  - Fields: `span`, `name: &'src str`, `begin_span`, `end_span`, `body: Vec<DocumentBodyNode<'src>>`, `raw_range: Range<usize>`
  - Parse logic: validates environment matching (`LTX::E101` on mismatch, `LTX::E102` on EOF)
- [x] Add environment stack to `LtxParser`
  - `env_stack: Vec<(&'src str, LtxSpan)>`
  - `push_env`, `pop_env`, `current_env`, `env_depth` helper methods

---

## 5. Math Nodes

- [x] `Math` struct
  - Fields: `span`, `delimiter: MathDelimiter`, `tokens: Range<usize>` (zero clone)
  - Parse logic: matches `MathStart` and `MathEnd` with delimiter validation

---

## 6. Comment Node

- [x] `Comment` struct
  - Fields: `span`, `comment_text: &'src str`
  - Parse logic: consumes `LtxTokenKind::Comment`

---

## 7. Preamble-Specific Nodes

- [x] `DocumentClassDecl` struct
  - Fields: `span`, `class_name: &'src str`, `options: Option<OptionalArg<'src>>`
- [x] `UsePackage` struct
  - Fields: `span`, `package_name: &'src str`, `options: Option<OptionalArg<'src>>`

---

## 8. Optional Argument Parsing `[...]`

- [x] Add `[...]` optional argument support (`OptionalArg`)
  - Zero-copy string slice extraction and token index range tracking
  - Used in `\usepackage[opts]{pkg}`, `\documentclass[opts]{class}`, `\section[opts]{title}`

---

## 9. Command Argument Types

- [x] Braced arguments `{...}` (`Group`)
- [x] Optional arguments `[...]` (`OptionalArg`)
- [x] Unified `Arg` enum: `Arg::Braced` and `Arg::Optional`

---

## 10. Top-Level Entry Point

- [x] `parse_document(parser)` convenience function
- [x] Wired up in `lib.rs` and `parser.rs` as public re-exports

---

## 11. Error Recovery & Diagnostics

- [x] `expect` emits `LTX::E001` and calls `skip_to_boundary`
- [x] `Group` emits `LTX::E005` for unterminated groups
- [x] `Document` emits `LTX::E002` for missing `\begin{document}`
- [x] `Environment` emits `LTX::E101`/`LTX::E102` for mismatched/unclosed environments
- [x] Extended `skip_to_boundary` to stop at `BeginEnv`/`EndEnv`

---

## 12. Testing

- [x] Unit and integration tests in `crates/ltx_parser/tests/parser_tests.rs`:
  - Full document parse (`\documentclass` + `\usepackage` + body + `\end{document}`)
  - Environment mismatch and unclosed environment diagnostics
  - Math block parsing (inline vs display)
  - Optional and braced command argument parsing

---

## 13. Examples

- [x] Complete `parser_example.rs` demonstrating full document parse, preamble inspection, AST body tree traversal, math, environments, and diagnostic rendering.
