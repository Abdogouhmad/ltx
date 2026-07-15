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
- [ ] `utils.rs` — currently empty; add shared helpers here as needed (e.g. span merging, optional-argument parsing, bracket `[...]` parsing)

---

## 2. Existing AST Nodes (basic, low-level)

- [x] **Text** (`ast/text.rs`) — consumes `LtxTokenKind::Text`
- [x] **Command** (`ast/command.rs`) — consumes `LtxTokenKind::Command` + optional `{...}` braced `Group` args
- [x] **Group** (`ast/group.rs`) — consumes balanced `{...}` collecting raw tokens

---

## 3. Document-Level AST Nodes (not yet implemented)

### 3a. Document root

- [ ] `Document` — top-level root node
  - Fields: `span`, `preamble: Vec<PreambleItem>`, `body: Vec<DocumentBodyNode>`
  - Parse logic: loop over all tokens; everything before `\begin{document}` is preamble, everything after (until `\end{document}` or EOF) is body
  - Emit `LTX::E002` if `\begin{document}` is never found
  - Emit `LTX::E102` if `\end{document}` is missing

### 3b. Preamble items

- [ ] `PreambleItem` — enum or trait-based, covers preamble-only constructs:
  - `DocumentClassDecl` — `\documentclass{class}`
  - `UsePackage` — `\usepackage[opts]{pkg}`
  - `PreambleCommand` — any other `\command{...}` in the preamble
  - `PreambleText` — stray text/comments in the preamble
  - `PreambleComment` — `%...` comment lines

### 3c. Document body nodes

- [ ] `DocumentBodyNode` — enum covering what can appear in the document body:
  - `Environment(...)` — `\begin{env}...\end{env}`
  - `Command(...)` — standalone `\command` or `\command{...}` (already exists)
  - `Text(...)` — plain text run (already exists)
  - `Math(...)` — inline `$$...$$` or display math
  - `Comment(...)` — `%...` comment
  - `Group(...)` — stray `{...}` group (already exists)
  - `Paragraph` — sequence of inline nodes forming a paragraph (up to blank line or block element)

---

## 4. Environment Node (high priority)

- [ ] `Environment` struct
  - Fields: `span`, `name: &'src str`, `begin_span`, `end_span`, `body: Vec<DocumentBodyNode>`
  - Parse logic:
    1. Expect `LtxTokenKind::BeginEnv(name)` — record `name` and `begin_span`
    2. Push the environment name onto a parser-level env stack
    3. Parse body tokens in a loop until `LtxTokenKind::EndEnv(name)` is seen
    4. Validate name matches; emit `LTX::E101` on mismatch, `LTX::E102` on EOF
    5. Pop env stack
  - Note: the lexer already validates `BeginEnv`/`EndEnv` matching, but the parser should still do its own semantic check and produce its own diagnostics
- [ ] Add environment stack to `LtxParser` (separate from the lexer's env stack)
  - `env_stack: Vec<(&'src str, LtxSpan)>`
  - `push_env`, `pop_env`, `current_env` helper methods

---

## 5. Math Nodes

- [ ] `Math` struct (or `MathInline` / `MathDisplay` variants)
  - Fields: `span`, `delimiter: MathDelimiter`, `body: Vec<DocumentBodyNode>`
  - Parse logic:
    1. Expect `LtxTokenKind::MathStart(delimiter)`
    2. Parse tokens in math body until `LtxTokenKind::MathEnd(delimiter)`
    3. Emit `LTX::E004` if delimiter mismatch or `LTX::E002` on EOF
  - For now, math body can be a flat list of tokens/commands/text; full math grammar refinement comes later

---

## 6. Comment Node

- [ ] `Comment` struct
  - Fields: `span`, `text: &'src str` (the raw comment text including `%`)
  - Parse logic: consume `LtxTokenKind::Comment`
  - Simple; can be attached to following node for doc-comment support later

---

## 7. Preamble-Specific Nodes

- [ ] `DocumentClassDecl` struct
  - Fields: `span`, `class_name: &'src str`
  - Parse logic: consume `LtxTokenKind::DocumentClass(name)`

- [ ] `UsePackage` struct
  - Fields: `span`, `options: Option<Group>`, `package_name: Group`
  - Parse logic: consume `Command` where name is `"usepackage"`, parse optional `[...]` args, then required `{...}` arg
  - Note: optional `[...]` args use `GroupStart`/`GroupEnd` tokens but with `[`/`]` catcodes — **lexer currently does NOT produce bracket-delimited groups as a distinct token kind**, so this needs either:
    - (a) A lexer change to emit `BracketStart`/`BracketEnd` tokens for `[`/`]`
    - (b) Parse `[...]` contents as raw text until `]` is found (simpler, less structured)
  - **Decision needed from you on which approach to take**

---

## 8. Optional Argument Parsing `[...]`

- [ ] Add `[...]` optional argument support
  - Option A: Lexer emits `BracketGroup` tokens (requires lexer change in `ltx_lexer`)
  - Option B: Parser scans text/commands between `[` and `]` at parse time (no lexer change, less structured)
  - This is a prerequisite for: `\usepackage[opts]{pkg}`, `\section[toc]{title}`, `\includegraphics[width=...]{file}`, etc.
- [ ] `OptionalArg` struct (if option B)
  - Fields: `span`, `tokens: Vec<LtxToken<'src>>`
  - Parse logic: expect `[`, collect tokens until `]`, track nesting depth

---

## 9. Command Argument Types

- [x] Braced arguments `{...}` (already handled by `Command` via `Group`)
- [ ] Optional arguments `[...]` (see section 8 above)
- [ ] Distinguish between "required" and "optional" arg counts per command
  - This is more of a semantic pass concern; the parser can just collect args and leave validation to later passes
- [ ] Consider: `Command` should store args as a unified `Vec<Arg>` enum:
  ```rust
  enum Arg<'src> {
      Braced(Group<'src>),
      Optional(OptionalArg<'src>),
  }
  ```

---

## 10. Top-Level Entry Point

- [ ] `parse_document(source) -> Document` convenience function
  - Lexes source → builds `TokenStream` → creates `LtxParser` → calls `parser.parse::<Document>()`
- [ ] Wire up in `lib.rs` as a public re-export

---

## 11. Error Recovery & Diagnostics

- [x] `expect` emits `LTX::E001` and calls `skip_to_boundary`
- [x] `Group` emits `LTX::E005` for unterminated groups
- [ ] `Document` emits `LTX::E002` for missing `\begin{document}`
- [ ] `Environment` emits `LTX::E101`/`LTX::E102` for mismatched/unclosed environments
- [ ] `Math` emits `LTX::E004` for unmatched math delimiters
- [ ] Extend `skip_to_boundary` to also stop at `BeginEnv`/`EndEnv` tokens
- [ ] Consider adding `skip_to_env_end` for recovery inside environments

---

## 12. Testing

- [ ] Unit tests for each AST node's `Parse` impl
  - Happy path (valid input)
  - Error path (missing tokens, unexpected EOF)
- [ ] Integration test: full document parse (`\documentclass` + `\begin{document}` + body + `\end{document}`)
- [ ] Test error recovery: verify diagnostics are emitted and parsing continues
- [ ] Test with real-world LaTeX snippets (sections, itemize, figures, math)

---

## 13. Examples

- [x] Basic `parser_example.rs` (text, command, group)
- [ ] Full document parse example
- [ ] Math parse example
- [ ] Environment parse example

---

## Suggested Implementation Order

1. **Comment node** — trivial, builds momentum
2. **Math node** — self-contained, needed for body
3. **Environment node + parser env stack** — core LaTeX construct
4. **Document root + preamble/body split** — ties everything together
5. **DocumentClassDecl + UsePackage** — preamble specifics
6. **Optional `[...]` arg support** — requires decision on lexer vs parser approach
7. **Refactor `Command.args` to unified `Arg` enum** — once optional args exist
8. **Top-level `parse_document` entry point**
9. **Tests and examples**
10. **`utils.rs` helpers** — fill in as needed during above work
