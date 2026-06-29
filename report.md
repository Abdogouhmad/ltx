# ltx_lexer — Code Audit Report

**Audit date:** 2026-06-29  
**Location:** `crates/ltx_lexer/`  
**Files analyzed:** 13 (4 source modules, 3 examples, 1 test, 1 bench, 1 todo, 1 Cargo.toml, 1 lib.rs)

---

## 1. CRITICAL: Lexer is >80% Unfinished

`crates/ltx_lexer/src/lexer.rs` only implements **3 of 16 token types**:

| Implemented | Not Implemented |
|---|---|
| `WhiteSpace` | `DocumentClass(String)` |
| `EOL` | `Command(String)` |
| `Comment` | `BeginEnv(String)`, `EndEnv(String)` |
| | `Text(String)`, `Math(String)` |
| | `Verbatim(String)`, `VerbatimStart` |
| | `Parameter(String)`, `Active(char)` |
| | `GroupStart`, `GroupEnd` |
| | `Error(String)` |

**No `Iterator<Item = LtxToken>` impl** — users must manually loop with `peek()`/`bump()`.  
**No mode-aware tokenization** — `LtxMode` field is declared (line 24) but never read or written by any method.  
**No `\n\n` paragraph-break detection** despite being in `todo.md`.

> **Action:** Add scan methods for all remaining `LtxTokenKind` variants. Implement `Iterator`. Wire up `LtxMode` to affect tokenization (e.g., verbatim mode disables escape processing, math mode treats `$` as delimiter).

---

## 2. HIGH: `#[repr(u8)]` on Data-Carrying Enum

`crates/ltx_lexer/src/token.rs:18`

```rust
#[repr(u8)]
pub enum LtxTokenKind {
    DocumentClass(String) = 1,
    Command(String) = 2,
    // ...
}
```

`#[repr(u8)]` on an enum with `String` / `char` payloads compiles in modern Rust but is **misleading and dangerous**. If any future code adds `unsafe` transmute to `u8`, this will produce **undefined behavior** because the payload data overlaps with the discriminant.

> **Action:** Remove `#[repr(u8)]` unless the enum is refactored to be fieldless (which would require storing payload data separately).

---

## 3. HIGH: Per-Token Heap Allocations

`crates/ltx_lexer/src/token.rs:12` — `text: String` forces a heap allocation for **every token**, including single-character ones like `{`, `}`, `$`.

`crates/ltx_lexer/src/lexer.rs:124` — `consumed_source_text()` calls `.to_string()` on every scan.

> **Action:** Change `LtxToken.text` to `&'source str` (requires lifetime parameter on `LtxToken`) or use `Cow<'source, str>`. This eliminates all per-token allocations.

---

## 4. HIGH: Dead Code — `from_source_map()`

`crates/ltx_lexer/src/errors_core.rs:42-49`

```rust
pub const fn from_source_map(file_id: LtxFileId, source_map: Arc<LtxSourceMap>) -> Self {
    // identical to new()
}
```

This is a line-for-line duplicate of `new()` (lines 30-37). The doc comment says "from a mutable source map" but takes `Arc<LtxSourceMap>` (immutable). Never called anywhere in the codebase.

> **Action:** Remove `from_source_map()`. If a separate constructor is needed for clarity, give it a distinct signature.

---

## 5. HIGH: `scan_comment()` Uses Catcode-Based EOL Detection

`crates/ltx_lexer/src/lexer.rs:180`

```rust
if self.catcode.get(ch) == LtxCatCode::EndOfLine {
    break;
}
```

In TeX, comments always end at the **physical newline character**, regardless of catcode reassignment. If `\n` is re-categorized at runtime, comments will never terminate.

> **Action:** Check for `'\n'` directly instead of using catcode lookup for line-ending detection in `scan_comment()`.

---

## 6. MEDIUM: Catcode Table Gaps

### `\r` (Carriage Return) Not Mapped
`crates/ltx_lexer/src/catcode.rs:98` — Only `\n` is mapped to `EndOfLine`. On CR+LF or CR-only files, `\r` has catcode `Other` (12).

### `\0` (Null Byte) Not Mapped
Standard TeX maps the null character to catcode 9 (`Ignored`). It is absent from `default_tex()`.

### Missing Explicit Arm for Value 12 in `get()`
`crates/ltx_lexer/src/catcode.rs:146` — The wildcard `_ => LtxCatCode::Other` catches value 12 by coincidence, but also silently corrupts any values >15 that might end up in the array due to bugs.

> **Action:** Add `map[b'\r' as usize] = LtxCatCode::EndOfLine.as_u8()` and `map[0] = LtxCatCode::Ignored.as_u8()` to `default_tex()`. Add an explicit arm `12 => LtxCatCode::Other` in `get()` and move the wildcard to `_ => unreachable!()` or a panic.

---

## 7. MEDIUM: `set()` Silently Ignores Characters ≥ U+00FF

`crates/ltx_lexer/src/catcode.rs:159-164`

```rust
pub const fn set(&mut self, c: char, cat: LtxCatCode) {
    let byte = c as u32;
    if byte < 256 {
        self.map[byte as usize] = cat.as_u8();
    }
}
```

Setting catcodes for Unicode characters beyond Latin-1 silently does nothing.

> **Action:** Add `debug_assert!` or return `Result` to surface this limitation.

---

## 8. MEDIUM: `take_errors()` Silently Drops Non-Lexer Diagnostics

`crates/ltx_lexer/src/errors_core.rs:103-112`

```rust
for diag in other {
    if let LtxDiagnosticInner::Lexer(err) = diag.inner {
        errors.push(err);
    }
}
```

Any `ParserError` or other diagnostic variants in `other_diagnostics` are silently discarded.

> **Action:** Log a warning or return those as a separate collection. At minimum document the behavior.

---

## 9. MEDIUM: Inconsistent `source_map` Ownership

- `LtxLexer::new()` (lexer.rs:40) takes `LtxSourceMap` by value and wraps in `Arc` internally.
- `LexerErrorHandler::new()` (errors_core.rs:30) takes `Arc<LtxSourceMap>` directly.

This forces callers to juggle two patterns.

> **Action:** Pick one convention and use it consistently.

---

## 10. MEDIUM: `peek()` / `peek_nth()` Create `Chars` Iterator Per Call

`crates/ltx_lexer/src/lexer.rs:67-76`

```rust
pub fn peek_nth(&self, offset: usize) -> Option<char> {
    self.source[self.cursor..].chars().nth(offset)
}
pub fn peek(&self) -> Option<char> {
    self.source[self.cursor..].chars().next()
}
```

On every call a new `Chars` iterator is created from a slice. In hot loops this adds measurable overhead.

> **Action:** Cache the current `char` and byte offset in the struct, or use `.as_bytes()` with byte-level indexing (safe for ASCII-heavy TeX source).

---

## 11. LOW: Misleading Doc Comments

| File:Line | Issue |
|---|---|
| `token.rs:11` | `text` doc says "Represents the text of `main.tex`" — should say "the source text of this token" |
| `errors_core.rs:40` | `from_source_map()` doc says "from a mutable source map" but param is `Arc<LtxSourceMap>` (immutable) |
| `lexer.rs` | No module-level doc describing which token types are actually implemented |

---

## 12. LOW: `errors_core.rs:len()` Conflates Errors & Other Diagnostics

`crates/ltx_lexer/src/errors_core.rs:75`

```rust
pub fn len(&self) -> usize {
    self.errors.len() + self.other_diagnostics.len()
}
```

Callers expecting just the error count get an inflated number.

> **Action:** Rename to `total_count()` or add separate `error_count()` / `diagnostic_count()` methods.

---

## 13. Testing Gaps

| Gap | Detail |
|---|---|
| **No lexer tests** | Zero tests for `LtxLexer::scan_whitespace`, `scan_eol`, `scan_comment`, `peek`, `bump`, `slice`, `is_eof`, `lexer_span` |
| **No error handler tests** | Zero tests for `LexerErrorHandler` |
| **Missing catcode variant in test** | `tests/catcode_test.rs:45-61` — `AlignmentTab` (value 4) excluded from `from_u8` round-trip test |
| **Redundant test** | `test_get_catcodestate` (line 30) is a subset of `test_default_catcode_state` (line 7) |
| **No integration tests** | No end-to-end lexer tests with real TeX input |
| **No edge case tests** | Empty source, unicode > U+00FF, `\r\n`, `\r`-only, consecutive commands, malformed input, cursor overflow |
| **No snapshot tests** | Mentioned in `todo.md` with `insta` + fixtures — none exist |

---

## 14. Summary of Recommendations

| Priority | Area | Recommendation |
|---|---|---|
| Critical | lexer.rs | Implement remaining 13 scan methods + `Iterator` + mode switching |
| High | token.rs | Remove `#[repr(u8)]` from data-carrying enum |
| High | token.rs / lexer.rs | Use `&'source str` instead of `String` for `LtxToken.text` |
| High | errors_core.rs | Delete dead `from_source_map()` duplicate |
| High | lexer.rs:180 | Use literal `'\n'` check in `scan_comment()`, not catcode |
| Medium | catcode.rs | Map `\r` → `EndOfLine` and `\0` → `Ignored` in default table |
| Medium | catcode.rs | Add explicit arm for value 12 in `get()` |
| Medium | catcode.rs | Surface errors when `set()` ignores non-Latin-1 chars |
| Medium | errors_core.rs | Don't silently discard non-lexer diagnostics in `take_errors()` |
| Medium | lexer.rs | Cache characters to avoid `Chars` iterator creation per call |
| Low | various | Fix misleading doc comments |
| Low | errors_core.rs | Rename `len()` or add separate counting methods |
| Low | tests/catcode_test.rs | Add missing `AlignmentTab` test, remove redundant test |
| Gap | tests/ | Add lexer, error handler, integration, edge case, and snapshot tests |
