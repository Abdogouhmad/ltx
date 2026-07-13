# `crates/ltx_lexer` — Code Improvements

**Version:** 0.3.2  
**Files:** 7 source modules, ~850 lines

---

## 1. ~~`size_hint()` Lower Bound Is Wrong~~ ✅ Fixed

Lower bound was `remaining` (incorrect), now `0` — `lexer.rs:131`.

---

## 2. ~~`Escape` Token Variant Inconsistency~~ ✅ Fixed

No `Escape` variant exists in `LtxTokenKind`. All control symbols route through `scan_command()`.

---

## 3. ~~Trailing-Space Eating After Control Words~~ ✅ Fixed

`normal_cmd()` in `lexer_utils.rs:238-244` consumes trailing `WhiteSpace`/`EndOfLine` after control words.

---

## 4. ~~`\end` Mismatch Doesn't Pop the Stack~~ ✅ Fixed

`scan_end()` in `lexer_utils.rs:199` calls `pop_env()` before emitting the mismatch diagnostic.

---

## 5. ~~Dead Token Variants to Clean~~ ✅ Fixed

`Verbatim`, `VerbatimStart`, `Parameter`, `Active`, `InlineMathStart`, `InlineMathEnd` all removed.

---

## 6. ~~Dead Error Factory to Clean~~ ✅ Fixed

`unterminated_verbatim()` removed from `errors_core.rs`. The `LtxError::UnterminatedVerbatim` variant remains in `ltx_diagnostics` as part of the error spec (reserved for future use).

---

## 7. ~~Unicode Letter Support Is Still ASCII-Only~~ ✅ Fixed

`catcode.rs:141` uses `c.is_alphabetic()` (full Unicode), not `c.is_ascii_alphabetic()`.

---

## 8. `AlignmentTab` / `Superscript` / `Subscript` Not Handled in Math Mode

**File:** `lexer.rs:85-105`

`next_math_token()` routes `^`, `_`, and `&` to `scan_text()` via the `_ =>` catch-all. In LaTeX math mode these are operators, not text.

**Fix:** Add explicit arms in the math dispatch:
```rust
LtxCatCode::Superscript | LtxCatCode::Subscript => self.scan_text(),
LtxCatCode::AlignmentTab => self.scan_text(),
```

At minimum these should get their own token kinds (`Superscript`, `Subscript`, `AlignmentTab`) instead of being silently absorbed into `Text`. This matters for syntax highlighting and math-mode-aware parsing.

**Effort:** ~10 lines (new token variants + match arms)

---

## Summary

| # | Issue | Status |
|---|---|---|
| 1 | `size_hint()` wrong lower bound | ✅ Fixed |
| 2 | `Escape` vs `Command` inconsistency | ✅ Fixed |
| 3 | Trailing-space eating missing | ✅ Fixed |
| 4 | `\end` mismatch doesn't pop stack | ✅ Fixed |
| 5 | Dead `Verbatim`/`VerbatimStart` variants | ✅ Fixed |
| 6 | Dead `unterminated_verbatim` error factory | ✅ Fixed |
| 7 | Unicode `is_letter` still ASCII | ✅ Fixed |
| 8 | Math mode `^`/`_`/`&` not categorized | **Open** |
