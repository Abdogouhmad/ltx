# Roadmap: Implementing the Stateful `LtxLexer` Struct (Manual Character Scanner)

This document outlines the step-by-step roadmap for implementing a manual, stateful character scanner for `LtxLexer` in the `ltx_lexer` crate. 

Instead of using a static lexer generator like `logos`, we will write a character-by-character scanner that relies dynamically on `LtxCatCodeState` and `LtxMode` to tokenize LaTeX source code.

---

## 1. Struct Definition & Field Rationale

Here is the structure of the manual `LtxLexer` struct:

```rust
pub struct LtxLexer<'source> {
    /// 1. The input LaTeX source code slice
    source: &'source str,
    
    /// 2. Current byte offset of the scanner
    cursor: usize,
    
    /// 3. Identifier of the file being processed
    file_id: LtxFileId,
    
    /// 4. Stateful LaTeX category codes
    pub catcodes: LtxCatCodeState,
    
    /// 5. Current lexing mode (Normal, Math, Verbatim)
    pub mode: LtxMode,
    
    /// 6. Collector for errors
    pub error_core: LexerErrorCore,
}
```

### Why we chose these fields:

1. **`source: &'source str`**
   * **Why**: The raw input source string. Since we are not using a generator, we need direct access to the source code to iterate over characters and slice tokens out of it.
2. **`cursor: usize`**
   * **Why**: Keeps track of our current byte position inside the `source` string. It starts at `0` and increases as we consume characters.
3. **`file_id: LtxFileId`**
   * **Why**: Maps the location of every token (span) back to its original file in the source map for accurate error reporting.
4. **`catcodes: LtxCatCodeState`**
   * **Why**: LaTeX allows changing character behaviors on the fly (e.g. making `@` act as a letter). Keeping this table inside the lexer allows us to query character category codes dynamically.
5. **`mode: LtxMode`**
   * **Why**: LaTeX syntax rules change drastically depending on whether we are scanning normal text, math blocks (`$...$`), or verbatim environments. Changing this enum dictates which tokenization logic we run.
6. **`error_core: LexerErrorCore`**
   * **Why**: Accumulates lexical errors without halting execution, allowing the compiler to perform error recovery.

---

## 2. Functions to Implement inside `impl LtxLexer`

Below is the list of all functions that need to be implemented in `LtxLexer`.

### Creation Method
* **`pub fn new(source: &'source str, file_id: LtxFileId, source_map: Arc<LtxSourceMap>) -> Self`**
  * Initializes the `LtxLexer` with the source text, cursor at `0`, standard TeX catcodes (`default_tex()`), normal mode, and a fresh `LexerErrorCore`.

### Character Iteration & Peeking Helpers
* **`fn peek(&self) -> Option<char>`**
  * Returns the character at the current `cursor` position without advancing. Returns `None` if EOF is reached.
* **`fn peek_nth(&self, n: usize) -> Option<char>`**
  * Looks ahead `n` characters from the current cursor without advancing.
* **`fn bump(&mut self) -> Option<char>`**
  * Returns the current character and advances the `cursor` by its UTF-8 length (e.g. `ch.len_utf8()`).
* **`fn is_eof(&self) -> bool`**
  * Returns `true` if the cursor is at or beyond the length of `source`.

### Stateful Mode Dispatchers
* **`pub fn next_token(&mut self) -> Option<LtxToken>`**
  * The main entry point. Matches on `self.mode` and routes execution to:
    * `next_normal_token()`
    * `next_math_token()`
    * `next_verbatim_token()`
* **`fn next_normal_token(&mut self) -> Option<LtxToken>`**
  * Handles normal document syntax. Peeks at the next character, retrieves its `LtxCatCode` from `self.catcodes`, and branches accordingly (e.g., starts scanning a command on `Escape`, a comment on `Comment`, or text on `Letter`/`Other`).
* **`fn next_math_token(&mut self) -> Option<LtxToken>`**
  * Handles math mode tokenization. Matches math shift symbols, inline equations, subscript/superscript characters (`_` and `^`), and math-only commands.
* **`fn next_verbatim_token(&mut self) -> Option<LtxToken>`**
  * Handles verbatim text blocks. Scans everything as raw text until it encounters the exact sequence `\end{verbatim}`.

### Sub-Tokenizers (Normal Mode)
* **`fn scan_command(&mut self, start: usize) -> LtxToken`**
  * Triggered by `LtxCatCode::Escape` (`\`). Consumes letters (`LtxCatCode::Letter`) to form command names (like `\section`). Handles special cases like single non-letter escape sequences (e.g., `\#`, `\\`).
* **`fn scan_text(&mut self, start: usize) -> LtxToken`**
  * Consumes characters (letters, numbers, basic punctuation) until hitting a special control character (like `\`, `{`, `}`, `%`, `$`).
* **`fn scan_space(&mut self, start: usize) -> LtxToken`**
  * Consumes consecutive whitespace characters. In LaTeX, multiple consecutive space or tab characters are squashed into a single `Space` token.
* **`fn scan_comment(&mut self, start: usize) -> LtxToken`**
  * Triggered by `LtxCatCode::Comment` (`%`). Consumes characters until the end of the line (`\n` or `\r\n`).

### Span Helper
* **`fn span(&self, start: usize, end: usize) -> LtxSpan`**
  * Helper to build a `LtxSpan` using the given `start` and `end` byte offsets and the lexer's `file_id`.

---

## 3. Step-by-Step Implementation Guide

1. **Enable crate dependencies**: Add `logos` if desired, but since we are implementing manually, we only need standard library functions.
2. **Implement Iteration Helpers**: Write `peek`, `bump`, and `is_eof` in `crates/ltx_lexer/src/lexer.rs` first. They are the foundation of all scanning.
3. **Implement Sub-Tokenizers**: Write `scan_command`, `scan_text`, and `scan_space`. Test them with basic inputs (like `Hello \world`).
4. **Implement Mode Dispatching**: Write `next_token` to select between `Normal`, `Math`, and `Verbatim` parsing modes.
5. **Implement Iterator Trait**: Connect `LtxLexer` to the `Iterator` trait:
   ```rust
   impl<'source> Iterator for LtxLexer<'source> {
       type Item = LtxToken;
       fn next(&mut self) -> Option<Self::Item> {
           self.next_token()
         }
     }
   ```

---

## 4. Main Example: The Finished Product

Here is how the manual, dynamic lexer will be used in your code:

```rust
use ltx_diagnostics::{LtxSourceMap, LtxDiagnosticSink};
use ltx_lexer::{LtxLexer, LtxMode, LtxCatCode};
use miette::Report;
use std::sync::Arc;

fn main() {
    // 1. Setup the SourceMap and inline LaTeX document
    let source = "\\documentclass{article}\nHello @world % @ is set as a letter here!";
    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline("main.tex", source);
    let source_map = Arc::new(source_map);

    // 2. Initialize the manual Lexer
    let mut lexer = LtxLexer::new(source, file_id, source_map);

    // 3. Demonstrate Dynamic CatCode modification!
    // Set '@' to be treated as a Letter (LtxCatCode::Letter)
    lexer.catcodes.set('@', LtxCatCode::Letter);

    // 4. Tokenize the input stream
    println!("--- Lexed Tokens ---");
    while let Some(token) = lexer.next() {
        println!(
            "Kind: {:?}, Span: [{}..{}], Text: {:?}",
            token.kind, token.span.start, token.span.end, token.text
        );
    }

    // 5. Gather and print pretty miette diagnostics if errors were encountered
    if lexer.error_core.has_errors() {
        let mut sink = LtxDiagnosticSink::new();
        for diag in lexer.error_core.take_diagnostics() {
            sink.push(diag);
        }

        println!("\n🔍 Found {} issue(s):\n", sink.len());
        for diag in sink.drain_sorted() {
            let report = Report::new(diag);
            println!("{report:?}\n");
        }
    } else {
        println!("\nLexing completed with 0 errors!");
    }
}
```
