//! Test for the source file of `ltxDiagnostics`

use std::path::PathBuf;

use ltx_diagnostics::LtxSourceMap;
use pretty_assertions::assert_eq;
#[test]
fn test_add_inline() {
    let source = "Hello World".to_string();

    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline("test.tex", &source);

    assert_eq!(file_id.0, 0);
}

#[test]
fn test_add_file() -> Result<(), Box<dyn std::error::Error>> {
    let test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/test.tex");

    let mut source_map = LtxSourceMap::new();

    let file_id = source_map.add_file(test_path)?;

    assert_eq!(file_id.0, 0);
    Ok(())
}
