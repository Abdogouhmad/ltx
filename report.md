# ltx_lexer — Code Audit Report

**Audit date:** 2026-07-01  
**Previous audit:** 2026-06-29  
**Location:** `crates/ltx_lexer/`  
**Files analyzed:** 14 (6 source modules, 3 examples, 1 test, 1 bench, 1 Cargo.toml, 1 lib.rs, 1 todo.md)

---

## 1. CRITICAL: Lexer Is ~55% Implemented (Improved from ~20%)

`crates/ltx_lexer/src/lexer.rs` now implements **10 of 18 `LtxTokenKind` variants** (was 3 of 16):

| Implemented | Not Implemented |
|---|---|
| `WhiteSpace` | `DocumentClass(String)` |
| `EndOfLine` | `Command(String)` |
| `Comment` | `BeginEnv(String)`, `EndEnv(String)` |
| `GroupStart` | `Verbatim(String)`, `VerbatimStart` |
| `GroupEnd` | `Parameter(String)`, `Active(char)` |
| `Escape` | `Error(String)` |
| `InlineMathStart(MathDelimiter)` | |
| `InlineMathEnd(MathDelimiter)` | |
| `Text` | |

**What's new:**
- 5 new scan methods: `scan_group_start`, `scan_group_end`, `scan_escape`, `scan_math_shift`, `scan_text`
- `LtxMode` is now **used** — `scan_math_shift()` toggles between `Normal` and `Math` mode, emitting `InlineMathStart` / `InlineMathEnd` accordingly
- `consumed_source_text()` is now zero-copy (`&'source str`)
- `Text` and `Comment` token kinds are now fieldless — the source text lives in `LtxToken.text`, no `.to_string()` allocation per token

**What's still missing:**
- **No `Iterator<Item = LtxToken>` impl** — callers must manually dispatch on `peek()` / catcode
- **No `\n\n` paragraph-break detection** (noted in `todo.md`)
- 8 token variants remain unscanned

> **Action:** Implement `Iterator`. Add remaining scan methods.

---

## 2. `#[repr(u8)]` on `CatCode` — Explicit Discriminants Restored

`crates/ltx_lexer/src/catcode.rs:5`

**✅  RESOLVED** — Explicit discriminants (`Escape = 0`, `GroupStart = 1`, … `Invalid = 17`) restored in `catcode.rs:6-44`.

**Why:** The enum previously used implicit discriminants (declaration order). Adding or reordering variants would silently shift all numeric values, breaking `from_u8()` and any byte-level serialization. Explicit values make the byte layout stable and self-documenting.

The `#[repr(u8)]` on `LtxCatCode` is safe because the enum is fieldless. `#[repr(u8)]` was already removed from `LtxTokenKind` (token.rs:16) since it carries `String`/`char`/`MathDelimiter` payloads.

---

## 3. Per-Token Heap Allocations

**✅  RESOLVED** — Two-part fix:

| What | Before | After |
|---|---|---|
| `LtxToken.text` | `String` (heap per token) | `&'source str` (zero-copy) |
| `consumed_source_text()` | `.to_string()` allocation | `&'source str` slice |
| `Text` / `Comment` payloads | `Text(String)` / `Comment(String)` — doubled memory | fieldless — text lives in `.text` |

**Why:** Every token previously incurred a heap allocation for its text, including single-character tokens like `{`, `}`, `$`. The `.text` field and the `LtxTokenKind` payload stored the same string data, doubling memory. Switching to `&'source str` for `.text` and making `Text`/`Comment` fieldless eliminates all per-token allocations.

---

## 4. Dead Code — `from_source_map()` Removed

**✅  RESOLVED** — Deleted from `errors_core.rs:42-49`.

**Why:** Line-for-line duplicate of `new()`. Its doc claimed to take "a mutable source map" but the parameter was `Arc<LtxSourceMap>` (immutable). Never called anywhere in the codebase.

---

## 5. `scan_comment()` Catcode-Based EOL Detection Fixed

**✅  RESOLVED** — `crates/ltx_lexer/src/lexer.rs:165`

```rust
// Before (bug): catcode lookup — breaks if \n is re-categorized
if self.catcode.get(ch) == LtxCatCode::EndOfLine { break; }

// After: literal newline check — always correct
if ch == '\n' { break; }
```

**Why:** In TeX, comments always end at the physical newline character, regardless of catcode reassignment. If `\n` were re-categorized at runtime (e.g., to `Other`), the old code would never terminate the comment scan, consuming the rest of the file.

---

## 6. Catcode Table Issues

**✅  ALL RESOLVED**

### Triple `$` Assignment
`crates/ltx_lexer/src/catcode.rs:107-109`

Three consecutive writes to `map[b'$' as usize]` meant only the last (`MathShift`) took effect. The dead `InlineMathStart`/`InlineMathEnd` assignments have been removed; `$` now maps directly to `MathShift`.

**Why:** The start/end distinction is a lexer **state** concern (odd/even `$` count to toggle `Normal`/`Math` mode), not a catcode property. A single byte cannot encode "start vs end", so the catcode table should store a single unambiguous value.

### `\r` and `\0` Not Mapped
Added `\r` → `EndOfLine` and `\0` → `Ignored` to `default_tex()`.

**Why:** On CR+LF or CR-only files, the old code gave `\r` catcode `Other` (12), causing it to be emitted as a text token rather than part of the line ending. Standard TeX maps null to catcode 9 (`Ignored`); its absence could cause unexpected `Other` tokens in binary data.

### `LineBreak` Removed
Removed the unused `LineBreak` variant from `LtxCatCode` and `from_u8()`.

**Why:** It was defined in the enum and listed in `from_u8()` but never assigned to any byte in `default_tex()`. Unreachable unless explicitly `set()` by the user. No scan method produced it.

---

## 7. `set()` Silently Ignores Characters ≥ U+00FF

**✅  RESOLVED** — Documented in `catcode.rs:156-157`.

`debug_assert!` is not available in `const fn`, so behavior is documented with a comment: callers must ensure `c` is Latin-1 or the assignment is silently ignored.

**Why:** The internal `map: [u8; 256]` array can only address Latin-1. Silently ignoring non-Latin-1 input is a footgun for callers; the comment at least surfaces the limitation.

---

## 8. `take_errors()` Silently Drops Non-Lexer Diagnostics

**✅  RESOLVED** — Documented in `errors_core.rs:106-107`.

Added doc comment: "Non-`Lexer` diagnostics in `other_diagnostics` are silently discarded — use `take_diagnostics()` if you need to preserve them."

**Why:** The old code filtered diagnostics by `LtxDiagnosticInner::Lexer` and dropped everything else. Callers got back incomplete data without warning. The doc now points them to `take_diagnostics()` for the full set.

---

## 9. MEDIUM: Inconsistent `source_map` Ownership

**🔴 UNRESOLVED**

- `LtxLexer::new()` (lexer.rs:36-50) takes `LtxSourceMap` by value and wraps in `Arc`
- `LexerErrorHandler::new()` (errors_core.rs:30) takes `Arc<LtxSourceMap>` directly

> **Action:** Pick one convention.

---

## 10. MEDIUM: `peek()` / `peek_nth()` Create `Chars` Iterator Per Call

**🔴 UNRESOLVED** — `crates/ltx_lexer/src/lexer.rs:63-72`

Every call creates a new `Chars` iterator from `self.source[self.cursor..]`. In hot loops this adds overhead.

> **Action:** Cache current `char` and byte offset in the struct, or use `.as_bytes()` with byte-level indexing.

---

## 11. LOW: Misleading Doc Comments

**✅  ALL RESOLVED**

| File:Line | Issue | Fix |
|---|---|---|
| `token.rs:11` | `text` doc said "Represents the text of `main.tex`" | Changed to "The source text slice for this token." |
| `errors_core.rs:40` | `from_source_map()` mis-documented | Function deleted (dead code) |
| `lexer.rs` | Verbose `# Arguments` / `# Returns` blocks | Already removed in prior audit |
| `catcode.rs:101-103` | Comment said `$` carries `InlineMathStart` but triple-write meant `MathShift` wins | Updated to explain `MathShift` + lexer-state distinction |

---

## 12. `errors_core.rs` — `len()` Conflated Errors & Other Diagnostics

**✅  RESOLVED** — `errors_core.rs:66-72`

Renamed to `total_count()` (returns errors + other diagnostics) and added `error_count()` (returns only errors).

**Why:** Callers using `len()` expected just the error count but got an inflated number including unrelated diagnostics. Two separate methods make the intent explicit.

---

## 13. LOW: `MathDelimiter::Parentheses` / `Brackets` Removed

**✅  RESOLVED** — `token.rs:62-67`

Removed unused `Parentheses` and `Brackets` variants from `MathDelimiter`. Only `Dollar` and `DoubleDollar` remain.

**Why:** Dead code. These variants were never produced by any scan method and existed only in anticipation of `\( \)` / `\[ \]` support. They can be re-added when those scan methods are implemented. Also derived `Copy` on `MathDelimiter` to simplify usage.

---

## 14. BUG: `scan_math_shift()` Over-Consumed on Single `$`

**✅  RESOLVED** — `crates/ltx_lexer/src/lexer.rs:231-253`

```rust
// Old (bug): always bumped twice, then a third time for single $
let is_double = self.peek_nth(1) == Some('$');
let _ = (self.bump(), self.bump());
let delimiter = if is_double { DoubleDollar } else { let _ = self.bump(); Dollar };

// New (correct): bump once, conditionally bump second
let _ = self.bump();
let delimiter = if self.peek() == Some('$') {
    let _ = self.bump();
    DoubleDollar
} else {
    Dollar
};
```

**Why:** For a single `$`, the old code called `bump()` **3 times**, consuming the `$` plus two extra characters. This silently skipped input and corrupted the cursor. The new logic bumps exactly once for single `$` and twice for `$$`.

---

## 15. Testing Gaps

**🟡 PARTIALLY ADDRESSED**

| Gap | Detail | Status |
|---|---|---|
| **No lexer tests** | Zero tests for any `LtxLexer` method | 🔴 Unchanged |
| **No error handler tests** | Zero tests for `LexerErrorHandler` | 🔴 Unchanged |
| **Missing catcode variant in test** | `AlignmentTab`, `InlineMathEnd`, `MathShift` were excluded from `from_u8` round-trip | ✅ Added |
| **Redundant test** | `test_get_catcodestate` (line 30) is a subset of `test_default_catcode_state` (line 7) | 🔴 Unchanged |
| **No integration tests** | No end-to-end lexer tests with real TeX input | 🔴 Unchanged |
| **No edge case tests** | Empty source, unicode > U+00FF, `\r\n`, `\r`-only, etc. | 🔴 Unchanged |
| **No snapshot tests** | Mentioned in `todo.md` — none exist | 🔴 Unchanged |
| **Regression tests** | No test for the `scan_math_shift` over-consumption bug | 🔴 Skipped per request |

---

## 16. Summary of Recommendations

| Priority | Area | Recommendation | Status |
|---|---|---|---|
| Critical | lexer.rs | Implement remaining 8 scan methods + `Iterator` + mode switching | 🟡 In progress (10/18 done) |
| High | various | **All high-priority findings from previous audit resolved** | ✅ Done |
| Medium | lexer.rs | Cache characters to avoid `Chars` iterator creation per call | 🔴 Unchanged |
| Medium | errors_core.rs / lexer.rs | Inconsistent `source_map` ownership | 🔴 Unchanged |
| Gap | tests/ | Add lexer, error handler, integration, edge case, regression tests | 🔴 Unchanged |

### Findings closed this session (2026-07-01):

| # | Finding | Fix |
|---|---|---|
| 2 | `#[repr(u8)]` / implicit discriminants | Restored explicit discriminants on `LtxCatCode` |
| 3 | Per-token heap allocations | `&'source str` text + fieldless `Text`/`Comment` |
| 4 | Dead code `from_source_map()` | Deleted |
| 5 | Catcode-based EOL in `scan_comment()` | Changed to `ch == '\n'` |
| 6a | Triple `$` assignment | Single `MathShift` entry |
| 6b | `\r` / `\0` not mapped | Added to `default_tex()` |
| 6c | `LineBreak` unused | Removed |
| 7 | `set()` silent ≥ U+00FF | Documented |
| 8 | `take_errors()` silent drop | Documented |
| 11 | Misleading doc comments | Fixed all 3 stale comments |
| 12 | `len()` conflated counts | Renamed to `total_count()`, added `error_count()` |
| 13 | Unused `MathDelimiter` variants | Removed `Parentheses`/`Brackets` |
| 14 | `scan_math_shift()` over-consumption | Fixed single-`$` bump count |
