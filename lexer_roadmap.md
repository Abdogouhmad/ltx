# Roadmap: Implementing the Stateful `LtxLexer` Struct

> **How to use this document**: Every function below lists ordered TODOs. Work top to bottom — each function depends on the ones above it. Do not skip ahead.

---

## Phase 1 — The Struct

**`pub struct LtxLexer<'source>`**

* Declare a field `source: &'source str` — this holds the raw LaTeX text we will scan
* Declare a field `cursor: usize` — this is our current position in bytes inside `source`
* Declare a field `file_id: LtxFileId` — needed to attach file information to every span we create
* Declare a field `pub catcodes: LtxCatCodeState` — the live category-code table; `pub` because the caller may mutate it (e.g. `lexer.catcodes.set('@', Letter)`)
* Declare a field `pub mode: LtxMode` — the current scanning mode; `pub` because the parser will switch it to `Verbatim` after seeing `\begin{verbatim}`
* Declare a field `pub error_core: LexerErrorCore` — the error collector; `pub` so the caller can read errors after lexing finishes

---

## Phase 2 — Construction

**`pub fn new(source, file_id, source_map) -> Self`**

* Set `source` to the incoming `source` parameter
* Set `cursor` to `0` — we always start at the beginning
* Set `file_id` to the incoming `file_id` parameter
* Set `catcodes` to `LtxCatCodeState::default_tex()` — this loads the standard TeX category code table
* Set `mode` to `LtxMode::Normal` — LaTeX documents always start in normal mode
* Set `error_core` to `LexerErrorCore::new(source_map)` — pass the source map so errors can reference it

---

## Phase 3 — Iteration Helpers

Implement these four functions first. Everything else calls them.

---

**`fn is_eof(&self) -> bool`**

* Check if `self.cursor` is greater than or equal to `self.source.len()`
* Return that result — `true` means we have consumed the entire input

---

**`fn peek(&self) -> Option<char>`**

* Slice `self.source` from `self.cursor` to the end — this gives the unscanned remainder
* Call `.chars().next()` on that slice to get the first character without moving anything
* Return it — `None` if we are at EOF

---

**`fn peek_nth(&self, n: usize) -> Option<char>`**

* Slice `self.source` from `self.cursor` to the end, same as `peek`
* Call `.chars().nth(n)` on that slice — `n = 0` is the same character as `peek`, `n = 1` is one ahead, and so on
* Return it — `None` if there are fewer than `n + 1` characters remaining
* Note: use `.chars()` and not byte indexing — UTF-8 characters can be more than one byte wide

---

**`fn bump(&mut self) -> Option<char>`**

* Call `self.peek()` to get the current character — if `None`, return `None` immediately
* Advance `self.cursor` by `ch.len_utf8()` — this is the correct byte length for that character
* Return the character we just consumed

---

## Phase 4 — Span Helper

**`fn span(&self, start: usize, end: usize) -> LtxSpan`**

* Construct an `LtxSpan` using `start`, `end`, and `self.file_id`
* Return it
* Note: `start` is always captured at the top of a scan function before any `bump()` calls; `end` is always `self.cursor` after all `bump()` calls

---

## Phase 5 — Normal Mode Sub-Tokenizers

Implement these before wiring up the dispatcher. Test each one in isolation.

---

**`fn scan_space(&mut self, start: usize) -> LtxToken`**

* Loop: while `self.peek()` is `Some(' ')` or `Some('\t')`, call `self.bump()`
* Stop looping when the next character is neither a space nor a tab
* Build a token with kind `LtxTokenKind::Space`
* Set its text to a single `" "` string — TeX collapses multiple spaces into one
* Set its span by calling `self.span(start, self.cursor)`
* Return the token

---

**`fn scan_eol(&mut self, start: usize) -> LtxToken`**

* Check if the character we already consumed (look at `self.source[start..self.cursor]`) ends with `'\r'`
* If yes, check if `self.peek()` is `Some('\n')` — if so, call `self.bump()` to consume the `\n` too (this handles Windows-style `\r\n` line endings)
* Build a token with kind `LtxTokenKind::Eol`
* Set its text to `"\n"` — normalize all line endings to Unix style
* Set its span by calling `self.span(start, self.cursor)`
* Return the token

---

**`fn scan_comment(&mut self, start: usize) -> LtxToken`**

* Note: the `%` character has already been consumed before this function is called
* Loop: while `self.peek()` is not `Some('\n')`, not `Some('\r')`, and not `None`, call `self.bump()`
* Stop when you hit a newline or EOF — do NOT consume the newline itself; that belongs to the next token
* Build a token with kind `LtxTokenKind::Comment`
* Set its text to `self.source[start..self.cursor]` — this includes the `%` character
* Set its span by calling `self.span(start, self.cursor)`
* Return the token

---

**`fn scan_text(&mut self, start: usize) -> LtxToken`**

* Loop: call `self.peek()` and get the catcode of that character via `self.catcodes.get(ch)`
* If the catcode is any of `Escape`, `BeginGroup`, `EndGroup`, `MathShift`, `Space`, `EOL`, or `Comment` — break out of the loop; these characters end a text run
* Otherwise call `self.bump()` and continue
* Build a token with kind `LtxTokenKind::Text`
* Set its text to `self.source[start..self.cursor]`
* Set its span by calling `self.span(start, self.cursor)`
* Return the token

---

**`fn scan_command(&mut self, start: usize) -> LtxToken`**

* Note: the `\` character has already been consumed before this function is called
* Call `self.peek()` and check the catcode of the next character
* **Case 1 — next character is `LtxCatCode::Letter`**:
  * Loop: while `self.peek()` has catcode `LtxCatCode::Letter`, call `self.bump()`
  * After the loop, consume trailing spaces: while `self.peek()` is `Some(' ')` or `Some('\t')`, call `self.bump()` — TeX rule: word commands absorb following whitespace
* **Case 2 — next character exists but is NOT a letter**:
  * Call `self.bump()` exactly once — symbol commands like `\\` or `\#` are always one character
* **Case 3 — `self.peek()` is `None` (backslash at EOF)**:
  * Push a `LexerError::StrayEscape` onto `self.error_core` with the current span
  * Do not consume anything further
* Build a token with kind `LtxTokenKind::Command`
* Set its text to `self.source[start..self.cursor]` — this includes the `\`
* Set its span by calling `self.span(start, self.cursor)`
* Return the token

---

**`fn scan_math_shift(&mut self, start: usize) -> LtxToken`**

* Note: the first `$` has already been consumed before this function is called
* Check if `self.peek()` is `Some('$')` — if yes, call `self.bump()` to consume the second `$`
* Choose the token kind:
  * If we consumed two `$` signs → kind is `LtxTokenKind::DisplayMathShift`
  * If we consumed one `$` sign → kind is `LtxTokenKind::InlineMathShift`
* Set `self.mode = LtxMode::Math` — from this point on `next_token` will call `next_math_token`
* Build the token with the chosen kind
* Set its text to `self.source[start..self.cursor]`
* Set its span by calling `self.span(start, self.cursor)`
* Return the token

---

## Phase 6 — Normal Mode Dispatcher

**`fn next_normal_token(&mut self) -> Option<LtxToken>`**

* Save the current position: `let start = self.cursor`
* Call `self.peek()` — if `None`, return `None` (EOF)
* Get the catcode of that character via `self.catcodes.get(ch)`
* Match on the catcode:
  * `Escape` → call `self.bump()` to consume `\`, then call `self.scan_command(start)` and return it
  * `BeginGroup` → call `self.bump()`, build a single-character token with kind `LBrace`, return it
  * `EndGroup` → call `self.bump()`, build a single-character token with kind `RBrace`, return it
  * `MathShift` → call `self.bump()` to consume `$`, then call `self.scan_math_shift(start)` and return it
  * `Space` → call `self.bump()`, then call `self.scan_space(start)` and return it
  * `EOL` → call `self.bump()`, then call `self.scan_eol(start)` and return it
  * `Comment` → call `self.bump()` to consume `%`, then call `self.scan_comment(start)` and return it
  * Any other catcode → call `self.scan_text(start)` and return it (do NOT bump first; `scan_text` will consume the character)

---

## Phase 7 — Math Mode

**`fn next_math_token(&mut self) -> Option<LtxToken>`**

* First, check `self.is_eof()` — if true, push a `LexerError::UnclosedMath` error, set `self.mode = LtxMode::Normal`, and return `None`
* Save the current position: `let start = self.cursor`
* Call `self.peek()` to look at the current character
* Match on the raw character (not catcode — math mode has fixed syntax):
  * `'$'` → call `self.bump()`; then check if `self.peek()` is another `'$'` and if so bump again; set `self.mode = LtxMode::Normal`; choose kind `DisplayMathShift` or `InlineMathShift`; build and return the token
  * `'^'` → call `self.bump()`; build a token with kind `Superscript`; return it
  * `'_'` → call `self.bump()`; build a token with kind `Subscript`; return it
  * `'{'` → call `self.bump()`; build a token with kind `LBrace`; return it
  * `'}'` → call `self.bump()`; build a token with kind `RBrace`; return it
  * `'\\'` → call `self.bump()`; call `self.scan_command(start)`; return it — commands work identically in math mode
  * `'%'` → call `self.bump()`; call `self.scan_comment(start)`; return it — comments are valid in math mode
  * `' '` or `'\t'` → call `self.bump()`; call `self.scan_space(start)`; return it
  * `'\n'` or `'\r'` → call `self.bump()`; call `self.scan_eol(start)`; return it
  * Anything else → call `self.bump()`; build a token with kind `MathAtom`; set text to `self.source[start..self.cursor]`; return it

---

## Phase 8 — Verbatim Mode

**`fn next_verbatim_token(&mut self) -> Option<LtxToken>`**

* Define the terminator string as a constant: `const TERMINATOR: &str = r"\end{verbatim}"`
* Save the current position: `let start = self.cursor`
* Loop:
  * Check `self.is_eof()` — if true, push a `LexerError::UnclosedVerbatim` error and break out of the loop
  * Check if `self.source[self.cursor..]` starts with `TERMINATOR` using `.starts_with()` — do NOT use `peek_nth` for this; string `.starts_with()` is cleaner and faster for a multi-character sequence
  * If the terminator is found:
    * Save `self.cursor` as `content_end` — this is where the verbatim content stops
    * Advance `self.cursor` by `TERMINATOR.len()` to skip past `\end{verbatim}`
    * Set `self.mode = LtxMode::Normal`
    * If `content_end > start` (there was actual content), build a token with kind `VerbatimContent`, text `self.source[start..content_end]`, span `self.span(start, content_end)`, and return it
    * If `content_end == start` (empty verbatim body), return `None`
  * If the terminator is NOT found, call `self.bump()` and continue the loop
* After the loop (only reached on EOF error): if `self.cursor > start`, build and return a `VerbatimContent` token for whatever was collected; otherwise return `None`

---

## Phase 9 — Main Dispatcher

**`pub fn next_token(&mut self) -> Option<LtxToken>`**

* Check `self.is_eof()` — if true, return `None` immediately
* Match on `self.mode`:
  * `LtxMode::Normal` → call and return `self.next_normal_token()`
  * `LtxMode::Math` → call and return `self.next_math_token()`
  * `LtxMode::Verbatim` → call and return `self.next_verbatim_token()`

---

## Phase 10 — Iterator Trait

**`impl<'source> Iterator for LtxLexer<'source>`**

* Set `type Item = LtxToken`
* Implement `fn next(&mut self) -> Option<Self::Item>`:
  * Call and return `self.next_token()`
* That's it — this one line unlocks `for token in lexer`, `.collect()`, `.filter()`, `.map()`, and every other iterator adapter for free

---

## Mode Transition Cheat Sheet

| You are in | You see | You call | Mode becomes |
|---|---|---|---|
| `Normal` | `$` | `scan_math_shift` | `Math` |
| `Normal` | `$$` | `scan_math_shift` | `Math` |
| `Normal` | `\begin{verbatim}` | *(parser sets mode)* | `Verbatim` |
| `Math` | closing `$` or `$$` | inside `next_math_token` | `Normal` |
| `Verbatim` | `\end{verbatim}` | inside `next_verbatim_token` | `Normal` |

> **Important**: The lexer itself never detects `\begin{verbatim}`. It tokenizes `\begin` as a normal `Command` token and `{verbatim}` as normal tokens. It is the **parser** that recognizes this sequence and sets `lexer.mode = LtxMode::Verbatim` before calling `next_token()` again.

---

## Implementation Order (Do Not Deviate)

1. The struct fields
2. `new`
3. `is_eof` → `peek` → `peek_nth` → `bump`
4. `span`
5. `scan_space` → `scan_eol` → `scan_comment` → `scan_text` → `scan_command` → `scan_math_shift`
6. `next_normal_token`
7. `next_math_token`
8. `next_verbatim_token`
9. `next_token`
10. `Iterator` impl
