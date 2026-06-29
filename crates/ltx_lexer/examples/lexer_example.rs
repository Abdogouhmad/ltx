//! Step 1: Basic lexer functionality – testing EOF.

use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::LtxLexer;

fn main() {
    // 1. Create a source string.
    let source = "Hello, world!";

    // 2. Create a source map and file ID.
    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline("example.tex", source);

    // 3. Instantiate the lexer.
    let mut lexer = LtxLexer::new(source, file_id, source_map);

    while let Some(ch) = lexer.bump() {
        println!("consumed '{}'", ch);
    }
    println!("EOF: {}", lexer.is_eof());
}
