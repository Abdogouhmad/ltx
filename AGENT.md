# AGENT.md

> Instructions for AI agents and contributors working on this project.

---

# Project Overview

## Purpose

LTX is a Rust implementation of a LaTeX project manager and compiler.

Example:

This project is a Rust implementation of a LaTeX project manager and compiler.
The workspace consists of multiple crates responsible for parsing, diagnostics,
compilation, and the CLI.

## Goals

- Fast
- Memory efficient
- Safe
- Well documented
- Minimal dependencies


---

# Workspace Layout

```
.
├── crates/
│   ├── ltx_cli/
│   ├── ltx_parser/
│   ├── ltx_lexer/
│   ├── ltx_config/
│   ├── ltx_utils/
│   ├── ltx_diagnostics/
│   └── ...
├── examples/
├── tests/
└── docs/
```

Describe each crate in one sentence.

| Crate | Responsibility |
|--------|----------------|
| ltx_cli | CLI interface |
| ltx_parser | Parse AST |
| ltx_lexer | Tokenization |
| ltx_config | Configuration |
| ltx_utils | Utility functions |
| ltx_diagnostics | Errors and spans |

---

# Design Principles

## Keep APIs small

Prefer small focused APIs over generic abstractions.

## Zero-cost abstractions

Avoid unnecessary allocations.

## Panic-free library

Library code should return `Result`.
Panics are bugs.

## Ownership

Avoid cloning unless required.

Prefer borrowing.

---

# Coding Style

## General

- Follow rustfmt.
- Follow clippy.
- Prefer explicit names.
- Avoid abbreviations.

Good

```rust
source_file
diagnostic
compiler
```

Bad

```rust
srcf
diag
cmp
```

---

## Functions

Functions should usually do one thing.

Prefer

```rust
parse_argument()
```

instead of

```rust
parse_argument_and_validate_and_emit_errors()
```

---

## Comments

Document why.

Avoid comments describing obvious code.

Good

```rust
// TeX ignores spaces after control words.
```

Bad

```rust
// Increment i
i += 1;
```

---

## Documentation

Every public item should have documentation.

Include

- Purpose
- Arguments
- Errors
- Examples (when useful)

---

# Error Handling

Never ignore errors.

Prefer

```rust
Result<T, Error>
```

over

```rust
Option<T>
```

unless absence is expected.

Do not use

```rust
unwrap()
expect()
```

inside library code.

---

# Performance

Before introducing allocations ask:

- Is it necessary?
- Can it borrow?
- Can it use Cow?
- Can it reuse buffers?

Avoid

- unnecessary cloning
- temporary Strings
- repeated allocations

---

# Testing

Every new parser feature should include

- valid example
- invalid example
- edge cases

Prefer unit tests over integration tests unless testing crate interaction.

---

# Parser Rules

The parser must

- never panic on invalid input
- recover where possible
- produce diagnostics
- preserve spans

---

# Lexer Rules

The lexer must

- never allocate tokens unnecessarily
- preserve byte offsets
- never lose source information

---

# AST Rules

AST nodes should

- own only required data
- preserve spans
- avoid duplicated information

---

# Dependencies

Before adding a dependency ask

1. Can std do this?
2. Can an existing dependency do this?
3. Is this dependency actively maintained?
4. Does it increase compile time?

Adding dependencies requires justification.

---

# Public API

Public APIs should be

- stable
- documented
- easy to discover

Avoid exposing implementation details.

---

# Pull Request Checklist

Before finishing work ensure

- [ ] cargo fmt
- [ ] cargo clippy
- [ ] cargo test
- [ ] documentation updated
- [ ] no unnecessary allocations
- [ ] no unwrap in library code
- [ ] new behavior tested

---

# AI Agent Guidelines

When modifying code:

1. Understand the surrounding module first.
2. Preserve the existing architecture.
3. Prefer consistency over cleverness.
4. Do not introduce unnecessary abstractions.
5. Explain significant design changes.
6. Do not remove comments unless replacing them with better ones.
7. Keep commits focused on one logical change.
8. If unsure, ask rather than guessing.

When writing Rust:

- Prefer iterators when they improve readability.
- Prefer explicit types in public APIs.
- Minimize allocations.
- Avoid unsafe unless absolutely required.
- Keep modules cohesive.

---

# References

- Rust API Guidelines
- Rustonomicon
- Rust Reference
- The Rust Book
