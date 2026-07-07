# LTX Parser Implementation Roadmap

This roadmap outlines the plan for building a robust, high-performance, and error-resilient parser for LaTeX documents (`ltx_parser`), building upon the zero-allocation `ltx_lexer` and the improved `ltx_diagnostics` crate.

---

## 1. Objectives & Guidelines

- **Zero-Allocation Parsing:** The AST (Abstract Syntax Tree) must use lifetimes to borrow text slice representations (`&'a str`) directly from the original source files, avoiding unnecessary `String` allocations.
- **Rich Diagnostics & Spans:** Every AST node must preserve its `LtxSpan` to power rich, localized diagnostics through `miette` and `ltx_diagnostics`.
- **Resilient Error Recovery:** The parser should not panic on invalid syntax. It must recover gracefully from syntax errors (e.g. skip to the next environment or command boundary) to collect and report multiple errors in a single compiler run.
- **Test-Driven Design:** Standardize unit tests and integration tests using snapshot testing with the `insta` crate.

---

## 2. Phase 1: AST (Abstract Syntax Tree) Design

Define the core node structures in `crates/ltx_parser/src/ast.rs`:

```rust
pub struct Ast<'a> {
    pub root: Node<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'a> {
    Document(DocumentNode<'a>),
    Preamble(PreambleNode<'a>),
    Body(BodyNode<'a>),
    Text(&'a str, LtxSpan),
    Command(CommandNode<'a>),
    Environment(EnvironmentNode<'a>),
    Group(GroupNode<'a>),
    Math(MathNode<'a>),
    Comment(&'a str, LtxSpan),
}
```

### Key AST Structs
- `DocumentNode<'a>`: Holds the preamble node and the body node.
- `CommandNode<'a>`: Represents a macro command (e.g., `\name`). Includes `name: &'a str`, optional arguments `Vec<Node<'a>>`, and mandatory arguments `Vec<Node<'a>>`.
- `EnvironmentNode<'a>`: Represents `\begin{env} ... \end{env}`. Contains `name: &'a str`, options, and children node lists.
- `MathNode<'a>`: Inline (`$`) or Display (`$$`) math nodes holding math content.

---

## 3. Phase 2: Parser Architecture

Implement the parser state machine in `crates/ltx_parser/src/parser.rs`.

### Parser State
```rust
pub struct LtxParser<'a> {
    lexer: LtxLexer<'a>,
    current_token: Option<LtxToken<'a>>,
    peek_token: Option<LtxToken<'a>>,
    diagnostics: LtxDiagnosticSink,
}
```

### Core Parsing Routines
1. **Preamble Parsing:** Consume up to `\begin{document}`, parsing command definitions, packages, and metadata.
2. **Body Parsing:** Process the content inside the `document` environment.
3. **Environment Dispatcher:** When encountering `\begin{env}`, match with `\end{env}` and build child scopes recursively.
4. **Command & Group Parsing:** Handle macro calls (including optional bracket arguments `[...]` and mandatory brace arguments `{...}`).

---

## 4. Phase 3: Error Recovery Strategies

To ensure a smooth compiler experience, error recovery is paramount:

- **Missing Braces:** If a mandatory block is missing the closing `}`, consume tokens until the next control sequence, comment, or new paragraph is hit. Emit an `LTX::E105` or `LTX::E106` diagnostic and continue.
- **Unclosed Environments:** If `\end{env}` is missing at EOF, bubble up the error pointing to the corresponding `\begin{env}` location span.
- **Mismatched Environments:** If `\end{env_b}` closes `\begin{env_a}`, report a mismatched tag diagnostic. Automatically pop the mismatched environment and proceed as if it was closed to parse subsequent nodes correctly.

---

## 5. Phase 4: Testing & Snapshots

- **Unit Testing:** Write parsing tests inside `crates/ltx_parser/tests/`.
- **Snapshot Testing:** Integrate `insta` snapshot tests to verify structural parsing results of complex LaTeX files:
  ```rust
  #[test]
  fn test_sample_document() {
      let source = r"\documentclass{article}\begin{document}Hello World\end{document}";
      let ast = parse(source);
      insta::assert_debug_snapshot!(ast);
  }
  ```

---

## 6. Implementation Roadmap Timeline

1. **Step 1:** Define the AST nodes with proper lifetimes (`crates/ltx_parser/src/ast.rs`).
2. **Step 2:** Implement the basic recursive descent parsing loops (`crates/ltx_parser/src/lib.rs`).
3. **Step 3:** Implement macro arguments and nesting groups.
4. **Step 4:** Integrate parsing error diagnostics into `ParserError` and connect `LtxDiagnosticSink`.
5. **Step 5:** Add regression snapshot tests.
