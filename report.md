# `crates/ltx_lexer` — Improvement Analysis

**Goal:** Identify dead code, inconsistencies, design issues, and missing features that should be addressed before shipping.

---

## 1. Dead Code — 2 Unused Functions

### `scan_escape()` — `lexer_utils.rs:345`

Defined but never called. The `Escape` catcode in `next_normal_token()` routes to `scan_command()`, not `scan_escape()`. If the intent is that `\` followed by a non-letter should return an `Escape` token, the dispatcher needs fixing. Otherwise, delete this function.

### `scan_control_symbol()` — `lexer_utils.rs:229`

Defined but never called. Control symbols are handled inline in `scan_command()` (line 443–452), duplicating the logic. Delete the function or use it from `scan_command()`.

---

## 2. Inconsistencies

### 2a. Three `$` Catcode Variants, Only One Used

`InlineMathStart` (value 3), `InlineMathEnd` (value 4), `MathShift` (value 5) are all defined. Only `MathShift` (5) is assigned in `default_tex()`. Values 3 and 4 are wasted enum slots. Either:
- Remove `InlineMathStart` and `InlineMathEnd` (the start/end distinction is lexer state, not catcode), **or**
- Assign them meaningfully (e.g., map `$` to `MathShift` only, as currently).

### 2b. `error_count()` vs `has_errors()` Mismatch

```rust
pub fn has_errors(&self) -> bool {
    !self.errors.is_empty()
        || self.other_diagnostics.iter().any(|d| d.severity() == Error)
}

pub fn error_count(&self) -> usize { self.errors.len() }  // ignores other_diagnostics
```

If a diagnostic with `Error` severity is pushed via `push_diagnostic()`, `has_errors()` returns `true` but `error_count()` returns `0`. Either `error_count()` should count `other_diagnostics` items with Error severity, or `has_errors()` should only check `self.errors`.

### 2c. `Default` Impl for `LexerErrorHandler`

```rust
impl Default for LexerErrorHandler {
    fn default() -> Self {
        let source_map = Arc::new(LtxSourceMap::new());
        Self::new(LtxFileId(0), source_map)
    }
}
```

Creates a dummy `LtxFileId(0)` and empty `LtxSourceMap`. Useful for tests but panics if `take_diagnostics()` is called without a real source map backing the spans. Document this or gate behind `#[cfg(test)]`.

---

## 3. Unicode Limitation — Latin-1 Only

```rust
pub const fn get(&self, c: char) -> LtxCatCode {
    if (c as u32) >= 256 { return LtxCatCode::Other; }
    ...
}
```

All Unicode characters > U+00FF (e.g., `é`, `ñ`, `中文`, `日本語`) are classified as `Other`, **not** `Letter`. This means commands like `\café` would break at `é` instead of treating it as part of the command name. For a real-world LaTeX lexer, the catcode table should support Unicode letters or use `char::is_alphanumeric()` as a fallback in the `Letter` check.

### Impact
- `\usepackage[utf8]{inputenc}` with accented chars in command names → broken lexing
- `is_letter()` returns `false` for any non-ASCII letter
- `set()` silently ignores non-Latin-1 input

---

## 4. Missing Trailing-Space Eating After Control Words

Per LaTeX convention, control words (letter-based commands like `\LaTeX`, `\section`) swallow subsequent whitespace. This is not implemented — the space after `\LaTeX ` would produce a separate `WhiteSpace` token. Add space eating in `scan_command()` after a control word.

---

## 5. Incomplete Mode Implementations

```rust
pub fn next_token(&mut self) -> Option<LtxToken<'lxr>> {
    match self.mode {
        LtxMode::Normal => self.next_normal_token(),
        LtxMode::Math => todo!(),       // not implemented
        LtxMode::Verbatim => todo!(),   // not implemented
    }
}
```

If any `$` or `$$` is encountered, mode flips to `Math`, and the **next call** to `next_token()` panics with `todo!()`. The lexer is effectively unusable for any document containing math mode. These should at minimum be non-panicking stubs that skip content and return to Normal.

---

## 6. `\end` Validation Edge Cases

### 6a. Mismatched env name still leaves old env on stack

In `scan_end()`, when `env != expected`, the error is returned but `pop_env()` is **not** called. The old env remains on the stack, causing cascading errors on subsequent `\end` calls.

### 6b. No document-level env tracking

`\begin{document}` is pushed but there's no special handling for the implicit document environment. If the user writes `\end{document}` with nothing on the stack, they get `\end{document} has no matching \begin` — which is technically correct but confusing because LaTeX documents have an implicit group.

---

## 7. `scan_env_name_optional()` — Silent Failure

When `{` is missing or the closing `}` is not found, the function returns `None` without emitting any error. The callers (`scan_begin`, `scan_end`, `scan_documentclass`) each emit their own error, but:

```rust
// In scan_documentclass:
} else {
    self.error_handler.unexpected_token('\\', start, self.cursor);
    self.error_token(start, "Expected \\documentclass{...}")
}
```

The error message says "Expected `\documentclass{...}`" but the actual problem is unclosed/missing braces. The error message is misleading.

---

## 8. `Iterator` — No `size_hint()`

```rust
impl<'lxr> Iterator for LtxLexer<'lxr> {
    type Item = LtxToken<'lxr>;
    fn next(&mut self) -> Option<Self::Item> { self.next_token() }
}
```

No `size_hint()` override. For known source length, an upper bound of `source.len()` (one token per byte, worst case) is trivial to provide and enables optimizations in collect/adaptor chains.

---

## 9. `Comment` Scanner Checks Catcode Instead of `'\n'`

```rust
while let Some(ch) = self.peek() {
    if self.catcode.get(ch) == LtxCatCode::EndOfLine { break; }
    let _ = self.bump();
}
```

This is technically correct and flexible (respects catcode changes), but in practice comments always end at `\n`. The function could also be `\\r`-aware (though `scan_eol` handles `\r\n`/`\r`/`\n`). If catcodes never change during lexing, a direct `ch == '\n'` check is simpler and faster.

---

## 10. Summary of Recommendations

| Priority | Issue | Fix |
|---|---|---|
| **High** | `next_token()` panics on Math/Verbatim mode via `todo!()` | Replace with non-panicking stubs that return to Normal |
| **High** | Unicode > U+00FF never classified as `Letter` | Extend `is_letter()` with `char::is_alphabetic()` fallback |
| **Medium** | `scan_escape()` and `scan_control_symbol()` dead code | Delete or wire up |
| **Medium** | `error_count()` vs `has_errors()` inconsistency | Align counting logic |
| **Medium** | No trailing-space eating after control words | Add to `scan_command()` |
| **Medium** | `\end` mismatch doesn't pop env | Pop after mismatch to avoid cascading errors |
| **Low** | Three `$` variants, one used | Remove unused `InlineMathStart`/`InlineMathEnd` |
| **Low** | `Default` for `LexerErrorHandler` has dummy IDs | Document or gate behind `#[cfg(test)]` |
| **Low** | No `Iterator::size_hint()` | Add upper bound |
| **Low** | `Comment` scanner checks catcode per char | Direct `'\n'` check if catcodes don't mutate |
