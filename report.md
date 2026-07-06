# `crates/ltx_lexer` — Code Improvements

**Version:** 0.2.2  
**Files:** 7 source modules, ~1,010 lines

---

## 1. `size_hint()` Lower Bound Is Wrong

**File:** `lexer.rs:136`  
**Current code:**
```rust
fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = self.source.len() - self.cursor;
    (remaining, Some(remaining))
}
```

**Problem:** The lower bound `remaining` means "I guarantee at least this many tokens". That's false — `"abcdef"` produces 1 `Text` token, not 6. Consumers like `.collect::<Vec<_>>()` will pre-allocate for `remaining` slots, which is wasteful but correct. However, iterators that trust the lower bound for exact sizing will panic or behave incorrectly.

**Fix:** Lower bound must be 0:
```rust
(0, Some(remaining))
```

---

## 2. `Escape` Token Variant Becomes Dead Code

**File:** `token.rs:52-53`

`Escape` was wired to handle `\\` (double backslash → line break), but LaTeX uses `\\` as a control symbol just like `\$` or `\#`. The current dispatcher routes `\\` to `scan_escape()` (producing `Escape`) and everything else to `scan_command()` (producing `Command`). This means `\\` produces a **different token kind** than every other control symbol (`\$`, `\#`, `\%`, etc.), making it inconsistent.

**Options:**
- **(Recommended)** Remove the `Escape` variant, delete `scan_escape()`, route all `Escape` catcode → `scan_command()`. `\\` falls through to the control symbol path in `scan_command()` and produces `Command("\\")` like every other control symbol.
- Keep `Escape` if you want to distinguish line breaks (`\\`) from commands semantically. Document the distinction clearly.

---

## 3. Trailing-Space Eating After Control Words

**File:** `lexer_utils.rs:417` (`scan_command()`)

LaTeX convention: control words (`\LaTeX`, `\section`) swallow subsequent whitespace. Currently `\LaTeX ` produces `Command("LaTeX")` then `WhiteSpace`. This is technically valid but un-idiomatic — parsers expecting LaTeX semantics may be surprised.

**Fix:** After a control word (the letter-based path in `scan_command()`), peek at the next character. If it's `WhiteSpace` or `EndOfLine`, consume it.

---

## 4. `\end` Mismatch Doesn't Pop the Stack

**File:** `lexer_utils.rs:199-206`

When `\end{env}` doesn't match the expected environment, an error is returned but `pop_env()` is skipped. The stale environment stays on the stack, causing cascading errors on subsequent `\end` calls.

**Fix:** Pop before returning the error:
```rust
if env != expected {
    let _ = self.pop_env();
    // ... error
}
```

---

## 5. Dead Token Variants to Clean

**File:** `token.rs:34-36`

`Verbatim(&str)` and `VerbatimStart` are dead since verbatim mode is removed from the codebase. Delete them to keep the enum lean.

---

## 6. Dead Error Factory to Clean

**File:** `errors_core.rs:196-201`

`unterminated_verbatim()` (E009) is dead code since verbatim mode is removed. Delete it and the corresponding `LexerError::UnterminatedVerbatim` variant in `ltx_diagnostics` (if applicable).

---

## 7. Unicode Letter Support Is Still ASCII-Only

**File:** `catcode.rs:140-143`

```rust
pub fn is_letter(&self, c: char) -> bool {
    let cat = self.get(c);
    cat == LtxCatCode::Letter || c.is_ascii_alphabetic()
}
```

`is_ascii_alphabetic()` rejects `é`, `ü`, `ñ`, etc. Commands like `\café` break at `é`. The `get()` call above it also returns `Other` for any char > U+00FF.

**Fix:** Use `c.is_alphabetic()` instead of `c.is_ascii_alphabetic()` — this covers all Unicode letters.

---

## 8. `AlignmentTab` / `Superscript` / `Subscript` Not Handled in Math Mode

**File:** `lexer.rs:84-114`

`next_math_token()` routes `^`, `_`, and `&` (among others) to `scan_text()`. In LaTeX math mode, `^` and `_` are superscript/subscript operators and `&` is used for alignment in `{align}` environments.

**Fix:** Add dedicated catcode arms in `next_math_token()` or handle them generically. At minimum, don't silently absorb them into `Text` tokens.

---

## Summary

| Priority | Issue | Effort |
|---|---|---|
| **High** | `size_hint()` wrong lower bound | 1 line |
| **High** | `Escape` vs `Command` inconsistency for `\\` | 3 lines or delete function |
| **Medium** | Trailing-space eating missing | ~5 lines |
| **Medium** | `\end` mismatch doesn't pop stack | 1 line |
| **Low** | Dead `Verbatim`/`VerbatimStart` variants | Delete 2 enum entries |
| **Low** | Dead `unterminated_verbatim` error factory | Delete 1 method |
| **Low** | Unicode `is_letter` still ASCII | 1 char change |
| **Low** | Math mode `^`/`_`/`&` not categorized | TBD |
